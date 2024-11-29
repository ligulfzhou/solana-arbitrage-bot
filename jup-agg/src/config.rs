use dotenv::dotenv;

pub struct Config {
    pub mnemonic_code: String,
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok();
        let mnemonic_code = dotenv::var("MNEMONIC_CODE").expect("MNEMONIC_CODE must be defined");

        Self { mnemonic_code }
    }
}
