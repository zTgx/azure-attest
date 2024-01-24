use azure_core::auth::{AccessToken, TokenCredential};
use azure_core::{base64, date};
use std::fs::File;
use std::io::Read;
// use hex;
use time::OffsetDateTime;

#[derive(Debug)]
pub(crate) struct MockCredential;

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for MockCredential {
    async fn get_token(&self, _scopes: &[&str]) -> azure_core::Result<AccessToken> {
        let token = read_data(".token");
        Ok(AccessToken::new(
            token.to_owned(),
            OffsetDateTime::now_utc() + date::duration_from_days(14),
        ))
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        Ok(())
    }
}

pub fn read_data(path: &str) -> String {
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
