use anchor_lang::prelude::*;

#[account()]
#[derive(InitSpace)]
pub struct Pool{

    pub token_mint_1: Pubkey,
    pub token_mint_2: Pubkey,
    pub token_vault_1: Pubkey,
    pub token_vault_2: Pubkey,
    pub global_liquidity: u128,
    pub sqrt_price_x96: u128,
    pub current_tick: i32,
    pub tick_spacing: i32,
    pub bump: u8
}

impl Pool {

    pub const SPACE:usize = 8 + 32 + 32 + 32 + 32 + 16 + 16 + 4 + 4 + 1;
}