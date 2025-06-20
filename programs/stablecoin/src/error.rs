use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Price")]
    InvalidPrice,

    #[msg("Below Minmum Health Factor")]
    BelowMinHealthFactor,

    #[msg("Cannot liquidate a Healthy Account")]
    AboveMinHealthFactor,

     #[msg("Math Overflow")]
    MathOverflow

}