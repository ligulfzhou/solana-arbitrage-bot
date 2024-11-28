mod keypair_generator;
mod config;

use serde::{Deserialize, Serialize};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const JUP_V6_API: &str = "https://public.jupiterapi.com";
const QUICKNODE_MAINNET: &str = "https://broken-delicate-diamond.solana-mainnet.quiknode.pro/50e19bfb7dfe66ceaafbc998ebfeda75f442d096";


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotaParam {
    pub input_mint: String,
    pub output_mint: String,
    pub amount: u64
}


fn main() {
    println!("Hello, world!");
}
