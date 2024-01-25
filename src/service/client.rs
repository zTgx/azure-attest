use bytes::Bytes;
use reqwest::blocking::Request;
use serde::{de::DeserializeOwned, Serialize};
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

	pub(crate) fn endpoint(&self) -> &azure_core::Url {
		&self.endpoint
	}

	pub(crate) fn bearer_token(&self) -> &String {
		&self.token
	}

	pub fn attestation_client(&self) -> attestation::Client {
		attestation::Client(self.clone())
	}

	pub(crate) fn send(&self, request: Request) -> azure_core::Result<azure_core::Response> {
		self.pipeline.send(request);

		todo!()
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
	pub fn build(self) -> azure_core::Result<Client> {
		Ok(Client::new(self.token, self.endpoint))
	}
}

pub mod attestation {
	use crate::enclaves::model::{AttestOpenEnclaveRequest, AttestSgxEnclaveRequest};

	pub struct Client(pub(crate) super::Client);
	impl Client {
		#[doc = "Attest to an SGX enclave."]
		#[doc = "Processes an OpenEnclave report , producing an artifact. The type of artifact produced is dependent upon attestation policy."]
		#[doc = ""]
		#[doc = "Arguments:"]
		#[doc = "* `request`: Request object containing the quote"]
		pub fn attest_open_enclave(
			&self,
			request: impl Into<AttestOpenEnclaveRequest>,
		) -> attest_open_enclave::RequestBuilder {
			attest_open_enclave::RequestBuilder { client: self.0.clone(), request: request.into() }
		}

		#[doc = "Attest to an SGX enclave."]
		#[doc = "Processes an SGX enclave quote, producing an artifact. The type of artifact produced is dependent upon attestation policy."]
		#[doc = ""]
		#[doc = "Arguments:"]
		#[doc = "* `request`: Request object containing the quote"]
		pub fn attest_sgx_enclave(
			&self,
			request: impl Into<AttestSgxEnclaveRequest>,
		) -> attest_sgx_enclave::RequestBuilder {
			attest_sgx_enclave::RequestBuilder { client: self.0.clone(), request: request.into() }
		}
	}

	pub mod attest_open_enclave {
		use reqwest::blocking::Request;

		use crate::enclaves::model::{AttestOpenEnclaveRequest, AttestationResponse};

		#[derive(Debug)]
		pub struct Response(azure_core::Response);
		impl Response {
			pub async fn into_body(self) -> azure_core::Result<AttestationResponse> {
				let bytes = self.0.into_body().collect().await?;
				let body: AttestationResponse = serde_json::from_slice(&bytes)?;
				Ok(body)
			}
			pub fn into_raw_response(self) -> azure_core::Response {
				self.0
			}
			pub fn as_raw_response(&self) -> &azure_core::Response {
				&self.0
			}
		}
		impl From<Response> for azure_core::Response {
			fn from(rsp: Response) -> Self {
				rsp.into_raw_response()
			}
		}
		impl AsRef<azure_core::Response> for Response {
			fn as_ref(&self) -> &azure_core::Response {
				self.as_raw_response()
			}
		}
		#[derive(Clone)]
		#[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
		#[doc = r""]
		#[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
		#[doc = r" parameters can be chained."]
		#[doc = r""]
		#[doc = r" To finalize and submit the request, invoke `.await`, which"]
		#[doc = r" which will convert the [`RequestBuilder`] into a future"]
		#[doc = r" executes the request and returns a `Result` with the parsed"]
		#[doc = r" response."]
		#[doc = r""]
		#[doc = r" In order to execute the request without polling the service"]
		#[doc = r" until the operation completes, use `.send().await` instead."]
		#[doc = r""]
		#[doc = r" If you need lower-level access to the raw response details"]
		#[doc = r" (e.g. to inspect response headers or raw body data) then you"]
		#[doc = r" can finalize the request using the"]
		#[doc = r" [`RequestBuilder::send()`] method which returns a future"]
		#[doc = r" that resolves to a lower-level [`Response`] value."]
		pub struct RequestBuilder {
			pub(crate) client: super::super::Client,
			pub(crate) request: AttestOpenEnclaveRequest,
		}
		impl RequestBuilder {
			#[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
			#[doc = ""]
			#[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
			#[doc = "However, this function can provide more flexibility when required."]
			pub fn send(self) -> azure_core::Result<Response> {
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

				let req_body = azure_core::to_json(&self.request)?;
				*req.body_mut() = Some(req_body.into());

				Ok(Response(self.client.send(req)?))
			}

