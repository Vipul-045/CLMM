use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crates::states::*;
use crates::utils::ErrorCode;
use crate::utils::math::*;

#[derive(Accounts)]
#[intruction(tick_spacing: i32)]
pub struct InitializePool <'info> {
    #[account(mut)]
    pub payer: signer<'info>,

    pub token_mint_1: Account<'info, Mint>,
    pub token_mint_2: Account<'info, Mint>,

    #[account(
        mut,
        payer = payer,
        space = pool.SPACE,
        seeds = [
            b"pool".as_ref(),
            token_mint_1.key().as_ref(),
            token_mint_2.key().as_ref(),
            tick_spacing.to_le_bytes()
        ],
        bumps
    )]
    pub pool : Account<'info, Pool>,

    #[account(
        init,
        payer = payer,
        token::mint = token_mint_1,
        token::authority = pool,
    )]
    pub token_vault_1 : Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        token::mint = token_mint_2,
        token::authority = pool,
    )]
    pub token_vault_2 : Account<'info, TokenAccount>,

    pub system_program : Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent : Sysvar<'info, Rent>
}

impl<'info> InitializePool<'info>{
    pub fn init_pool(tick_spacing: i32, initial_sqrt_price: u128) -> Result<()>{
        let pool = &mut ctx.accounts.pool;

        require!(tick_spacing > 0 , ErrorCode::InvalidTickSpacing);

        require!(ctx.accounts.token_mint_1.key() != ctx.accounts.token_mint_2.key() , ErrorCode::InvalidTokenPair);

        pool.token_mint_1 = self.token_mint_1.key();
        pool.token_mint_2 = self.token_mint_2.key();
        pool.token_vault_1 = self.token_vault_1.key();
        pool.token_vault_2 = self.token_vault_2.key();
        pool.global_liquidity = 0;
        pool.sqrt_price_x96 = initial_sqrt_price;
        pool.current_tick = get_tick_at_sqrt_price(initial_sqrt_price)?;
        pool.tick_spacing = tick_spacing;
        pool.bump = self.bumps.pool;

        Ok(())
    }
}