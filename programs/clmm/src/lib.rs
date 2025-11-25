use anchor_lang::prelude::*;

declare_id!("4YfgeshfgHDeURYNvZH77W4yF8554oenSHAzaAQEmedo");

#[program]
pub mod clmm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
