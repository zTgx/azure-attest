use crate::{
	config::Config,
	utils::{base64, read_string_from_file, MockCredential},
};
use azure_svc_attestation::{
	models::{AttestSgxEnclaveRequest, RuntimeData},
	ClientBuilder,
};
use reqwest::Url;
use std::{str::FromStr, sync::Arc};
use tokio::runtime::Builder;

pub fn verify() {
	let endpoint = Config::default().endpoint;
	let builder =
		ClientBuilder::new(Arc::new(MockCredential)).endpoint(Url::from_str(&endpoint).unwrap());

	let client = builder.build().unwrap();
	let client = client.attestation_client();

	let mut request = AttestSgxEnclaveRequest::new();

	let quote = read_string_from_file("quotes/sgx_enclave_quote.txt");
	let quote = hex::decode(quote).unwrap();
	let quote = base64(quote);
	println!("quote : {:#?}", quote);

	request.quote = Some(quote);

	let mut rundata = RuntimeData::new();

	let ehd = read_string_from_file("quotes/sgx_enclave_ehd.txt");
	let ehd = hex::decode(ehd).unwrap();
	let ehd = base64(ehd);
	println!("ehd : {:#?}", ehd);

	rundata.data = Some(ehd);
	rundata.data_type = Some(azure_svc_attestation::models::DataType::Binary);

	request.runtime_data = Some(rundata);

	let request_builder = client.attest_sgx_enclave(request);

	let rt = Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();

	rt.block_on(async {
		let response = request_builder.send().await;

		println!("response: {:#?}", response);
	});

	// let attestation_result = response.json().unwrap();

	// println!("Is Debuggable: {}", attestation_result.is_debuggable);
	// println!("Is Valid: {}", attestation_result.is_valid);
	// println!("Version: {}", attestation_result.version);
}
