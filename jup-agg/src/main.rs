mod config;
mod jup_api;
mod keypair_generator;
mod mints;

use crate::jup_api::get_quote;
use serde::{Deserialize, Serialize};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::pubkey::Pubkey;
use solana_sdk::pubkey;

const QUICKNODE_MAINNET: &str = "https://broken-delicate-diamond.solana-mainnet.quiknode.pro/50e19bfb7dfe66ceaafbc998ebfeda75f442d096";

const W_SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let initial_amount = LAMPORTS_PER_SOL;
    let quote0_res = get_quote(W_SOL_MINT, USDC_MINT, initial_amount).await?;
    // dbg!(&quote0_res);

    let output_amount = quote0_res.out_amount.clone().parse::<u64>()?;
    let quote1_res = get_quote(USDC_MINT, W_SOL_MINT, output_amount).await?;
    // dbg!(&quote1_res);

    let final_output_amount = quote1_res.out_amount.clone().parse::<u64>()?;

    if final_output_amount > initial_amount {
        println!(
            "有的赚, 赚{:?}",
            (final_output_amount - initial_amount) as f64 / LAMPORTS_PER_SOL as f64
        );
    } else {
        println!(
            "没有套利机会: 会亏损: {:?} Sol",
            (initial_amount - final_output_amount) as f64 / LAMPORTS_PER_SOL as f64
        );
    }

    Ok(())
}
