use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::utils::{base64, read_data};

#[derive(Serialize, Deserialize)]
enum DataType {
    #[serde(rename = "JSON")]
    Json,

    #[serde(rename = "Binary")]
    Binary,
}

#[derive(Serialize, Deserialize)]
struct RunTimeData {
    pub data: Option<String>, // base64 format
    pub data_type: Option<DataType>,
}

#[derive(Serialize, Deserialize)]
struct AttestationRequest {
    quote: Option<String>,

    #[serde(rename = "runtimeData")]
    runtime_data: Option<RunTimeData>,
}

pub fn azure_attest() {
    let endpoint =
        "https://testazureprovider.eus.attest.azure.net/attest/SgxEnclave?api-version=2020-10-01";

    let subscription_key = read_data(".token");
    let bearer_token = format!("Bearer {}", subscription_key);

    let quote = read_data("quotes/sgx_enclave_quote.txt");
    let quote = hex::decode(quote).unwrap();
    let quote = base64(quote);

    let ehd = read_data("quotes/sgx_enclave_ehd.txt");
    let ehd = hex::decode(ehd).unwrap();
    let ehd = base64(ehd);

    let request_body = AttestationRequest {
        quote: Some(quote),
        runtime_data: Some(RunTimeData {
            data: Some(ehd),
            data_type: Some(DataType::Binary),
        }),
    };

    let request_builder = Client::new()
        .post(endpoint)
        .header("Content-Type", "application/json")
        .header("AUTHORIZATION", bearer_token)
        .json(&request_body);

    let response = request_builder.send();
    match response {
        Ok(res) => {
            println!("response: {}", res.text().unwrap());
        }
        Err(_) => {}
    }
}
