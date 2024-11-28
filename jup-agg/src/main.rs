mod config;
mod keypair_generator;
mod mints;

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
// #[serde(rename_all = "camelCase")]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    pub input_mint: String,
    pub in_amount: String,
    pub output_mint: String,
    pub out_amount: String,
    pub other_amount_threshold: String,
    pub swap_mode: String,
    pub slippage_bps: i64,
    pub platform_fee: PlatformFee,
    pub price_impact_pct: String,
    pub route_plan: Vec<RoutePlan>,
    pub context_slot: i64,
    pub time_taken: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlatformFee {
    pub amount: String,
    pub fee_bps: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RoutePlan {
    pub swap_info: SwapInfo,
    pub percent: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SwapInfo {
    pub amm_key: String,
    pub label: String,
    pub input_mint: String,
    pub output_mint: String,
    pub in_amount: String,
    pub out_amount: String,
    pub fee_amount: String,
    pub fee_mint: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let quote0_param = QuoteParam {
        input_mint: W_SOL_MINT.to_string(),
        output_mint: USDC_MINT.to_string(),
        amount: LAMPORTS_PER_SOL,
    };

    let quote0_res =
        get_json_with_params::<QuoteResponse>(JUP_V6_QUOTE_API, &quote0_param.to_reqwest_param())
            .await?;
    // dbg!(&quote0_res);

    let quote1_param = QuoteParam {
        input_mint: USDC_MINT.to_string(),
        output_mint: W_SOL_MINT.to_string(),
        amount: quote0_res.out_amount.parse()?,
    };

    let quote1_res =
        get_json_with_params::<QuoteResponse>(JUP_V6_QUOTE_API, &quote1_param.to_reqwest_param())
            .await?;
    // dbg!(&quote1_res);

    let output = quote1_res.out_amount.clone().parse::<u64>()?;
    let input = quote0_param.amount;

    if output > input {
        println!(
            "有的赚, 赚{:?}",
            (output - input) as f64 / LAMPORTS_PER_SOL as f64
        );
    } else {
        println!(
            "没有套利机会: 会亏损: {:?} Sol",
            (input - output) as f64 / LAMPORTS_PER_SOL as f64
        );
    }

    Ok(())
}
