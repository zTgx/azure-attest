use crate::config::Config;
use crate::enclaves::model::{AttestOpenEnclaveRequest, AttestSgxEnclaveRequest};
use crate::service::client::ClientBuilder;
use crate::utils::{base64, read_string_from_file};
use std::str::FromStr;
use url::Url;

pub fn verify() {
	let config = Config::default();

	let client = ClientBuilder::new(config.token, Url::from_str(&config.endpoint).unwrap())
		.build()
		.unwrap()
		.attestation_client();

	let mut request = AttestOpenEnclaveRequest::new();

	let report = read_string_from_file("quotes/open_enclave_quote.txt");
	let report = hex::decode(report).unwrap();
	let report = base64(report);

	request.report = Some(report);

	let request_builder = client.attest_open_enclave(request);
	let response = request_builder.send().unwrap();

	println!("Open Enclave response: {:#?}", response);
}
