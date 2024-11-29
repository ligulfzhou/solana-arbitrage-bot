// use common::reqwest_utils::{get_json_with_params, post_json, post_json_raw};
// use serde::{Deserialize, Serialize};
//
// pub const JUP_V6_API: &str = "https://public.jupiterapi.com";
// pub const JUP_V6_QUOTE_API: &str = "https://public.jupiterapi.com/quote";
// pub const JUP_V6_SWAP_INSTRUCTION_API: &str = "https://public.jupiterapi.com/swap-instructions";
//
// #[derive(Debug, Serialize, Deserialize)]
// // #[serde(rename_all = "camelCase")]
// pub struct QuoteParam {
//     pub input_mint: String,
//     pub output_mint: String,
//     pub amount: u64,
// }
//
// impl QuoteParam {
//     pub fn to_reqwest_param(&self) -> Vec<(String, String)> {
//         vec![
//             ("inputMint".to_string(), self.input_mint.clone()),
//             ("outputMint".to_string(), self.output_mint.clone()),
//             ("amount".to_string(), self.amount.to_string()),
//         ]
//     }
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct QuoteResponse {
//     pub input_mint: String,
//     pub in_amount: String,
//     pub output_mint: String,
//     pub out_amount: String,
//     pub other_amount_threshold: String,
//     pub swap_mode: String,
//     pub slippage_bps: i64,
//     pub platform_fee: PlatformFee,
//     pub price_impact_pct: String,
//     pub route_plan: Vec<RoutePlan>,
//     pub context_slot: i64,
//     pub time_taken: f64,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PlatformFee {
//     pub amount: String,
//     pub fee_bps: i64,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct RoutePlan {
//     pub swap_info: SwapInfo,
//     pub percent: i64,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct SwapInfo {
//     pub amm_key: String,
//     pub label: String,
//     pub input_mint: String,
//     pub output_mint: String,
//     pub in_amount: String,
//     pub out_amount: String,
//     pub fee_amount: String,
//     pub fee_mint: String,
// }
//
// pub async fn get_quote(
//     input_mint: impl ToString,
//     output_mint: impl ToString,
//     amount: u64,
// ) -> anyhow::Result<QuoteResponse> {
//     let quote0_param = QuoteParam {
//         input_mint: input_mint.to_string(),
//         output_mint: output_mint.to_string(),
//         amount,
//     };
//
//     get_json_with_params::<QuoteResponse>(JUP_V6_QUOTE_API, &quote0_param.to_reqwest_param()).await
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct SwapInstructionJson {
//     pub user_public_key: String,
//     pub wrap_and_unwrap_sol: bool,
//     pub use_shared_accounts: bool,
//     pub compute_unit_price_micro_lamports: u64,
//     pub dynamic_compute_unit_limit: bool,
//     pub skip_user_accounts_rpc_calls: bool,
//     pub quote_response: QuoteResponse,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct SwapInstructionsResponse {
//     pub token_ledger_instruction: Value,
//     pub compute_budget_instructions: Vec<ComputeBudgetInstruction>,
//     pub setup_instructions: Vec<SetupInstruction>,
//     pub swap_instruction: SwapInstruction,
//     pub cleanup_instruction: Value,
//     pub other_instructions: Vec<Value>,
//     pub address_lookup_table_addresses: Vec<String>,
//     pub prioritization_fee_lamports: i64,
//     pub compute_unit_limit: i64,
//     pub prioritization_type: PrioritizationType,
//     pub simulation_slot: i64,
//     pub dynamic_slippage_report: Value,
//     pub simulation_error: Value,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ComputeBudgetInstruction {
//     pub program_id: String,
//     pub accounts: Vec<Value>,
//     pub data: String,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct SetupInstruction {
//     pub program_id: String,
//     pub accounts: Vec<Account>,
//     pub data: String,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Account {
//     pub pubkey: String,
//     pub is_signer: bool,
//     pub is_writable: bool,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct SwapInstruction {
//     pub program_id: String,
//     pub accounts: Vec<Account2>,
//     pub data: String,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Account2 {
//     pub pubkey: String,
//     pub is_signer: bool,
//     pub is_writable: bool,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PrioritizationType {
//     pub compute_budget: ComputeBudget,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ComputeBudget {
//     pub micro_lamports: i64,
//     pub estimated_micro_lamports: i64,
// }
//
// pub async fn get_swap_ixs(
//     user_public_key: impl ToString,
//     quote_response: QuoteResponse,
// ) -> anyhow::Result<()> {
//     let json = SwapInstructionJson {
//         user_public_key: user_public_key.to_string(),
//         wrap_and_unwrap_sol: false,
//         use_shared_accounts: false,
//         compute_unit_price_micro_lamports: 1,
//         dynamic_compute_unit_limit: true,
//         skip_user_accounts_rpc_calls: true,
//         quote_response,
//     };
//
//     let res = post_json_raw(JUP_V6_SWAP_INSTRUCTION_API, json).await?;
//
//     dbg!(&res);
//
//     Ok(())
// }
