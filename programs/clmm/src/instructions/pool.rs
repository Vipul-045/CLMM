use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crates::states::*;
use crates::utils::ErrorCode;
use crate::utils::math::*;

#[derive(Accounts)]
#[intruction(tick_spacing: i32)]
pub struct InitializePool <'info> {
    
}