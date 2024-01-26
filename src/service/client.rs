use reqwest::blocking::{Request, Response};
use url::Url;

use super::pipeline::Pipeline;

#[derive(Clone)]
pub struct Client {
	token: String,
	endpoint: Url,
	pipeline: Pipeline,
}

impl Client {
	pub fn new(token: String, endpoint: Url) -> Self {
		let pipeline = Pipeline::new();
		Self { token, endpoint, pipeline }
	}

	pub(crate) fn endpoint(&self) -> &Url {
		&self.endpoint
	}

	pub(crate) fn bearer_token(&self) -> &String {
		&self.token
	}

	pub fn attestation_client(&self) -> attestation::Client {
		attestation::Client(self.clone())
	}

	pub(crate) fn send(&self, request: Request) -> Result<Response, String> {
		self.pipeline.send(request)
	}
}

#[derive(Clone)]
pub struct ClientBuilder {
	token: String,
	endpoint: Url,
}

impl ClientBuilder {
	#[doc = "Create a new instance of `ClientBuilder`."]
	#[must_use]
	pub fn new(token: String, endpoint: Url) -> Self {
		Self { token, endpoint }
	}

	#[doc = "Convert the builder into a `Client` instance."]
	pub fn build(self) -> Result<Client, String> {
		Ok(Client::new(self.token, self.endpoint))
	}
}

pub mod attestation {
	use crate::enclaves::model::{AttestOpenEnclaveRequest, AttestSgxEnclaveRequest};

	pub struct Client(pub(crate) super::Client);
	impl Client {
		#[doc = "Attest to an OPEN enclave."]
		pub fn attest_open_enclave(
			&self,
			request: impl Into<AttestOpenEnclaveRequest>,
		) -> attest_open_enclave::RequestBuilder {
			attest_open_enclave::RequestBuilder { client: self.0.clone(), request: request.into() }
		}

		#[doc = "Attest to an SGX enclave."]
		pub fn attest_sgx_enclave(
			&self,
			request: impl Into<AttestSgxEnclaveRequest>,
		) -> attest_sgx_enclave::RequestBuilder {
			attest_sgx_enclave::RequestBuilder { client: self.0.clone(), request: request.into() }
		}
	}

	pub mod attest_open_enclave {
		use crate::{enclaves::model::AttestOpenEnclaveRequest, service::to_json};
		use reqwest::blocking::{Request, Response};
		use url::Url;

		#[derive(Clone)]
		#[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
		pub struct RequestBuilder {
			pub(crate) client: super::super::Client,
			pub(crate) request: AttestOpenEnclaveRequest,
		}
		impl RequestBuilder {
			pub fn send(self) -> Result<Response, String> {
				let url = self.url().unwrap();

				let mut req = Request::new(reqwest::Method::POST, url);
				let headers = req.headers_mut();

				let bearer_token = self.client.bearer_token();
				headers.insert(
					reqwest::header::AUTHORIZATION,
					reqwest::header::HeaderValue::from_str(&format!("Bearer {}", bearer_token))
						.unwrap(),
				);
				headers.insert(
					"content-type",
					reqwest::header::HeaderValue::from_static("application/json"),
				);

				let req_body = to_json(&self.request).unwrap();
				*req.body_mut() = Some(req_body.into());

				self.client.send(req)
			}

			fn url(&self) -> Result<Url, String> {
				let mut url =
					Url::parse(&format!("{}attest/OpenEnclave", self.client.endpoint(),)).unwrap();
				let has_api_version_already =
					url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
				if !has_api_version_already {
					url.query_pairs_mut()
						.append_pair(azure_core::query_param::API_VERSION, "2020-10-01");
				}
				Ok(url)
			}
		}
	}
	pub mod attest_sgx_enclave {
		use reqwest::blocking::{Request, Response};
		use url::Url;

		use crate::{enclaves::model::AttestSgxEnclaveRequest, service::to_json};

		#[derive(Clone)]
		#[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
		pub struct RequestBuilder {
			pub(crate) client: super::super::Client,
			pub(crate) request: AttestSgxEnclaveRequest,
		}
		impl RequestBuilder {
			pub fn send(self) -> Result<Response, String> {
				let url = self.url()?;

				let mut req = Request::new(reqwest::Method::POST, url);
				let headers = req.headers_mut();

				let bearer_token = self.client.bearer_token();
				headers.insert(
					reqwest::header::AUTHORIZATION,
					reqwest::header::HeaderValue::from_str(&format!("Bearer {}", bearer_token))
						.unwrap(),
				);
				headers.insert(
					"content-type",
					reqwest::header::HeaderValue::from_static("application/json"),
				);

				let req_body = to_json(&self.request).unwrap();
				*req.body_mut() = Some(req_body.into());

				self.client.send(req)
			}
			fn url(&self) -> Result<Url, String> {
				let mut url =
					Url::parse(&format!("{}attest/SgxEnclave", self.client.endpoint(),)).unwrap();
				let has_api_version_already =
					url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
				if !has_api_version_already {
					url.query_pairs_mut()
						.append_pair(azure_core::query_param::API_VERSION, "2020-10-01");
				}
				Ok(url)
			}
		}
	}
}
