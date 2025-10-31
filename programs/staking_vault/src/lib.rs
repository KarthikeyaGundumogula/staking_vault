use anchor_lang::prelude::*;

declare_id!("6AD9gckrLi1LxJuS6TJeA4myevWbSGULYKHc3o2mJkzu");

#[program]
pub mod staking_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
