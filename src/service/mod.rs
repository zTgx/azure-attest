pub mod client;
pub mod maa;
pub mod pipeline;

use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};

/// Serialize a type to json.
pub fn to_json<T>(value: &T) -> azure_core::Result<Bytes>
where
	T: ?Sized + Serialize,
{
	Ok(Bytes::from(serde_json::to_vec(value)?))
}

/// Reads the XML from bytes.
#[allow(dead_code)]
pub fn from_json<S, T>(body: S) -> azure_core::Result<T>
where
	S: AsRef<[u8]>,
	T: DeserializeOwned,
{
	serde_json::from_slice(body.as_ref()).map_err(Into::into)
}
