// use azure_svc_attestation::models::AttestationResult;
use serde::{Deserialize, Serialize};

use crate::utils::{base64, read_string_from_file, AttestationResult};

#[derive(Serialize, Deserialize, Debug)]
pub struct EnclaveInfo {
	#[serde(rename = "Type")]
	pub etype: u8,

	#[serde(rename = "MrEnclaveHex")]
	pub mrenclave_hex: String,

	#[serde(rename = "MrSignerHex")]
	pub mrsigner_hex: String,

	#[serde(rename = "ProductIdHex")]
	pub product_id_hex: String,

	#[serde(rename = "SecurityVersion")]
	pub security_version: f64,

	#[serde(rename = "Attributes")]
	pub attributes: u8,

	#[serde(rename = "QuoteHex")]
	pub quote_hex: String,

	#[serde(rename = "EnclaveHeldDataHex")]
	pub enclave_held_data_hex: String,
}

impl EnclaveInfo {
	pub fn create_from_file(path: &str) -> EnclaveInfo {
		let contents = read_string_from_file(path);
		let info: EnclaveInfo = serde_json::from_str(&contents).expect("Failed to parse JSON");
		info
	}
}

pub trait ShowTime {
	fn show_attest(&self, attest_result: &AttestationResult, include_details: bool);
}

impl ShowTime for EnclaveInfo {
	fn show_attest(&self, attest_result: &AttestationResult, include_details: bool) {
		let is_debuggable = (self.attributes & 2) != 0; // In SGX, DEBUG flag is equal to 0x0000000000000002ULL
		let isdpassed = is_debuggable == attest_result.x_ms_policy.is_debuggable;
		println!("IsDebuggable match                 : {isdpassed}");
		if include_details {
			println!("    We think   : {is_debuggable}");
			println!("    MAA service: {}", attest_result.x_ms_policy.is_debuggable);
		}

		let mrepassed = self.mrenclave_hex ==
			attest_result.x_ms_policy.sgx_mrenclave.clone().to_ascii_uppercase();
		println!("MRENCLAVE match                    : {mrepassed}");
		if include_details {
			println!("    We think   : {}", self.mrenclave_hex);
			println!(
				"    MAA service: {}",
				attest_result.x_ms_policy.sgx_mrenclave.clone().to_ascii_uppercase()
			);
		}

		let mrspassed = self.mrsigner_hex ==
			attest_result.x_ms_policy.sgx_mrsigner.clone().to_ascii_uppercase();
		println!("MRSIGNER match                     : {mrspassed}");
		if include_details {
			println!("    We think   : {}", self.mrsigner_hex);
			println!(
				"    MAA service: {}",
				attest_result.x_ms_policy.sgx_mrsigner.clone().to_ascii_uppercase()
			);
		}

		// let product_id = u64::from_str_radix(&self.product_id_hex, 16).unwrap() as f64;
		// let pidpassed =  product_id == attest_result.product_id.unwrap();
		// println!("ProductID match                    : {pidpassed}");
		// if include_details
		// {
		//     println!("    We think   : {product_id}");
		//     println!("    MAA service: {}", attest_result.product_id.unwrap());
		// }

		let svn_passed = self.security_version == attest_result.x_ms_policy.svn as f64;
		println!("Security Version match             : {svn_passed}");
		if include_details {
			println!("    We think   : {}", self.security_version);
			println!("    MAA service: {}", attest_result.x_ms_policy.svn.to_string());
		}

		let ehd_expected = hex::decode(&self.enclave_held_data_hex).unwrap();
		let ehd_expected = base64(ehd_expected);

		let ehd_actual = attest_result.x_ms_sgx_ehd.clone().unwrap();
		let ehd_passed = ehd_expected == ehd_actual;
		println!("Enclave Held Data match            : {ehd_passed}");
		if include_details {
			println!("    We think   :  {}", ehd_expected);
			println!("    MAA service:  {}", ehd_actual);
		}

		println!("");
	}
}
