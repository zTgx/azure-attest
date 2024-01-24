mod open_enclave;
mod sgx_enclave;
mod utils;

pub enum EnclaveType {
    SgxEnclave,
    OpenEnclave,
}

fn main() {
    let enclave = EnclaveType::SgxEnclave;
    match enclave {
        EnclaveType::SgxEnclave => sgx_enclave::verify(),
        EnclaveType::OpenEnclave => open_enclave::verify(),
    }
}
