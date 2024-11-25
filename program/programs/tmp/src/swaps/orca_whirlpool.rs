use crate::state::SwapState;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use anchor_spl::token::TokenAccount;
use solana_program;
use solana_program::instruction::Instruction;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AccountsType {
    TransferHookA,
    TransferHookB,
    TransferHookReward,
    TransferHookInput,
    TransferHookIntermediate,
    TransferHookOutput,
    SupplementalTickArrays,
    SupplementalTickArraysOne,
    SupplementalTickArraysTwo,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RemainingAccountsSlice {
    pub accounts_type: AccountsType,
    pub length: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RemainingAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct OrcaWhirlpoolData {
    pub discriminator: u64,
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

pub fn _orca_whirlpool_swap_v2<'info>(
    ctx: &Context<'_, '_, '_, 'info, OrcaWhirlpoolSwapV2<'info>>,
    amount_in: u64,
    other_amount_threshold: u64,
    sqrt_price_limit: u128,
    a_to_b: bool,
) -> Result<()> {
    let data = OrcaWhirlpoolData {
        discriminator: 7070309578724672555,
        amount: amount_in,
        other_amount_threshold,
        sqrt_price_limit,
        amount_specified_is_input: true,
        a_to_b,
        remaining_accounts_info: None,
    };

    let ix_accounts = vec![
        AccountMeta::new_readonly(*ctx.accounts.token_program_a.key, false),
        AccountMeta::new_readonly(ctx.accounts.token_program_b.key(), false),
        AccountMeta::new_readonly(*ctx.accounts.memo_program.key, false),
        AccountMeta::new_readonly(*ctx.accounts.token_authority.key, true),
        AccountMeta::new(*ctx.accounts.whirlpool.key, false),
        AccountMeta::new_readonly(*ctx.accounts.token_mint_a.key, false),
        AccountMeta::new_readonly(*ctx.accounts.token_mint_b.key, false),
        AccountMeta::new(ctx.accounts.token_owner_account_a.key(), false),
        AccountMeta::new(*ctx.accounts.token_vault_a.key, false),
        AccountMeta::new(ctx.accounts.token_owner_account_b.key(), false),
        AccountMeta::new(*ctx.accounts.token_vault_b.key, false),
        AccountMeta::new(*ctx.accounts.tick_array_0.key, false),
        AccountMeta::new(*ctx.accounts.tick_array_1.key, false),
        AccountMeta::new(*ctx.accounts.tick_array_2.key, false),
        AccountMeta::new(*ctx.accounts.oracle.key, false),
    ];

    let instruction = Instruction {
        program_id: *ctx.accounts.whirlpool_program.key,
        accounts: ix_accounts,
        data: data.try_to_vec()?,
    };

    let accounts = vec![
        ctx.accounts.token_program_a.to_account_info(),
        ctx.accounts.token_program_b.to_account_info(),
        ctx.accounts.memo_program.to_account_info(),
        ctx.accounts.token_authority.to_account_info(),
        ctx.accounts.whirlpool.to_account_info(),
        ctx.accounts.token_mint_a.to_account_info(),
        ctx.accounts.token_mint_b.to_account_info(),
        ctx.accounts.token_owner_account_a.to_account_info(),
        ctx.accounts.token_vault_a.to_account_info(),
        ctx.accounts.token_owner_account_b.to_account_info(),
        ctx.accounts.token_vault_b.to_account_info(),
        ctx.accounts.tick_array_0.to_account_info(),
        ctx.accounts.tick_array_1.to_account_info(),
        ctx.accounts.tick_array_2.to_account_info(),
        ctx.accounts.oracle.to_account_info(),
        ctx.accounts.whirlpool_program.to_account_info(),
    ];

    solana_program::program::invoke(&instruction, &accounts)?;
    Ok(())
}

// https://github.com/orca-so/whirlpool-cpi/blob/fe994b783d95869b6ccfbd183ef7454936e74331/versions/0.29.0/src/context.rs#L1165

// https://solscan.io/tx/GuLKJQamV8eYDLkdNhPtsAkMrRshCp4aVPM1LbwhjDbpw7QTrqSVfJy7PHPPDUmMq4vzDmmL5rrDoodLRdJRFUa
#[derive(Accounts)]
pub struct OrcaWhirlpoolSwapV2<'info> {
    /// spl_token::ID ??
    /// CHECK: not care
    #[account()]
    pub token_program_a: UncheckedAccount<'info>,
    /// CHECK: not care
    #[account()]
    pub token_program_b: UncheckedAccount<'info>,

    /// CHECK: not care
    pub memo_program: UncheckedAccount<'info>,

    pub token_authority: Signer<'info>,

    /// CHECK: not care
    #[account(mut)]
    pub whirlpool: UncheckedAccount<'info>,

    /// CHECK: not care
    #[account()]
    pub token_mint_a: UncheckedAccount<'info>,
    /// CHECK: not care
    #[account()]
    pub token_mint_b: UncheckedAccount<'info>,

    #[account(mut)]
    pub token_owner_account_a: Account<'info, TokenAccount>,
    /// CHECK: not care
    #[account(mut)]
    pub token_vault_a: UncheckedAccount<'info>,

    #[account(mut)]
    pub token_owner_account_b: Account<'info, TokenAccount>,
    /// CHECK: not care
    #[account(mut)]
    pub token_vault_b: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_0: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_1: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_2: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Oracle is currently unused and will be enabled on subsequent updates
    pub oracle: UncheckedAccount<'info>,
    // remaining accounts
    // - accounts for transfer hook program of token_mint_a
    // - accounts for transfer hook program of token_mint_b
    // - supplemental TickArray accounts
    /// CHECK: not care
    pub whirlpool_program: UncheckedAccount<'info>,
    #[account(mut, seeds=[b"swap_state"], bump)]
    pub swap_state: Account<'info, SwapState>,
}

#[derive(Accounts)]
pub struct OrcaWhirlpoolSwap<'info> {
    /// CHECK: not care
    #[account()]
    pub token_program: UncheckedAccount<'info>,

    pub token_authority: Signer<'info>,

    /// CHECK: not care
    #[account(mut)]
    pub whirlpool: UncheckedAccount<'info>,

    /// CHECK: not care
    #[account()]
    pub token_mint_a: UncheckedAccount<'info>,
    /// CHECK: not care
    #[account()]
    pub token_mint_b: UncheckedAccount<'info>,

    #[account(mut)]
    pub token_owner_account_a: Account<'info, TokenAccount>,
    /// CHECK: not care
    #[account(mut)]
    pub token_vault_a: UncheckedAccount<'info>,

    #[account(mut)]
    pub token_owner_account_b: Account<'info, TokenAccount>,
    /// CHECK: not care
    #[account(mut)]
    pub token_vault_b: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_0: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_1: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_2: UncheckedAccount<'info>,

    #[account(mut, seeds = [b"oracle", whirlpool.key().as_ref()], bump)]
    /// CHECK: Oracle is currently unused and will be enabled on subsequent updates
    pub oracle: UncheckedAccount<'info>,
    // remaining accounts
    // - accounts for transfer hook program of token_mint_a
    // - accounts for transfer hook program of token_mint_b
    // - supplemental TickArray accounts
    /// CHECK: not care
    pub whirlpool_program: UncheckedAccount<'info>,
    #[account(mut, seeds=[b"swap_state"], bump)]
    pub swap_state: Account<'info, SwapState>,
}
