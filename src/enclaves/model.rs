use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum DataType {
	#[serde(rename = "JSON")]
	Json,

	#[serde(rename = "Binary")]
	Binary,
}

#[derive(Serialize, Deserialize)]
pub struct RunTimeData {
	pub data: String, // base64 format
	pub data_type: DataType,
}

impl RunTimeData {
	pub fn new(data: String, data_type: DataType) -> RunTimeData {
		Self { data, data_type }
	}
}

#[derive(Serialize, Deserialize)]
pub struct AttestationRequest {
	pub(crate) quote: String,

	#[serde(rename = "runtimeData")]
	pub(crate) runtime_data: Option<RunTimeData>,
}

impl AttestationRequest {
	pub fn new(quote: String, runtime_data: Option<RunTimeData>) -> AttestationRequest {
		Self { quote, runtime_data }
	}
}
