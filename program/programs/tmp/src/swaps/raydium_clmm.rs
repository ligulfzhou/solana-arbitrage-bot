use crate::state::SwapState;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use anchor_spl::token::{Token, TokenAccount};
use anchor_spl::token_interface::{Mint, Token2022};
use solana_program;

// swap_v2 https://solscan.io/tx/3fcfV3ZaxHTjpSLStZaLop8m3QgsAZcRRxBBan4vYgKcgcGppsoJsK7wzkHTsAYReKs79bgefgYnThvb5pyUokiS
//         https://solscan.io/tx/2APhCCfTCRfuM7LDPdySftD1AZsxEPZzzkFLgfeonze1wWCQjoGvf74DvuQXhfu77h7iFDaf7Xa1jU4qjxWhXNY6
// swap_v1 https://solscan.io/tx/3MrSpZJMSwT94otpVvLXydgUkyDgPTmSXDwsxBaZzwXVYxsxQ7MV55aK7hVMfcux6uJDwng8VGxovhfKDUBJedcm

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RaydiumClmmData {
    pub discriminator: u64,
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: bool,
}

pub fn _raydium_clmm_swap<'info>(
    ctx: &Context<'_, '_, '_, 'info, RaydiumClmmSwap<'info>>,
    amount_in: u64,
) -> Result<()> {
    let data = RaydiumClmmData {
        discriminator: 14449647541112719096,
        amount: amount_in,
        other_amount_threshold: 0,
        sqrt_price_limit_x64: 0,
        is_base_input: true,
    };

    let ix_accounts = vec![
        AccountMeta::new(ctx.accounts.payer.key(), true),
        AccountMeta::new_readonly(*ctx.accounts.amm_config.key, false),
        AccountMeta::new(*ctx.accounts.pool_state.key, false),
        AccountMeta::new(ctx.accounts.input_token_account.key(), false),
        AccountMeta::new(ctx.accounts.output_token_account.key(), false),
        AccountMeta::new(*ctx.accounts.input_vault.key, false),
        AccountMeta::new(*ctx.accounts.output_vault.key, false),
        AccountMeta::new(*ctx.accounts.observation_state.key, false),
        AccountMeta::new_readonly(*ctx.accounts.token_program.key, false),
        AccountMeta::new_readonly(*ctx.accounts.token_program_2022.key, false),
        AccountMeta::new_readonly(*ctx.accounts.memo_program.key, false),
        AccountMeta::new_readonly(*ctx.accounts.input_vault_mint.key, false),
        AccountMeta::new_readonly(*ctx.accounts.output_vault_mint.key, false),
    ];
    // let cpi_accounts = vec! [
    //     payer: ctx.accounts.payer.to_account_info(),
    //     amm_config: ctx.accounts.amm_config.to_account_info(),
    //     pool_state: ctx.accounts.pool_state.to_account_info(),
    //     input_token_account: ctx.accounts.input_token_account.to_account_info(),
    //     output_token_account: ctx.accounts.output_token_account.to_account_info(),
    //     input_vault: ctx.accounts.input_vault.to_account_info(),
    //     output_vault: ctx.accounts.output_vault.to_account_info(),
    //     observation_state: ctx.accounts.observation_state.to_account_info(),
    //     token_program: ctx.accounts.token_program.to_account_info(),
    //     token_program_2022: ctx.accounts.token_program_2022.to_account_info(),
    //     memo_program: ctx.accounts.memo_program.to_account_info(),
    //     input_vault_mint: ctx.accounts.input_vault_mint.to_account_info(),
    //     output_vault_mint: ctx.accounts.output_vault_mint.to_account_info(),
    // ];

    // let cpi_context = CpiContext::new(ctx.accounts.clmm_program.to_account_info(), cpi_accounts)
    //     .with_remaining_accounts(ctx.remaining_accounts.to_vec());
    //
    // cpi::swap_v2(
    //     cpi_context,
    //     amount,
    //     other_amount_threshold,
    //     sqrt_price_limit_x64,
    //     is_base_input,
    // )

    todo!()
}
/// Memo msg for swap
pub const SWAP_MEMO_MSG: &'static [u8] = b"raydium_swap";

#[derive(Accounts)]
pub struct RaydiumClmmSwap<'info> {
    /// The user performing the swap
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The factory state to read protocol fees
    /// CHECK: not care
    #[account()]
    pub amm_config: UncheckedAccount<'info>,

    /// The program account of the pool in which the swap will be performed
    /// CHECK: not care
    #[account(mut)]
    pub pool_state: UncheckedAccount<'info>,

    /// The user token account for input token
    #[account(mut)]
    pub input_token_account: Account<'info, TokenAccount>,

    /// The user token account for output token
    #[account(mut)]
    pub output_token_account: Account<'info, TokenAccount>,

    /// The vault token account for input token
    /// CHECK: not care
    #[account(mut)]
    pub input_vault: UncheckedAccount<'info>,
    // pub input_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    /// The vault token account for output token
    /// CHECK: not care
    #[account(mut)]
    pub output_vault: UncheckedAccount<'info>,
    // pub output_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    /// The program account for the most recent oracle observation
    // #[account(mut, address = pool_state.load()?.observation_key)]
    /// CHECK: not care
    #[account(mut)]
    pub observation_state: UncheckedAccount<'info>,

    /// SPL program for token transfers
    pub token_program: Program<'info, Token>,

    /// SPL program 2022 for token transfers
    pub token_program_2022: Program<'info, Token2022>,

    /// CHECK: not care
    pub memo_program: UncheckedAccount<'info>,

    /// The mint of token vault 0
    /// CHECK: not care
    pub input_vault_mint: UncheckedAccount<'info>,

    /// The mint of token vault 1
    /// CHECK: not care
    pub output_vault_mint: UncheckedAccount<'info>,

    #[account(mut, seeds=[b"swap_state"], bump)]
    pub swap_state: Account<'info, SwapState>,
    // remaining accounts
    // tickarray_bitmap_extension: must add account if need regardless the sequence
    // tick_array_account_1
    // tick_array_account_2
    // tick_array_account_...
}
