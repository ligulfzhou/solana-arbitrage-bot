use crate::ix_data::SwapData;
use crate::state::SwapState;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use anchor_spl::token::{Token, TokenAccount};
use solana_program;
use solana_program::instruction::{AccountMeta, Instruction};

pub fn _raydium_amm_swap<'info>(
    ctx: &Context<'_, '_, '_, 'info, RaydiumAmmSwap<'info>>,
    amount_in: u64,
) -> Result<()> {
    // buf.push(9);
    // buf.extend_from_slice(&amount_in.to_le_bytes());
    // buf.extend_from_slice(&minimum_amount_out.to_le_bytes());
    let data = SwapData {
        instruction: 9,
        amount_in,
        minimum_amount_out: 0,
    };

    let ix_accounts = vec![
        // spl token (token program)
        AccountMeta::new_readonly(*ctx.accounts.token_program.key, false),
        // amm
        AccountMeta::new(*ctx.accounts.amm.key, false),
        AccountMeta::new_readonly(*ctx.accounts.amm_authority.key, false),
        AccountMeta::new(*ctx.accounts.amm_open_orders.key, false),
        AccountMeta::new(*ctx.accounts.amm_coin_vault.key, false),
        AccountMeta::new(*ctx.accounts.amm_pc_vault.key, false),
        // market
        AccountMeta::new_readonly(*ctx.accounts.market_program.key, false),
        AccountMeta::new(*ctx.accounts.market.key, false),
        AccountMeta::new(*ctx.accounts.market_bids.key, false),
        AccountMeta::new(*ctx.accounts.market_asks.key, false),
        AccountMeta::new(*ctx.accounts.market_event_queue.key, false),
        AccountMeta::new(*ctx.accounts.market_coin_vault.key, false),
        AccountMeta::new(*ctx.accounts.market_pc_vault.key, false),
        AccountMeta::new_readonly(*ctx.accounts.market_vault_signer.key, false),
        // user
        AccountMeta::new(ctx.accounts.user_token_source.key(), false),
        AccountMeta::new(ctx.accounts.user_token_destination.key(), false),
        AccountMeta::new_readonly(*ctx.accounts.user_source_owner.key, true),
    ];

    let instruction = Instruction {
        program_id: *ctx.accounts.amm_program.key,
        accounts: ix_accounts,
        data: data.try_to_vec()?,
    };

    let accounts = vec![
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.amm.to_account_info(),
        ctx.accounts.amm_authority.to_account_info(),
        ctx.accounts.amm_open_orders.to_account_info(),
        ctx.accounts.amm_coin_vault.to_account_info(),
        ctx.accounts.amm_pc_vault.to_account_info(),
        ctx.accounts.market_bids.to_account_info(),
        ctx.accounts.market_asks.to_account_info(),
        ctx.accounts.market_event_queue.to_account_info(),
        ctx.accounts.market_coin_vault.to_account_info(),
        ctx.accounts.market_pc_vault.to_account_info(),
        ctx.accounts.market_vault_signer.to_account_info(),
        ctx.accounts.user_token_source.to_account_info(),
        ctx.accounts.user_token_destination.to_account_info(),
        ctx.accounts.user_source_owner.to_account_info(),
        ctx.accounts.amm_program.to_account_info(),
    ];

    solana_program::program::invoke(&instruction, &accounts)?;

    Ok(())
}

#[derive(Accounts)]
pub struct RaydiumAmmSwap<'info> {
    // RaydiumAmmSwapBaseIn
    /// CHECK: Safe
    pub amm_program: UncheckedAccount<'info>,
    /// CHECK: Safe. amm Account
    #[account(mut)]
    pub amm: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm authority Account
    #[account()]
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: Safe. amm open_orders Account
    #[account(mut)]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_coin_vault Amm Account to swap FROM or To,
    #[account(mut)]
    pub amm_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_pc_vault Amm Account to swap FROM or To,
    #[account(mut)]
    pub amm_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe.OpenBook program id
    pub market_program: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook market Account. OpenBook program is the owner.
    #[account(mut)]
    pub market: UncheckedAccount<'info>,
    /// CHECK: Safe. bids Account
    #[account(mut)]
    pub market_bids: UncheckedAccount<'info>,
    /// CHECK: Safe. asks Account
    #[account(mut)]
    pub market_asks: UncheckedAccount<'info>,
    /// CHECK: Safe. event_q Account
    #[account(mut)]
    pub market_event_queue: UncheckedAccount<'info>,
    /// CHECK: Safe. coin_vault Account
    #[account(mut)]
    pub market_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. pc_vault Account
    #[account(mut)]
    pub market_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. vault_signer Account
    #[account(mut)]
    pub market_vault_signer: UncheckedAccount<'info>,
    /// CHECK: Safe. user source token Account. user Account to swap from.
    #[account(mut)]
    pub user_token_source: Account<'info, TokenAccount>,
    /// CHECK: Safe. user destination token Account. user Account to swap to.
    #[account(mut)]
    pub user_token_destination: Account<'info, TokenAccount>,
    /// CHECK: Safe. user owner Account
    #[account(mut)]
    pub user_source_owner: Signer<'info>,
    /// CHECK: Safe. The spl token program
    pub token_program: Program<'info, Token>,

    #[account(mut, seeds=[b"swap_state"], bump)]
    pub swap_state: Account<'info, SwapState>,
}
