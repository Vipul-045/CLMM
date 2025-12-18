use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode{
    #[msg("Insufficient Pool Liquidity")]
    InsufficientPoolLiquidity,
    #[msg("Insufficient Input")]
    InsufficientInput,
    #[msg("Slippage Exceeded")]
    SlippageExceeded,
    #[msg("Invalid Tick Range")]
    InvalidTickRange,
    #[msg("Insufficient Input Amount")]
    InsufficientInputAmount,
    #[msg("Arithmetic Overflow")]
    ArithmeticOverflow,
    #[msg("Invalid Position owner")]
    InvalidPositionOwner,
    #[msg("Invalid Tick Spacing")]
    InvalidTickSpacing,
    #[msg("Invalid Token Pair")]
    InvalidTokenPair,
}