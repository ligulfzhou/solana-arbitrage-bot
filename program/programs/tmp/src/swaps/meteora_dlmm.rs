use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount};
use solana_program::instruction::Instruction;
use crate::state::SwapState;


#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MeteoraDlmmData {
    pub discriminator: u64,
    pub in_amount: u64,
    pub minimum_out_amount: u64,
}

pub fn _meteora_dlmm_swap<'info>(
    ctx: &Context<'_, '_, '_, 'info, MeteoraDlmmSwap<'info>>,
    amount_in: u64,
) -> Result<()> {
    let data = MeteoraDlmmData {
        discriminator: 14449647541112719096,
        in_amount: amount_in,
        minimum_out_amount: 0,
    };

    let ix_accounts = vec![
        AccountMeta::new(*ctx.accounts.pool.key, false),
        AccountMeta::new(ctx.accounts.user_source_token.key(), false),
        AccountMeta::new_readonly(ctx.accounts.user_destination_token.key(), false),
        AccountMeta::new(*ctx.accounts.a_vault.key, false),
        AccountMeta::new(*ctx.accounts.b_vault.key, false),
        AccountMeta::new(*ctx.accounts.a_token_vault.key, false),
        AccountMeta::new(*ctx.accounts.b_token_vault.key, false),
        AccountMeta::new(*ctx.accounts.a_vault_lp_mint.key, false),
        AccountMeta::new(*ctx.accounts.b_vault_lp_mint.key, false),
        AccountMeta::new(*ctx.accounts.a_vault_lp.key, false),
        AccountMeta::new(*ctx.accounts.b_vault_lp.key, false),
        AccountMeta::new(*ctx.accounts.admin_token_fee.key, false),
        AccountMeta::new_readonly(*ctx.accounts.user.key, true),
        AccountMeta::new_readonly(*ctx.accounts.vault_program.key, false),
        AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
    ];

    let instruction = Instruction {
        program_id: *ctx.accounts.dynamic_amm_program.key,
        accounts: ix_accounts,
        data: data.try_to_vec()?,
    };

    let accounts = vec![
        ctx.accounts.pool.to_account_info(),
        ctx.accounts.user_source_token.to_account_info(),
        ctx.accounts.user_destination_token.to_account_info(),
        ctx.accounts.a_vault.to_account_info(),
        ctx.accounts.b_vault.to_account_info(),
        ctx.accounts.a_token_vault.to_account_info(),
        ctx.accounts.b_token_vault.to_account_info(),
        ctx.accounts.a_vault_lp_mint.to_account_info(),
        ctx.accounts.b_vault_lp_mint.to_account_info(),
        ctx.accounts.a_vault_lp.to_account_info(),
        ctx.accounts.b_vault_lp.to_account_info(),
        ctx.accounts.admin_token_fee.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.vault_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.dynamic_amm_program.to_account_info(),
    ];

    solana_program::program::invoke(&instruction, &accounts)?;

    Ok(())
}


#[derive(Accounts)]
pub struct MeteoraDlmmSwap<'info> {
    #[account(mut)]
    /// CHECK: The pool account
    pub lb_pair: UncheckedAccount<'info>,

    /// CHECK: Bin array extension account of the pool
    pub bin_array_bitmap_extension: Option<UncheckedAccount<'info>>,

    #[account(mut)]
    /// CHECK: Reserve account of token X
    pub reserve_x: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Reserve account of token Y
    pub reserve_y: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: User token account to sell token
    pub user_token_in: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: User token account to buy token
    pub user_token_out: UncheckedAccount<'info>,

    /// CHECK: Mint account of token X
    pub token_x_mint: UncheckedAccount<'info>,
    /// CHECK: Mint account of token Y
    pub token_y_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Oracle account of the pool
    pub oracle: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Referral fee account
    pub host_fee_in: Option<UncheckedAccount<'info>>,

    /// CHECK: User who's executing the swap
    pub user: Signer<'info>,

    #[account(address = dlmm::ID)]
    /// CHECK: DLMM program
    pub dlmm_program: UncheckedAccount<'info>,

    /// CHECK: DLMM program event authority for event CPI
    pub event_authority: UncheckedAccount<'info>,

    /// CHECK: Token program of mint X
    pub token_x_program: UncheckedAccount<'info>,
    /// CHECK: Token program of mint Y
    pub token_y_program: UncheckedAccount<'info>,
    // Bin arrays need to be passed using remaining accounts

    #[account()]
    /// CHECK: Dynamic AMM program account
    pub dynamic_amm_program: UncheckedAccount<'info>,
    #[account(mut, seeds=[b"swap_state"], bump)]
    pub swap_state: Account<'info, SwapState>,
}
