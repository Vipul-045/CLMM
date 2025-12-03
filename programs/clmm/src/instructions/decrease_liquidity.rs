use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::states::*;
use crate::utils::ErrorCode;
use crate::utils::math::*;

#[derive(Accounts)]
pub struct DecreaseLiquidity <'info>{

    pub payer : Signer<'info>,

    pub token_mint_1 : Account<'info, Mint>,

    pub token_mint_2 : Account<'info>,

    #[account(
        mut,
        has_one = token_mint_1,
        has_one = token_mint_2
    )]
    pub pool : Account<'info, Pool>,

    #[account()]
    pub lower_tick_array : Account<'info, TickArray>,

    #[account()]
    pub upper_tick_array : Account<'info, TickArray>,

    #[account(
        constraint = position.pool == pool.key() @ ErrorCode::InvalidPositionOwner,
        constraint = position.owner == payer.key() @ ErrorCode::InvalidPositionOwner,
    )]
    pub position: Account<'info, Position>,

    #[account(
        mut,
        tokin_mint: token_mint_1
    )]
    pub user_token_1: Account<'info, TokenAccount>,

    #[account(
        mut,
        token_mint: token_mint_2
    )]
    pub user_token_2: Account<'info, TokenAccount>,

    #[account(
        mut,
        token_mint: token_mint_1,
    )]
    pub pool_token_1: Account<'info, TokenAccount>,

    #[account(
        mut,
        token_mint: token_mint_2
    )]
    pub pool_token_2: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    
    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>

}

