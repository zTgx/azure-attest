use azure_svc_attestation::models::AttestationResponse;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
	config::Config,
	enclave_info::{EnclaveInfo, ShowTime},
	utils::{base64, decode_attest_result, read_string_from_file},
};

#[derive(Serialize, Deserialize)]
enum DataType {
	#[serde(rename = "JSON")]
	Json,

	#[serde(rename = "Binary")]
	Binary,
}

#[derive(Serialize, Deserialize)]
struct RunTimeData {
	pub data: Option<String>, // base64 format
	pub data_type: Option<DataType>,
}

#[derive(Serialize, Deserialize)]
struct AttestationRequest {
	quote: Option<String>,

	#[serde(rename = "runtimeData")]
	runtime_data: Option<RunTimeData>,
}

pub fn azure_attest() {
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

	let request_body = AttestationRequest {
		quote: Some(quote),
		runtime_data: Some(RunTimeData { data: Some(ehd), data_type: Some(DataType::Binary) }),
	};

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
