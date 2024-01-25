use serde::{Deserialize, Serialize};

#[doc = "Specifies the type of the data encoded contained within the \"data\" field of a \"RuntimeData\" or \"InitTimeData\" object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DataType {
	Binary,
	#[serde(rename = "JSON")]
	Json,
}

impl Default for DataType {
	fn default() -> Self {
		DataType::Binary
	}
}

#[doc = "Initialization time data are a conduit for any configuration information that is unknown when building the Trusted Execution Environment (TEE) and is defined at TEE launch time. This data can be used with confidential container or VM scenarios to capture configuration settings such as disk volume content, network configuration, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InitTimeData {
	#[doc = "Initialization time data are passed into the Trusted Execution Environment (TEE) when it is created. For an Icelake SGX quote, the SHA256 hash of the InitTimeData must match the lower 32 bytes of the quote's \"config id\" attribute. For a SEV-SNP quote, the SHA256 hash of the InitTimeData must match the quote's \"host data\" attribute."]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub data: Option<String>,
	#[doc = "Specifies the type of the data encoded contained within the \"data\" field of a \"RuntimeData\" or \"InitTimeData\" object"]
	#[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
	pub data_type: Option<DataType>,
}
impl InitTimeData {
	pub fn new() -> Self {
		Self::default()
	}
}

#[doc = "Runtime data are a conduit for any information defined by the Trusted Execution Environment (TEE) when actually running."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuntimeData {
	pub data: String, // base64 format
	pub data_type: DataType,
}

impl RuntimeData {
	pub fn new(data: String, data_type: DataType) -> RuntimeData {
		Self { data, data_type }
	}
}

#[derive(Serialize, Deserialize)]
pub struct AttestationRequest {
	pub(crate) quote: String,

	#[serde(rename = "runtimeData")]
	pub(crate) runtime_data: Option<RuntimeData>,
}

impl AttestationRequest {
	pub fn new(quote: String, runtime_data: Option<RuntimeData>) -> AttestationRequest {
		Self { quote, runtime_data }
	}
}

#[doc = "Attestation request for Intel SGX enclaves"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttestSgxEnclaveRequest {
	#[doc = "Quote of the enclave to be attested"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub quote: Option<String>,

	#[doc = "Runtime data are a conduit for any information defined by the Trusted Execution Environment (TEE) when actually running."]
	#[serde(rename = "runtimeData", default, skip_serializing_if = "Option::is_none")]
	pub runtime_data: Option<RuntimeData>,

	#[doc = "Initialization time data are a conduit for any configuration information that is unknown when building the Trusted Execution Environment (TEE) and is defined at TEE launch time. This data can be used with confidential container or VM scenarios to capture configuration settings such as disk volume content, network configuration, etc."]
	#[serde(rename = "initTimeData", default, skip_serializing_if = "Option::is_none")]
	pub init_time_data: Option<InitTimeData>,

	#[doc = "Attest against the provided draft policy. Note that the resulting token cannot be validated."]
	#[serde(
		rename = "draftPolicyForAttestation",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub draft_policy_for_attestation: Option<String>,

	#[doc = "Nonce for incoming request - emitted in the generated attestation token"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub nonce: Option<String>,
}
impl AttestSgxEnclaveRequest {
	pub fn new() -> Self {
		Self::default()
	}
}

#[doc = "The result of an attestation operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttestationResponse {
	#[doc = "An RFC 7519 Json Web Token"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub token: Option<JsonWebToken>,
}
impl AttestationResponse {
	pub fn new() -> Self {
		Self::default()
	}
}

pub type JsonWebToken = String;

#[doc = "Attestation request for Intel SGX enclaves"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttestOpenEnclaveRequest {
	#[doc = "OpenEnclave report from the enclave to be attested"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub report: Option<String>,
	#[doc = "Runtime data are a conduit for any information defined by the Trusted Execution Environment (TEE) when actually running."]
	#[serde(rename = "runtimeData", default, skip_serializing_if = "Option::is_none")]
	pub runtime_data: Option<RuntimeData>,
	#[doc = "Initialization time data are a conduit for any configuration information that is unknown when building the Trusted Execution Environment (TEE) and is defined at TEE launch time. This data can be used with confidential container or VM scenarios to capture configuration settings such as disk volume content, network configuration, etc."]
	#[serde(rename = "initTimeData", default, skip_serializing_if = "Option::is_none")]
	pub init_time_data: Option<InitTimeData>,
	#[doc = "Attest against the provided draft policy. Note that the resulting token cannot be validated."]
	#[serde(
		rename = "draftPolicyForAttestation",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub draft_policy_for_attestation: Option<String>,
	#[doc = "Nonce for incoming request - emitted in the generated attestation token"]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub nonce: Option<String>,
}
impl AttestOpenEnclaveRequest {
	pub fn new() -> Self {
		Self::default()
	}
}
