use anchor_lang::prelude::*;

declare_id!("5fNnwbi4LvpjYgoYBsgs6kdKb1h16vH8ZwZvM73sV79Q");

#[program]
pub mod jup_agg_proxy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
