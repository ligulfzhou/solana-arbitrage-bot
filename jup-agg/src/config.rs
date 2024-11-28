use std::env;
use dotenv::dotenv;

pub struct Config {
    mnemonic_code: String,
}

impl Config {

    pub fn new() -> Config {
        dotenv().ok();
        let mnemonic_code = env::var("MNEMONIC_CODE").expect("MNEMONIC_CODE must be defined");

        Self {
            mnemonic_code
        }
    }
}