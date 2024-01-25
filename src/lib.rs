mod config;
mod service;
mod utils;

pub mod enclaves;

pub enum EnclaveType {
	SgxEnclave,
	OpenEnclave,
}

pub trait MAA {
	fn azure_attest(&self);
}
