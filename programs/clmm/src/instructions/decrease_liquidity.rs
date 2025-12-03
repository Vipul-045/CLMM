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

pub fn decrease_liquidity(
    ctx: Context<DecreaseLiquidity>,
    liquidity_amount: u128,
    lower_tick: i32,
    upper_tick: i32
    ) -> Result <(u64, u64)> {
        let pool = &mut ctx.accounts.pool;
        let position = &mut ctx.accounts.pool;

        require!(
            lower_tick < upper_tick
            && lower_tick % pool.tick_spacing == 0
            && upper_tick % pool.tick_spacing == 0, ErrorCode: InvalidTickRange
        );
        require!(liquidity_amount > 0 , ErrorCode: InsufficientInputAmount);

        require!(pool.current_tick >= lower_tick && pool.current_tick < upper_tick, ErrorCode: MintRangeMustCoverCurrentPrice);

        let lower_tick_array = &mut ctx.accounts.lower_tick_array;
        let upper_tick_array = &mut ctx.accounts.upper_tick_array;

        let lower_tick_info = lower_tick_array.get_info_tick_mutable(lower_tick, pool.tick_spacing)?;
        let upper_tick_info = lower_tick_array.get_info_tick_mutable(upper_tick, pool.tick_spacing)?;

        lower_tick_info.update_liquidity(liquidity_amount as i128, true)?;
        upper_tick_info.update_liquidity(liquidity_amount as i128, false)?;

        position.liquidity = position.liquidity.checked_sub(liquidity_amount).ok_or(ErrorCode::ArithemeticOverflow)?;

        
 
    }