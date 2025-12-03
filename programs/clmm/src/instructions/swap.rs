use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use anchor_spl::token::{Token, TokenAccount};
use crate::states::*;
use crate::utils::ErrorCode;
use crate::utils::math::*;

#[derive(accounts)]
#[instructions(amount_in: u64, swap_token_1_for_2: bool, amount_out_minimum: u64)]