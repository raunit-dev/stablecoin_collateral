use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Config { //this is the global configuration account,Think of it as the system-wide admin settings
    pub authority: Pubkey,//admin or protocol owner address,only this pubkey can update settings or pause/resume the protocol
    pub mint_account: Pubkey,//the spl token mint used to create synthetic tokens
    pub liquidation_threshold: u64,//This sets the risk limit. It tells the system how much collateral is required to keep a loan safe.
    pub liquidation_bonus: u64,
    pub min_health_factor: u64,
    pub bump: u8,//these is the bump for config account itself
    pub bump_mint_account: u8//for the mint PDA
}

//General Formula -> health_factor = (collateral_value * threshold) / minted_value
//If health_factor < min_health_factor, then your position is liquidatable.