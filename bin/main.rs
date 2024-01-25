use azure_attest::{
	enclaves::{open_enclave, sgx_enclave::SgxEnclave, test_enclave},
	EnclaveType, MAA,
};

fn main() {
	let enclave = EnclaveType::OpenEnclave;
	match enclave {
		EnclaveType::SgxEnclave => SgxEnclave.azure_attest(),
		// EnclaveType::OpenEnclave => test_enclave::TestEnclave.azure_attest(),
		EnclaveType::OpenEnclave => open_enclave::verify(),
	}
}
