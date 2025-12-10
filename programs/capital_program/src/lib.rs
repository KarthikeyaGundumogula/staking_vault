use anchor_lang::prelude::*;

declare_id!("DW9BXusirecGep9k5FXFDALYiY1HPtBpVWwPJ36ZD8KZ");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;
use errors::*;

#[program]
pub mod capital_program {


    use super::*;

    pub fn init_program(ctx: Context<InitProgram>, params: InitProgramConfig) -> Result<()> {
         // Step 1: Validate all parameters
        ctx.accounts.validate_params(&params)?;

        // Step 2: Initialize config account
        ctx.accounts.initialize_config(params.clone(), &ctx.bumps)?;

        // Step 3: Initialize NFT program via CPI
        ctx.accounts.initialize_nft_program(*ctx.program_id)?;

        // Emit initialization event
        emit!(ProgramInitializedEvent {
            config: ctx.accounts.config.key(),
            admin: ctx.accounts.admin.key(),
            agent: params.agent,
            nft_program: ctx.accounts.nft_program.key(),
            capital_program: *ctx.program_id,
            early_unlock_fee: params.early_unlock_fee,
            dispute_window: params.dispute_window,
            min_lock_duration: params.min_lock_duration,
            max_lock_duration: params.max_lock_duration,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Program initialized successfully");
        msg!("Config: {}", ctx.accounts.config.key());
        msg!("Admin: {}", ctx.accounts.admin.key());
        msg!("Agent: {}", params.agent);
        Ok(())
    }

    pub fn create_vault(ctx: Context<CreateVault>, config: InitVaultConfig) -> Result<()> {
        
        // Step 1: Validate configuration parameters
        ctx.accounts.validate_config(&config)?;

        // Step 2: Initialize vault account
        ctx.accounts.initialize_vault(config.clone(), &ctx.bumps)?;

        // Step 3: Create NFT collection via CPI
        ctx.accounts.create_nft_collection()?;

        // Emit event for indexing
        emit!(VaultCreatedEvent {
            vault: ctx.accounts.vault.key(),
            provider: ctx.accounts.provider.key(),
            node_operator: config.node_operator,
            staking_token: ctx.accounts.staking_token_mint.key(),
            reward_token: ctx.accounts.reward_token_mint.key(),
            nft_collection: ctx.accounts.nft_collection.key(),
            min_cap: config.min_cap,
            max_cap: config.max_cap,
            lock_phase_start_time: config.lock_phase_start_time,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Vault created successfully: {}", ctx.accounts.vault.key());

        Ok(())
    }

    pub fn open_position(ctx: Context<OpenPosition>, amount: u64) -> Result<()> {
       

        // Step 1: Initialize position account
        ctx.accounts.initialize_position(amount, &ctx.bumps)?;

        // Step 2: Transfer capital to vault
        ctx.accounts.transfer_capital(amount)?;

        // Step 3: Mint NFT representing the position
        ctx.accounts.mint_position_nft()?;

        // Emit event for indexing
        emit!(PositionOpenedEvent {
            position: ctx.accounts.position.key(),
            vault: ctx.accounts.vault.key(),
            capital_provider: ctx.accounts.capital_provider.key(),
            asset: ctx.accounts.asset.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Position opened successfully");
        msg!("Position: {}", ctx.accounts.position.key());
        msg!("Amount locked: {}", amount);

        Ok(())
    }

    pub fn update_position(ctx: Context<UpdatePosition>, update_amount: i64) -> Result<()> {
       
        // Process the update
        ctx.accounts.process_update(update_amount)?;

        // Emit event
        emit!(PositionUpdatedEvent {
            position: ctx.accounts.position.key(),
            vault: ctx.accounts.vault.key(),
            capital_provider: ctx.accounts.capital_provider.key(),
            update_amount,
            new_total: ctx.accounts.position.total_value_locked,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Position updated successfully");
        msg!("New total locked: {}", ctx.accounts.position.total_value_locked);

        Ok(())
    }

    pub fn deposit_rewards(ctx: Context<DepositRewards>, amount: u64) -> Result<()> {
        
        // Step 1: Validate deposit parameters
        ctx.accounts.validate_deposit(amount)?;

        // Step 2: Update vault state
        ctx.accounts.update_vault_state(amount)?;

        // Step 3: Transfer reward tokens
        ctx.accounts.transfer_rewards(amount)?;

        // Emit event for indexing
        emit!(RewardsDepositedEvent {
            vault: ctx.accounts.vault.key(),
            agent: ctx.accounts.agent.key(),
            amount,
            total_rewards: ctx.accounts.vault.total_rewards_deposited,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Rewards deposited successfully");
        msg!("Amount: {}", amount);
        msg!("Total rewards in vault: {}", ctx.accounts.vault.total_rewards_deposited);

        Ok(())
    }

    pub fn claim_investor_rewards(ctx:Context<ClaimInvestorRewards>) -> Result<()> {
        // Calculate claimable rewards
        let claimable_amount = ctx.accounts.calculate_claimable_rewards()?;

        // Ensure there are rewards to claim
        require_gt!(claimable_amount, 0, ClaimRewardsError::NoRewardsToClaim);

        // Update position state
        ctx.accounts.process_claim(claimable_amount)?;

        // Transfer rewards to holder
        ctx.accounts.transfer_rewards(claimable_amount)?;

        // Emit event for indexing
        emit!(RewardsClaimedEvent {
            holder: ctx.accounts.holder.key(),
            vault: ctx.accounts.vault.key(),
            position: ctx.accounts.position.key(),
            amount: claimable_amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Successfully claimed {} rewards", claimable_amount);

        Ok(())
    }

    // pub fn unstake(ctx: Context<UnStake>) -> Result<()> {
    //     // let staking_vault = &ctx.accounts.staking_vault;
    //     // let clock = Clock::get()?;
    //     // let elapsed_time = clock.unix_timestamp as u64 - staking_vault.start_time;
    //     // require!(
    //     //     elapsed_time >= staking_vault.duration,
    //     //     StakingVaultError::StakingPeriodNotCompleted
    //     // );
    //     let staked_amount = ctx.accounts.staking_vault_ata.amount;
    //     ctx.accounts.transfer_staked_tokens(staked_amount)?;
    //     let reward_amount = &ctx.accounts.vault_reward_ata.amount;
    //     if *reward_amount > 0 {
    //         ctx.accounts.transfer_rewards(*reward_amount)?;
    //     }
    //     Ok(())
    // }

    // pub fn close_vault(ctx: Context<Close>) -> Result<()> {
    //     let vault_reward_balance = ctx.accounts.vault_rewards_ata.amount;
    //     let vault_staked_balance = ctx.accounts.vault_staking_ata.amount;
    //     require!(
    //         vault_reward_balance == vault_staked_balance && vault_reward_balance == 0,
    //         StakingVaultError::VaultNotEmpty
    //     );
    //     ctx.accounts.close_vault_accounts()?;
    //     ctx.accounts.burn_nft()?;
    //     Ok(())
    // }
}

#[error_code]
pub enum StakingVaultError {
    #[msg("The staking period not completed yet")]
    StakingPeriodNotCompleted,
    #[msg("Vault still holds tokens unstake first")]
    VaultNotEmpty,
}
