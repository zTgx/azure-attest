use azure_attest::{
	enclaves::{sgx_enclave::SgxEnclave, *},
	EnclaveType, MAA,
};

fn main() {
	let enclave = EnclaveType::TestEnclave;
	match enclave {
		EnclaveType::SgxEnclave => SgxEnclave.azure_attest(),
		EnclaveType::TestEnclave => test_enclave::TestEnclave.azure_attest(),
		EnclaveType::OpenEnclave => open_enclave::verify(),
	}
}
