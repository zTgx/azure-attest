use crate::{config::Config, utils::base64};
use azure_core::base64;
use codec::{Decode, Encode};
use http_req::{
	request::{Method, RequestBuilder},
	tls,
	uri::Uri,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
	net::TcpStream,
	string::{String, ToString},
	vec::Vec,
};

type EnclaveResult<T> = Result<T, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MAAPolicy {
	#[serde(rename = "is-debuggable")]
	pub is_debuggable: bool,

	#[serde(rename = "product-id")]
	pub product_id: u32,

	#[serde(rename = "sgx-mrenclave")]
	pub sgx_mrenclave: String,

	#[serde(rename = "sgx-mrsigner")]
	pub sgx_mrsigner: String,

	pub svn: u32,
	pub tee: String,
}

#[derive(Debug, Serialize, Deserialize, Decode, Encode)]
pub struct MAAResponse {
	pub token: String,
}

impl Default for MAAPolicy {
	fn default() -> Self {
		MAAPolicy {
			is_debuggable: true,
			product_id: 0_u32,
			sgx_mrenclave: Default::default(),
			sgx_mrsigner: Default::default(),
			svn: 0u32,
			tee: "sgx".to_string(),
		}
	}
}

//  Trait to do Microsoft Azure Attestation
pub trait MAAHandler {
	//  Verify DCAP quote from MAA
	fn azure_attest(&self, quote: &[u8]) -> EnclaveResult<Vec<u8>>;
}

pub struct MAAService;
impl MAAService {
	pub fn parse_maa_policy(writer: &[u8]) -> EnclaveResult<Vec<u8>> {
		let res: MAAResponse = serde_json::from_slice(&writer).unwrap();
		println!("res: {:?}", res);

		let decompose_token: Vec<&str> = res.token.split(".").collect();
		if decompose_token.len() != 3 {
			println!("JSON Web Tokens must have 3 components delimited by '.' characters.");
		}

		let policy = base64::decode(decompose_token[1]).unwrap();

		Ok(policy)
	}
}

impl MAAHandler for MAAService {
	fn azure_attest(&self, quote: &[u8]) -> EnclaveResult<Vec<u8>> {
		println!("    [Enclave] Entering azure_attest.");

		let quote = base64(quote.to_vec());
		let req_body = json!({
			"quote": quote
		})
		.to_string();

		let url = Config::default().endpoint + "/attest/SgxEnclave?api-version=2020-10-01";
		let addr = Uri::try_from(&url[..]).unwrap();
		let sock = TcpStream::connect((addr.host().unwrap(), addr.corr_port())).unwrap();
		let mut writer = Vec::new();

		let mut stream = tls::Config::default().connect(addr.host().unwrap_or(""), sock).unwrap();

		let response = RequestBuilder::new(&addr)
			.method(Method::POST)
			.body(req_body.as_bytes())
			.header("Content-Length", &req_body.len())
			.header("Connection", "Close")
			.header("Content-Type", "application/json")
			.header("Authorization", &format!("Bearer {}", Config::default().token))
			.send(&mut stream, &mut writer)
			.unwrap();
		let status_code = response.status_code();
		let reason = response.reason();

		println!(">>> response status code: {}", status_code);
		println!(">>> response reason: {}", reason);

		let resp_string = String::from_utf8_lossy(&writer);
		println!("resp_string: {}", resp_string);

		Self::parse_maa_policy(&writer)
	}
}

#[cfg(test)]
pub mod tests {
	use azure_core::base64;

	use super::*;

	#[test]
	pub fn azure_attest_works() {
		pub const DCAP_QUOTE: &[u8] = include_bytes!("./quote_sample");
		let quote = hex::decode(DCAP_QUOTE).unwrap();

		let s = MAAService;
		let ret = s.azure_attest(&quote);
		println!("ret: {:?}", ret);
	}

	#[test]
	fn xxx_works() {
		pub const MAA_RES_SAMPLE: &[u8] = include_bytes!("./maa_response_sample");
		let ret = MAAService::parse_maa_policy(&MAA_RES_SAMPLE);
		assert!(ret.is_ok());

		let v = base64(ret.clone().unwrap());
		println!("Policy: {}", v);
		let x = base64::decode(v).unwrap();
		let v: MAAPolicy = serde_json::from_slice(&x).unwrap();
		println!("Policy: {:?}", v);
	}
}
