use anchor_lang::prelude::*;

#[account]
pub struct Position { 

    pub liquidity: u128,
    pub tick_lower: u32,
    pub tick_upper: u32, 
    pub owner : Pubkey,
    pub pool : Pubkey,
    pub bump : u8,
}

impl Position {
    pub const SPACE:usize = 8 + 16 + 4 + 4 + 32 + 32 + 1;
}