use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer, Mint, TokenAccount};

#[derive(Accounts)]
pub struct OpenPosition<'info>{
    pub token_mint_1: Account<'info, Mint>,
    pub token_mint_2: Account<'info, Mint>,

    #[account(
        mut,
        has_one = token_mint_1,
        has_one = token_mint_2
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init_if_needed,
        payer = payer,
        space = TickArray::Space,
        seeds = [
            b"tick_array",
            pool.key().as_ref(),
            &tick_array_lower_start_index.to_le_bytes()
        ],
        bump
    )]
    pub lower_tick_array: Account<'info, TickArray>,

    #[account(
        init_if_needed,
        payer = payer,
        space = TickArray::SPACE,
        seeds = [
            b"tick_array",
            pool.key().as_ref(),
            &tick_array_upper_start_index.to_le_bytes()
        ],
        bumps       
    )]
    pub upper_tick_array: Account<'info, TickArray>,

    #[account(
        init_if_needed,
        payer = payer,
        space = Position::SPACE,
        seeds = [
            b"position",
            owner.key().as_ref(),
            pool.key().as_ref(),
            &lower_tick.to_le_bytes(),
            &upper_tick.to_le_bytes(),
        ],
        bump,
    )]
    pub position: Account<'info, Position>,

    #[account(mut)]
    pub user_token_1: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_2: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_token_1: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_token_2: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program : Program<'info, System>,

    pub token_program : Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

impl<'info> OpenPosition<'info>{
    pub fn open_position(
        owner: Pubkey,
        lower_tick: i32,
        upper_tick: i32,
        liquidity_amount: u128,
        _tick_array_lower_start_index: i32,
        _tick_array_upper_start_index: i32,
    ) -> Result <(u64, u64)> {
        let pool = &mut self.pool.key();
        let position = &mut self.position.key();

        require!(liquidity_amount > 0, ErrorCode::InsufficientInputAmount);

        let lower_tick_array = &mut self.lower_tick_array;
        let upper_tick_array = &mut self.upper_tick_array;

        
    }
}