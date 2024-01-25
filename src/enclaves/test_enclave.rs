use std::str::FromStr;

use azure_svc_attestation::models::AttestationResponse;
use reqwest::blocking::Client;
use serde_json::Value;
use url::Url;

use super::{
	enclave_info::{EnclaveInfo, ShowTime},
	model::{AttestSgxEnclaveRequest, AttestationRequest, DataType, RuntimeData},
};
use crate::{
	config::Config,
	service::client::ClientBuilder,
	utils::{base64, decode_attest_result, read_string_from_file},
	MAA,
};

pub struct TestEnclave;

impl MAA for TestEnclave {
	fn azure_attest(&self) {
		let config = Config::default();
		let client = ClientBuilder::new(config.token, Url::from_str(&config.endpoint).unwrap())
			.build()
			.unwrap()
			.attestation_client();

		let quote = read_string_from_file("quotes/sgx_enclave_quote.txt");
		let quote = hex::decode(quote).unwrap();
		let quote = base64(quote);

		let mut request = AttestSgxEnclaveRequest::new();
		request.quote = Some(quote);

		let request_builder = client.attest_sgx_enclave(request);
		match request_builder.send() {
			Ok(res) => {
				println!("Got AttestationResponse from MAA service: {:#?}", res);

				// let value: Value = res.json().unwrap();
				// let attest_response: AttestationResponse = serde_json::from_value(value).unwrap();

				// // println!("Got AttestationResponse from MAA service: {:#?}", attest_response);

				// if let Some(token_body) = attest_response.token {
				// 	// println!("Got token body from MAA service: {:#?}", token_body);

				// 	let attest_result = decode_attest_result(token_body);

				// 	// println!(
				// 	//     "Got AttestationResult from MAA service: {:#?}",
				// 	//     attest_result
				// 	// );

				// 	let enclave_info =
				// 		EnclaveInfo::create_from_file("quotes/enclave.info.securityversion.json");
				// 	enclave_info.show_attest(&attest_result, true);
				// }
			},
			Err(_) => {},
		}
	}
}
