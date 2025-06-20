use anchor_lang::prelude::*;

declare_id!("ATmAsiEA1Kw9AL2eraMCdyNy3BnuMDEeFGVgBbWaLG6s");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
