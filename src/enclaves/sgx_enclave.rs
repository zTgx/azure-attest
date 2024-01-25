use azure_svc_attestation::models::AttestationResponse;
use reqwest::blocking::Client;
use serde_json::Value;

use super::model::AttestationRequest;
use super::{
	enclave_info::{EnclaveInfo, ShowTime},
	model::{DataType, RunTimeData},
};
use crate::{
	config::Config,
	utils::{base64, decode_attest_result, read_string_from_file},
	MAA,
};

pub struct SgxEnclave;

impl MAA for SgxEnclave {
	fn azure_attest(&self) {
		let config = Config::default();
		let endpoint = config.endpoint;

		let subscription_key = config.token;
		let bearer_token = format!("Bearer {}", subscription_key);

		let quote = read_string_from_file("quotes/sgx_enclave_quote.txt");
		let quote = hex::decode(quote).unwrap();
		let quote = base64(quote);

		let ehd = read_string_from_file("quotes/sgx_enclave_ehd.txt");
		let ehd = hex::decode(ehd).unwrap();
		let ehd = base64(ehd);

		let runtime_data = RunTimeData::new(ehd, DataType::Binary);
		let request_body = AttestationRequest::new(quote, Some(runtime_data));

		let request_builder = Client::new()
			.post(endpoint)
			.header("Content-Type", "application/json")
			.header("AUTHORIZATION", bearer_token)
			.json(&request_body);

		let response = request_builder.send();
		match response {
			Ok(res) => {
				let value: Value = res.json().unwrap();
				let attest_response: AttestationResponse = serde_json::from_value(value).unwrap();

				// println!("Got AttestationResponse from MAA service: {:#?}", attest_response);

				if let Some(token_body) = attest_response.token {
					// println!("Got token body from MAA service: {:#?}", token_body);

					let attest_result = decode_attest_result(token_body);

					// println!(
					//     "Got AttestationResult from MAA service: {:#?}",
					//     attest_result
					// );

					let enclave_info =
						EnclaveInfo::create_from_file("quotes/enclave.info.securityversion.json");
					enclave_info.show_attest(&attest_result, true);
				}
			},
			Err(_) => {},
		}
	}
}
