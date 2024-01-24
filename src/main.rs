mod open_enclave;
mod request;
mod sgx_enclave;
mod utils;

pub enum EnclaveType {
    SgxEnclave,
    OpenEnclave,
    CustomEnclave,
}

fn main() {
    let enclave = EnclaveType::CustomEnclave;
    match enclave {
        EnclaveType::SgxEnclave => sgx_enclave::verify(),
        EnclaveType::OpenEnclave => open_enclave::verify(),
        EnclaveType::CustomEnclave => request::azure_attest(),
    }
}
