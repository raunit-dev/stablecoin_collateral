use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface::Mint};

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = config_account.bump,
        has_one = authoirty,
        bump,
    )]
    pub config_account: Account<'info, Config>,

}

impl <'info> UpdateConfig <'info> {
    pub fn update_config (&mut self, min_health_factor: u64) -> Result<()> {
        self.config_account.min_health_factor = min_health_factor;
         Ok(())
    }
}