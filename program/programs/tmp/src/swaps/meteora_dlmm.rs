use crate::state::SwapState;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use solana_program::instruction::Instruction;

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

    let mut ix_accounts = vec![
        AccountMeta::new(*ctx.accounts.lb_pair.key, false),
        AccountMeta::new_readonly(*ctx.accounts.bin_array_bitmap_extension.key, false),
        AccountMeta::new(*ctx.accounts.reserve_x.key, false),
        AccountMeta::new(*ctx.accounts.reserve_y.key, false),
        AccountMeta::new(ctx.accounts.user_token_in.key(), false),
        AccountMeta::new(ctx.accounts.user_token_out.key(), false),
        AccountMeta::new_readonly(*ctx.accounts.token_x_mint.key, false),
        AccountMeta::new_readonly(*ctx.accounts.token_y_mint.key, false),
        AccountMeta::new(*ctx.accounts.oracle.key, false),
        AccountMeta::new_readonly(*ctx.accounts.host_fee_in.key, false),
        AccountMeta::new_readonly(*ctx.accounts.user.key, true),
        AccountMeta::new_readonly(*ctx.accounts.token_x_program.key, false),
        AccountMeta::new_readonly(*ctx.accounts.token_y_program.key, false),
        // todo: bin array
        // 3条bin array
        // https://solscan.io/tx/5s7LdseGMMg8Aw3sbzabsYxohY2j3aRpMpv2wRp37vYzFZaaTTwab8M4UDW47qTb9CZonsDC18ANqSpsY9UAo3WF
    ];
    let remain_account_metas = ctx
        .remaining_accounts
        .iter()
        .map(|account| AccountMeta::new(account.key(), false))
        .collect::<Vec<_>>();
    if !remain_account_metas.is_empty() {
        ix_accounts.extend_from_slice(&remain_account_metas);
    }

    let instruction = Instruction {
        program_id: *ctx.accounts.dlmm_program.key,
        accounts: ix_accounts,
        data: data.try_to_vec()?,
    };

    let mut accounts = vec![
        ctx.accounts.lb_pair.to_account_info(),
        ctx.accounts.bin_array_bitmap_extension.to_account_info(),
        ctx.accounts.reserve_x.to_account_info(),
        ctx.accounts.reserve_y.to_account_info(),
        ctx.accounts.user_token_in.to_account_info(),
        ctx.accounts.user_token_out.to_account_info(),
        ctx.accounts.token_x_mint.to_account_info(),
        ctx.accounts.token_y_mint.to_account_info(),
        ctx.accounts.oracle.to_account_info(),
        ctx.accounts.host_fee_in.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.token_x_program.to_account_info(),
        ctx.accounts.token_y_program.to_account_info(),
        ctx.accounts.dlmm_program.to_account_info(),
    ];

    let remain_accounts = ctx
        .remaining_accounts
        .iter()
        .map(|account| account.to_account_info())
        .collect::<Vec<_>>();
    if !remain_accounts.is_empty() {
        accounts.extend_from_slice(&remain_accounts);
    }
    solana_program::program::invoke(&instruction, &accounts)?;

    Ok(())
}

#[derive(Accounts)]
pub struct MeteoraDlmmSwap<'info> {
    #[account(mut)]
    /// CHECK: The pool account
    pub lb_pair: UncheckedAccount<'info>,

    /// TODO,  这里貌似就是DLMM program。。
    /// CHECK: Bin array extension account of the pool
    pub bin_array_bitmap_extension: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Reserve account of token X
    pub reserve_x: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Reserve account of token Y
    pub reserve_y: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: User token account to sell token
    pub user_token_in: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: User token account to buy token
    pub user_token_out: Account<'info, TokenAccount>,

    /// CHECK: Mint account of token X
    pub token_x_mint: UncheckedAccount<'info>,
    /// CHECK: Mint account of token Y
    pub token_y_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Oracle account of the pool
    pub oracle: UncheckedAccount<'info>,

    #[account(mut)]
    /// TODO,  这里貌似就是DLMM program。。
    /// CHECK: Referral fee account
    pub host_fee_in: UncheckedAccount<'info>,

    /// CHECK: User who's executing the swap
    pub user: Signer<'info>,

    /// CHECK: Token program of mint X
    pub token_x_program: UncheckedAccount<'info>,
    /// CHECK: Token program of mint Y
    pub token_y_program: UncheckedAccount<'info>,

    // Bin arrays need to be passed using remaining accounts
    /// CHECK: Dynamic AMM program account
    #[account()]
    pub dlmm_program: UncheckedAccount<'info>,
    #[account(mut, seeds=[b"swap_state"], bump)]
    pub swap_state: Account<'info, SwapState>,
}
