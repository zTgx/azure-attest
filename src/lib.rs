mod config;
mod service;
mod utils;

pub mod enclaves;

pub enum EnclaveType {
	SgxEnclave,
	OpenEnclave,
	TestEnclave,
}

pub trait MAA {
	fn azure_attest(&self);
}
