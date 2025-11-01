use anchor_lang::prelude::*;

declare_id!("6AD9gckrLi1LxJuS6TJeA4myevWbSGULYKHc3o2mJkzu");

pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod staking_vault {
    use super::*;

    pub fn open(
        ctx: Context<Open>,
        duration: u64,
        min_amount: u64,
        max_amount: u64,
        initial_rewards_deposit: u64,
    ) -> Result<()> {
        ctx.accounts
            .init_config(duration, min_amount, max_amount, ctx.bumps)?;
        ctx.accounts.deposit_rewards(initial_rewards_deposit)?;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        ctx.accounts.stake_tokens(amount)
    }

    pub fn unstake(ctx: Context<UnStake>) -> Result<()> {
        let staking_vault = &ctx.accounts.staking_vault;
        let clock = Clock::get()?;
        let elapsed_time = clock.unix_timestamp as u64 - staking_vault.start_time;
        require!(
            elapsed_time >= staking_vault.duration,
            StakingVaultError::StakingPeriodNotCompleted
        );
        let staked_amount = ctx.accounts.staking_vault_ata.amount;
        ctx.accounts.transfer_staked_tokens(staked_amount)?;
        let reward_amount = ctx.accounts.vault_reward_ata.amount;
        ctx.accounts.transfer_rewards(reward_amount)?;
        Ok(())
    }

    pub fn deposit_rewards(ctx: Context<DepositRewards>,amount:u64) -> Result<()> {
        ctx.accounts.deposit_rewards(amount)?;
        Ok(())
    }

    pub fn increment_stake(ctx: Context<IncrementStake>,amount:u64) -> Result<()>{
        ctx.accounts.deposit_rewards(amount)?;
        Ok(())
    }
}

#[error_code]
pub enum StakingVaultError {
    #[msg("The staking period has not yet been completed.")]
    StakingPeriodNotCompleted,
}
