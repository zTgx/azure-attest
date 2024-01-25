use crate::config::Config;
use crate::utils::{read_string_from_file, MockCredential};
use azure_core::Url;
use azure_svc_attestation::models::AttestSgxEnclaveRequest;
use azure_svc_attestation::{models::RuntimeData, ClientBuilder};
use std::str::FromStr;
use std::sync::Arc;
use tokio::runtime::Builder;

pub fn verify() {
	let endpoint = Config::default().endpoint;

	let builder =
		ClientBuilder::new(Arc::new(MockCredential)).endpoint(Url::from_str(&endpoint).unwrap());

	let client = builder.build().unwrap();
	let client = client.attestation_client();

	let mut request = AttestSgxEnclaveRequest::new();

	let quote = read_string_from_file("quotes/open_enclave_quote.txt");
	println!("quote len: {}", quote.len());

	request.quote = Some(quote);

	let mut rundata = RuntimeData::new();
	let ehd = read_string_from_file("quotes/open_enclave_ehd.txt");
	rundata.data = Some(ehd);
	rundata.data_type = Some(azure_svc_attestation::models::DataType::Binary);

	request.runtime_data = Some(rundata);
	// request.init_time_data = Some(InitTimeData::new());
	// request.draft_policy_for_attestation = None;
	// request.nonce = None;

	let request_builder = client.attest_sgx_enclave(request);

	let rt = Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();

	rt.block_on(async {
		let response = request_builder.send().await;

		println!("Open Enclave response: {:#?}", response);
	});
}
