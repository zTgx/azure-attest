use azure_attest::{
	enclaves::{open_enclave, sgx_enclave::SgxEnclave},
	EnclaveType, MAA,
};

fn main() {
	let enclave = EnclaveType::SgxEnclave;
	match enclave {
		EnclaveType::SgxEnclave => SgxEnclave.azure_attest(),
		EnclaveType::OpenEnclave => open_enclave::verify(),
	}
}
