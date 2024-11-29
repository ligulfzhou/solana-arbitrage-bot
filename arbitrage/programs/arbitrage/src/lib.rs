use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;


declare_id!("4mhXfgu8vPvWfqTMPb9rPFJS23bQB2XZBZekmAfKt9YC");

#[program]
pub mod arbitrage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn arbitrage_sol(ctx: Context<Arbitrage>, data: ArbitrageData) -> Result<()> {
        todo!()
    }
}

#[derive(Accounts)]
pub struct Arbitrage<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub user_src: Account<'info, TokenAccount>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ArbitrageData {
    pub amount_in: u64,
    pub steps: Vec<Step>,
    pub jito_percent: u8,  // 0~100 (20%)
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum Step {
    Raydium(bool),
    RaydiumClmm,
    OrcaWhirpool,
    MeteoraDamm,
    MeteoraDlmm,
}

#[derive(Accounts)]
pub struct Initialize {}
