use anchor_lang::{
    prelude::*,
    solana_program::{entrypoint::ProgramResult, instruction::Instruction, program::invoke_signed},
    system_program,
};

declare_id!("5fNnwbi4LvpjYgoYBsgs6kdKb1h16vH8ZwZvM73sV79Q");

pub const AUTHORITY_SEED: &[u8] = b"authority";
pub const WSOL_SEED: &[u8] = b"wsol";

mod jupiter {
    use anchor_lang::declare_id;
    declare_id!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");
}

#[derive(Clone)]
pub struct Jupiter;

impl anchor_lang::Id for Jupiter {
    fn id() -> Pubkey {
        jupiter::id()
    }
}

#[error_code]
pub enum ErrorCode {
    InvalidReturnData,
    InvalidJupiterProgram,
    IncorrectOwner,
}

#[program]
pub mod jup_agg_proxy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn arb(ctx: Context<Arb>) -> Result<()> {
        todo!()
    }
}

fn swap_on_jupiter<'info>(
    remaining_accounts: &[AccountInfo],
    jupiter_program: Program<'info, Jupiter>,
    data: Vec<u8>,
) -> ProgramResult {
    let accounts: Vec<AccountMeta> = remaining_accounts
        .iter()
        .map(|acc| AccountMeta {
            pubkey: *acc.key,
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect();

    // let accounts_infos: Vec<AccountInfo> = remaining_accounts
    //     .iter()
    //     .map(|acc| AccountInfo { ..acc.clone() })
    //     .collect();

    // TODO: Check the first 8 bytes. Only Jupiter Route CPI allowed.
    invoke_signed(
        &Instruction {
            program_id: *jupiter_program.key,
            accounts,
            data,
        },
        &remaining_accounts,
        &[],
    )
}

#[derive(Accounts)]
pub struct Initialize {}
