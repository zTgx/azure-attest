use crate::utils::read_string_from_file;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub token: String,
    pub endpoint: String,
}

impl Config {
    pub fn create_from_file(path: &str) -> Config {
        let contents = read_string_from_file(path);
        let info: Config = serde_json::from_str(&contents).expect("Failed to parse JSON");
        info
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::create_from_file(".config.json")
    }
}
