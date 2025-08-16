use anchor_lang::prelude::*;

declare_id!("8Fu2DQWUC2Rjq8oRTCjya8qfvCuLLf7rocVi3StLsKdz");

#[program]
pub mod lending {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
