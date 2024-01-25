use azure_core::auth::{AccessToken, TokenCredential};
use azure_core::{base64, date};
use azure_svc_attestation::models::AttestationResult;
use std::fs::File;
use std::io::Read;
use time::OffsetDateTime;

use crate::config::Config;

#[derive(Debug)]
pub(crate) struct MockCredential;

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for MockCredential {
    async fn get_token(&self, _scopes: &[&str]) -> azure_core::Result<AccessToken> {
        let token = Config::default().token;
        let atoken = AccessToken::new(
            token,
            OffsetDateTime::now_utc() + date::duration_from_days(14),
        );

        Ok(atoken)
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        Ok(())
    }
}

pub fn read_string_from_file(path: &str) -> String {
    let mut file = File::open(path).expect("Failed to open file");

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    // let hex_string = hex::encode(buffer);
    let hex_string = String::from_utf8(buffer).unwrap();

    hex_string
}

pub fn base64(data: Vec<u8>) -> String {
    base64::encode(&data)
}

pub fn decode_attest_result(token: String) -> AttestationResult {
    // use jwt::Token;
    // let token: Token<Header, Claims, _> = Token::parse_unverified(&token).unwrap();
    // println!(("Claims: {:#?}", token.claims());

    let decompose_token: Vec<&str> = token.split(".").collect();
    if decompose_token.len() != 3 {
        println!("JSON Web Tokens must have 3 components delimited by '.' characters.");
    }

    // let token_header = base64::decode(decompose_token[0]).unwrap();
    let token_body = base64::decode(decompose_token[1]).unwrap();
    // let token_sig = base64::decode(decompose_token[2]).unwrap();

    let attest_result: AttestationResult = serde_json::from_slice(&token_body).unwrap();

    attest_result
}
