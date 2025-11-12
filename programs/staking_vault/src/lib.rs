use anchor_lang::prelude::*;

declare_id!("6AD9gckrLi1LxJuS6TJeA4myevWbSGULYKHc3o2mJkzu");

pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod staking_vault {
    use super::*;

    pub fn open(ctx: Context<Open>, config: InitConfig) -> Result<()> {
        ctx.accounts.init_config(config, ctx.bumps)?;
        ctx.accounts.deposit_rewards()?;
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

    pub fn deposit_rewards(ctx: Context<DepositRewards>, amount: u64) -> Result<()> {
        ctx.accounts.deposit_rewards(amount)?;
        Ok(())
    }

    pub fn increment_stake(ctx: Context<IncrementStake>, amount: u64) -> Result<()> {
        ctx.accounts.deposit_rewards(amount)?;
        Ok(())
    }

    pub fn close_vault(ctx: Context<Close>) -> Result<()> {
        let vault_reward_balance = ctx.accounts.vault_rewards_ata.amount;
        let vault_staked_balance = ctx.accounts.vault_staking_ata.amount;
        require!(
            vault_reward_balance == vault_staked_balance && vault_reward_balance == 0,
            StakingVaultError::VaultNotEmpty
        );
        ctx.accounts.close_vault_accounts()?;
        Ok(())
    }
}

#[error_code]
pub enum StakingVaultError {
    #[msg("The staking period has not yet been completed.")]
    StakingPeriodNotCompleted,
    #[msg(" vault still holds tokens")]
    VaultNotEmpty,
}
