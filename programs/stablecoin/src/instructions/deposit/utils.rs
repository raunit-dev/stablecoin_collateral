use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{token_2022::{mint_to, MintTo, Token2022}, token_interface::{Mint, TokenAccount}};
use crate::SEED_MINT_ACCOUNT;

pub fn mint_token <'info> (
    mint_account: &InterfaceAccount<'info, Mint>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    token_program: &Program<'info, Token2022>,
    amount: u64,
    bump: u8,
) -> Result<()> {

    let signer_seeds: &[&[&[u8]]] = &[&[SEED_MINT_ACCOUNT, &[bump]]];

    // let cpi_program = token_program.to_account_info();
    // let mint_accounts = MintTo {
    //     mint: mint_account.to_account_info(),
    //     to: token_account.to_account_info(),
    //     authority: mint_account.to_account_info()
    // };

    // let cpi_ctx = CpiContext::new_with_signer(cpi_program, mint_accounts, signer_seeds);
    // mint_to(cpi_ctx, amount)

    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: mint_account.to_account_info(),
                to: token_account.to_account_info(),
                authority: mint_account.to_account_info()
            },
            signer_seeds
        ), 
        amount
    )

}

pub fn deposit_sol <'info> (
    from: &Signer<'info>,
    to: &SystemAccount<'info>,  
    system_program: &Program<'info, System>,
    amount: u64
) -> Result<()> {

    // let cpi_program = system_program.to_account_info();
    // let transfer_accounts = Transfer {
    //     from: from.to_account_info(),
    //     to: to.to_account_info()
    // };
    // let cpi_ctx = CpiContext::new(cpi_program, transfer_accounts);
    // transfer(cpi_ctx,amount);

    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: from.to_account_info(),
                to: to.to_account_info()
            }
        ),
        amount,
    )

}