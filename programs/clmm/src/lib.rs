pub mod instructions;
pub mod states;

use anchor_lang::prelude::*;
use crate::instructions::*;

declare_id!("4YfgeshfgHDeURYNvZH77W4yF8554oenSHAzaAQEmedo");

#[program]
pub mod clmm {
    use super::*;

    pub fn initializepool(ctx: Context<InitializePool>,
        tick_spacing: i32,
        initial_sqrt_price: u128) -> Result<()> {
        instructions::pool::init_pool(tick_spacing, initial_sqrt_price)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
