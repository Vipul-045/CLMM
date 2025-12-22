pub mod instructions;
pub mod states;

use anchor_lang::prelude::*;
use crate::instructions::*;

declare_id!("4YfgeshfgHDeURYNvZH77W4yF8554oenSHAzaAQEmedo");

#[program]
pub mod clmm {
    use crate::instructions::open_position::OpenPosition;

    use super::*;

    pub fn initializepool(ctx: Context<InitializePool>,
        tick_spacing: i32,
        initial_sqrt_price: u128) -> Result<()> {
        instructions::pool::init_pool(ctx, tick_spacing, initial_sqrt_price)
    }

    pub fn open_position(
        ctx: Context<OpenPosition>,
        owner: Pubkey,
        lower_tick: i32,
        upper_tick: i32,
        liquidity_amount: u128,
        _tick_array_lower_start_index: i32,
        _tick_array_upper_start_index: i32) -> Result <(u64, u64)> {
                instructions::open_position::open_position(ctx, owner, lower_tick, upper_tick, liquidity_amount, tick_array_lower_start_index, tick_array_upper_start_index)
    }


}
