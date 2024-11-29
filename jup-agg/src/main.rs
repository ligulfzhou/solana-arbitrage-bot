mod config;
// mod jup_api;
mod keypair_generator;
mod mints;

use crate::config::Config;
use std::sync::Arc;
// use crate::jup_api::{get_quote, get_swap_ixs};
use crate::keypair_generator::KeypairGenerator;
use anyhow::bail;
use jupiter_swap_api_client::quote::QuoteRequest;
use jupiter_swap_api_client::swap::SwapRequest;
use jupiter_swap_api_client::transaction_config::TransactionConfig;
use jupiter_swap_api_client::JupiterSwapApiClient;
use serde::{Deserialize, Serialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::pubkey::Pubkey;
use solana_sdk::pubkey;
use solana_sdk::signature::{Keypair, NullSigner, Signer};
use solana_sdk::transaction::VersionedTransaction;
use std::time::Duration;

pub const JUP_V6_API: &str = "https://public.jupiterapi.com";
pub const QUICKNODE_MAINNET: &str = "https://broken-delicate-diamond.solana-mainnet.quiknode.pro/50e19bfb7dfe66ceaafbc998ebfeda75f442d096";
pub const SOLANA_MAINNET: &str = "https://api.mainnet-beta.solana.com";

const W_SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const LUCE: Pubkey = pubkey!("CBdCxKo9QavR9hfShgpEBG3zekorAeD7W1jfq2o3pump");
const ANTI: Pubkey = pubkey!("HB8KrN7Bb3iLWUPsozp67kS4gxtbA4W5QJX4wKPvpump");
const MOODENG: Pubkey = pubkey!("ED5nyyWEzpPPiWimP8vYm7sD7TD3LAt3Q3gRTWHzPJBY");

async fn sol_to_sol() -> anyhow::Result<()> {
    // let initial_amount = LAMPORTS_PER_SOL;
    // let mut quote0_res = get_quote(W_SOL_MINT, W_SOL_MINT, initial_amount).await?;
    // dbg!(&quote0_res);
    //
    // let final_output_amount = quote0_res.out_amount.clone().parse::<u64>()?;
    // if final_output_amount <= initial_amount {
    //     let msg = format!(
    //         "没有套利机会: 会亏损: {:?} Sol",
    //         (initial_amount - final_output_amount) as f64 / LAMPORTS_PER_SOL as f64
    //     );
    //
    //     println!("{}", msg);
    //     bail!(msg);
    // }
    //
    // println!(
    //     "有的赚, 赚{:?}",
    //     (final_output_amount - initial_amount) as f64 / LAMPORTS_PER_SOL as f64
    // );

    Ok(())
}

async fn arbitrage(
    payer: &Keypair,
    jup_client: &JupiterSwapApiClient,
    rpc_client: Arc<RpcClient>,
) -> anyhow::Result<()> {
    println!("arbitrage...");

    let initial_amount = LAMPORTS_PER_SOL / 25;
    let quote0_req = QuoteRequest {
        input_mint: W_SOL_MINT,
        output_mint: MOODENG,
        amount: initial_amount,
        max_accounts: Some(30),
        ..Default::default()
    };
    let mut quote0_res = jup_client.quote(&quote0_req).await?;
    // dbg!(&quote0_res);

    let quote1_req = QuoteRequest {
        input_mint: MOODENG,
        output_mint: W_SOL_MINT,
        amount: quote0_res.out_amount,
        max_accounts: Some(30),
        ..Default::default()
    };
    let quote1_res = jup_client.quote(&quote1_req).await?;
    // dbg!(&quote1_res);

    if quote1_res.out_amount > initial_amount {
        println!(
            "有的赚, 赚{:?}",
            (quote1_res.out_amount - initial_amount) as f64 / LAMPORTS_PER_SOL as f64
        );
    } else {
        let msg = format!(
            "没有套利机会: 会亏损: {:?} Sol",
            (initial_amount - quote1_res.out_amount) as f64 / LAMPORTS_PER_SOL as f64
        );

        dbg!(&msg);

        bail!(msg)
    }

    quote0_res.route_plan.extend(quote1_res.route_plan);

    let swap_req = SwapRequest {
        user_public_key: payer.pubkey(),
        quote_response: quote0_res,
        config: Default::default(),
    };
    let swap_response = jup_client.swap(&swap_req).await?;

    dbg!(&swap_response.swap_transaction);
    dbg!(swap_response.last_valid_block_height);

    println!("Raw tx len: {}", swap_response.swap_transaction.len());

    let versioned_transaction: VersionedTransaction =
        bincode::deserialize(&swap_response.swap_transaction)?;

    // Replace with a keypair or other struct implementing signer
    // let null_signer = NullSigner::new(&);
    let signed_versioned_transaction =
        VersionedTransaction::try_new(versioned_transaction.message, &[payer])?;

    // // send with rpc client...
    // let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".into());

    // This will fail with "Transaction signature verification failure" as we did not really sign
    let sig = rpc_client
        .send_and_confirm_transaction(&signed_versioned_transaction)
        .await?;
    println!("{:}", sig.to_string());

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new();

    let rpc_client = Arc::new(RpcClient::new(SOLANA_MAINNET.to_string()));
    let jup_client = JupiterSwapApiClient::new(JUP_V6_API.to_string());

    let payer = KeypairGenerator::get_keypair_with(&config.mnemonic_code, 0);

    loop {
        if let Err(err) = arbitrage(&payer, &jup_client, rpc_client.clone()).await {
            println!("error occur: {:?}", err);
            tokio::time::sleep(Duration::from_secs(1)).await;
        } else {
            tokio::time::sleep(Duration::from_micros(100)).await;
            break;
        }

        // break;
    }

    // loop {
    //     if let Err(err) = arb(&payer).await {
    //         println!("error occur: {:?}", err);
    //         tokio::time::sleep(Duration::from_secs(1)).await;
    //     } else {
    //         tokio::time::sleep(Duration::from_micros(100)).await;
    //     }
    //
    //     break;
    // }

    Ok(())
}
