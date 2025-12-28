use crate::utils::ErrorCode;
use anchor_lang::prelude::*;

pub fn get_sqrt_price_from_tick(tick: i32) -> Result<u128> {
        let base_sqrt_price = 1u128 << 96;
        let adjustment_factor = 1_000_000_000/1000;
        let adjusted_price = base_sqrt_price.checked_add_signed((tick as i128) * (adjustment_factor as i128)).ok_or(ErrorCode::ArithmeticOverflow)?;
        Ok(adjusted_price)
}
