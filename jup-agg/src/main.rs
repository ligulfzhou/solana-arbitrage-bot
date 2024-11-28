mod config;
mod keypair_generator;

use common::reqwest_utils::get_json_with_params;
use serde::{Deserialize, Serialize};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::pubkey::Pubkey;
use solana_sdk::pubkey;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const JUP_V6_API: &str = "https://public.jupiterapi.com";
const JUP_V6_QUOTE_API: &str = "https://public.jupiterapi.com/quote";
const QUICKNODE_MAINNET: &str = "https://broken-delicate-diamond.solana-mainnet.quiknode.pro/50e19bfb7dfe66ceaafbc998ebfeda75f442d096";

const W_SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteParam {
    pub input_mint: String,
    pub output_mint: String,
    pub amount: u64,
}

impl QuoteParam {
    pub fn to_reqwest_param(&self) -> Vec<(String, String)> {
        vec![
            ("inputMint".to_string(), self.input_mint.clone()),
            ("outputMint".to_string(), self.output_mint.clone()),
            ("amount".to_string(), self.amount.to_string()),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    input_mint: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let param = QuoteParam {
        input_mint: W_SOL_MINT.to_string(),
        output_mint: USDC_MINT.to_string(),
        amount: LAMPORTS_PER_SOL,
    }
    .to_reqwest_param();

    let res = get_json_with_params::<QuoteResponse>(JUP_V6_QUOTE_API, &param).await?;
    dbg!(res);

    Ok(())
}
