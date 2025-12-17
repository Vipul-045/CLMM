use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use anchor_spl::token::{Token, TokenAccount};
use crate::states::*;
use crate::utils::ErrorCode;
use crate::utils::math::*;

#[derive(accounts)]
#[instructions(amount_in: u64, swap_token_1_for_2: bool, amount_out_minimum: u64)]
pub struct Swap<'info>{
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub user_token_1: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_2: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_token_1: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_token_2: Account<'info, TokenAccount>,

    #[account(
        mut, 
        constraint = tick_array.key() == Pubkey::find_program_address(
            &[
                b"tick_array".as_ref(),
                pool.key().as_ref(),
                &TickArray::get_starting_tick_index(pool.current_tick, pool.tick_spacing).to_le_bytes(),
            ],
            &crate::ID
        ).0 @ ErrorCode::InvalidTickArrayAccount
    )]
    pub tick_array: Account<'info, TickArray>,

    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, rent>

}

pub fn swap(
    ctx: Context<Swap>,
    amount_in: u64,
    swap_token_1_for_2: bool,
    amount_out_minimum: u64,
) -> Result<u64>{
    let pool = &mut ctx.accounts.pool;

    require!(pool.global_liquidity > 0 , ErrorCode::InsuficientPoolLiquidity);
    require!(amount_in > 0, ErrorCode::InsufficientInput);

    let(amount_in_used, amount_out_calculated, new_sqrt_price_x86) = swap_segment(
        pool.sqrt_price_x86,
        pool.global_liquidity,
        amount_in,
        swap_token_1_for_2
    )?;

    require!(amount_out_calculated >= amount_out_minimum, ErrorCode::SlippageExceeded);

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"pool",
        pool.token_mint_1.as_ref(),
        pool.token_mint_2.as_ref(),
        &pool.tick_spacing.to_le_bytes(),
        &[pool.bump],
    ]];

    

}