use common::reqwest_utils::get_json_with_params;
use serde::{Deserialize, Serialize};
use solana_program::native_token::LAMPORTS_PER_SOL;

pub const JUP_V6_API: &str = "https://public.jupiterapi.com";
pub const JUP_V6_QUOTE_API: &str = "https://public.jupiterapi.com/quote";

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

pub async fn get_quote(
    input_mint: impl ToString,
    output_mint: impl ToString,
    amount: u64,
) -> anyhow::Result<QuoteResponse> {
    let quote0_param = QuoteParam {
        input_mint: input_mint.to_string(),
        output_mint: output_mint.to_string(),
        amount,
    };

    get_json_with_params::<QuoteResponse>(JUP_V6_QUOTE_API, &quote0_param.to_reqwest_param()).await
}

pub fn get_ixs() {}
