use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Collateral {
    pub depositor: Pubkey,
    pub sol_account: Pubkey,//This is the PDA that holds the deposited SOL,storing address lets you -> Validate Withdrawals and ensure the funds belong to the particular collateral state
    pub token_account: Pubkey,//the token which the User is putting as the collateral !
    pub lamport_balance: u64,
    pub amount_minted: u64,
    pub bump: u8,//Bump = to generate PDA for this Collateral account
    pub bump_sol_account: u8,// to generate PDA for this collateral account holding a Sol account which will be PDA right
    pub is_initialized: bool,//is the Collateral State even initialized ?
}