			fn url(&self) -> azure_core::Result<azure_core::Url> {
				let mut url = azure_core::Url::parse(&format!(
					"{}/attest/OpenEnclave",
					self.client.endpoint(),
				))?;
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
		use reqwest::blocking::Request;
		use serde_json::json;
		use url::Url;

		use crate::{
			enclaves::model::{AttestSgxEnclaveRequest, AttestationResponse},
			service::client::to_json,
		};

		#[derive(Debug)]
		pub struct Response(azure_core::Response);
		impl Response {
			pub async fn into_body(self) -> azure_core::Result<AttestationResponse> {
				let bytes = self.0.into_body().collect().await?;
				let body: AttestationResponse = serde_json::from_slice(&bytes)?;
				Ok(body)
			}
			pub fn into_raw_response(self) -> azure_core::Response {
				self.0
			}
			pub fn as_raw_response(&self) -> &azure_core::Response {
				&self.0
			}
		}
		impl From<Response> for azure_core::Response {
			fn from(rsp: Response) -> Self {
				rsp.into_raw_response()
			}
		}
		impl AsRef<azure_core::Response> for Response {
			fn as_ref(&self) -> &azure_core::Response {
				self.as_raw_response()
			}
		}
		#[derive(Clone)]
		#[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
		#[doc = r""]
		#[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
		#[doc = r" parameters can be chained."]
		#[doc = r""]
		#[doc = r" To finalize and submit the request, invoke `.await`, which"]
		#[doc = r" which will convert the [`RequestBuilder`] into a future"]
		#[doc = r" executes the request and returns a `Result` with the parsed"]
		#[doc = r" response."]
		#[doc = r""]
		#[doc = r" In order to execute the request without polling the service"]
		#[doc = r" until the operation completes, use `.send().await` instead."]
		#[doc = r""]
		#[doc = r" If you need lower-level access to the raw response details"]
		#[doc = r" (e.g. to inspect response headers or raw body data) then you"]
		#[doc = r" can finalize the request using the"]
		#[doc = r" [`RequestBuilder::send()`] method which returns a future"]
		#[doc = r" that resolves to a lower-level [`Response`] value."]
		pub struct RequestBuilder {
			pub(crate) client: super::super::Client,
			pub(crate) request: AttestSgxEnclaveRequest,
		}
		impl RequestBuilder {
			#[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
			#[doc = ""]
			#[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
			#[doc = "However, this function can provide more flexibility when required."]
			pub fn send(self) -> azure_core::Result<Response> {
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

				let req_body = to_json(&self.request)?;
				*req.body_mut() = Some(req_body.into());

				Ok(Response(self.client.send(req).unwrap()))
			}
			fn url(&self) -> azure_core::Result<azure_core::Url> {
				let mut url = Url::parse(&format!("{}attest/SgxEnclave", self.client.endpoint(),))?;
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

/// Serialize a type to json.
pub fn to_json<T>(value: &T) -> azure_core::Result<Bytes>
where
	T: ?Sized + Serialize,
{
	Ok(Bytes::from(serde_json::to_vec(value)?))
}

/// Reads the XML from bytes.
pub fn from_json<S, T>(body: S) -> azure_core::Result<T>
where
	S: AsRef<[u8]>,
	T: DeserializeOwned,
{
	serde_json::from_slice(body.as_ref()).map_err(Into::into)
}
