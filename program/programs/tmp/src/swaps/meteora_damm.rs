use crate::state::SwapState;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use solana_program::instruction::Instruction;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MeteoraDammData {
    pub discriminator: u64,
    pub in_amount: u64,
    pub minimum_out_amount: u64,
}

pub fn _meteora_damm_swap<'info>(
    ctx: &Context<'_, '_, '_, 'info, MeteoraDammSwap<'info>>,
    amount_in: u64,
) -> Result<()> {
    let data = MeteoraDammData {
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
pub struct MeteoraDammSwap<'info> {
    #[account(mut)]
    /// CHECK: Pool account (PDA)
    pub pool: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: User token account. Token from this account will be transfer into the vault by the pool in exchange for another token of the pool.
    pub user_source_token: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: User token account. The exchanged token will be transfer into this account from the pool.
    pub user_destination_token: Account<'info, TokenAccount>,

    #[account(mut)]
    /// CHECK: Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Token vault account of vault A
    pub a_token_vault: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Token vault account of vault B
    pub b_token_vault: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Lp token mint of vault a
    pub a_vault_lp_mint: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Lp token mint of vault b
    pub b_vault_lp_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Admin fee token account. Used to receive trading fee. It's mint field must matched with user_source_token mint field.
    pub admin_token_fee: UncheckedAccount<'info>,
    // Protocol Token Fee
    /// CHECK: User account. Must be owner of user_source_token.
    pub user: Signer<'info>,

    /// CHECK: Vault program. the pool will deposit/withdraw liquidity from the vault.
    pub vault_program: UncheckedAccount<'info>,
    /// CHECK: Token program.
    pub token_program: UncheckedAccount<'info>,

    #[account()]
    /// CHECK: Dynamic AMM program account
    pub dynamic_amm_program: UncheckedAccount<'info>,
    #[account(mut, seeds=[b"swap_state"], bump)]
    pub swap_state: Account<'info, SwapState>,
}
