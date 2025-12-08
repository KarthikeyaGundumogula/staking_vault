use crate::state::{Vault,Config};
use crate::errors::CreateVaultError;
use crate::constants::*;
use nft_marketplace::program::NftMarketplace;
use nft_marketplace::cpi::accounts::CreateVaultCollection;
use nft_marketplace::state::Config as NftConfig;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ Mint, TokenInterface},
};

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub provider: Signer<'info>,
    #[account(
      init,
      payer = provider,
      seeds = [b"staking_vault",provider.key().as_ref()],
      space = Vault::INIT_SPACE + 8,
      bump 
    )]
    pub staking_vault: Account<'info, Vault>,
    #[account(
        seeds = [b"Config"],
        bump = config.bump
    )]
    pub config: Account<'info,Config>,
    pub nft_config: Account<'info,NftConfig>,
    #[account(mint::token_program = token_program)]
    pub reward_token_mint: InterfaceAccount<'info, Mint>,
    pub staking_token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub nft_collection: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: this account will be checked my the mpl_core_program
    pub mpl_core_program: UncheckedAccount<'info>,
    pub nft_marketplace: Program<'info, NftMarketplace>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateVault<'info> {
    pub fn validate_params(&self,config:&InitConfig) -> Result<()> {
        let total_bps: u16 = config.beneficiary_shares_bps.iter().sum::<u16>() + config.investor_bps;
        require!(total_bps <= BASE_BPS,CreateVaultError::InvalidConfig);
        require!(config.lock_phase_duration > MIN_LOCK_PERIOD,CreateVaultError::InvalidConfig);
        require!(config.min_cap>config.max_cap,CreateVaultError::InvalidConfig);
        let clock = Clock::get()?;
        require!(clock.unix_timestamp+MIN_FUND_RAISE_DURATION < config.lock_phase_start_time,CreateVaultError::TooEarlyToLock);
        Ok(())
    }

    pub fn init_config(&mut self, config: InitConfig, bumps: CreateVaultBumps) -> Result<()> {
        self.staking_vault.set_inner(Vault{
            locking_token_mint: self.staking_token_mint.key(),
            reward_token_mint:self.reward_token_mint.key(),
            min_cap:config.min_cap,
            max_cap:config.max_cap,
            total_capital_collected: 0,
            total_rewards_deposited:0,
            beneficiaries:config.beneficiaries, // TODO: Update this to optional value 
            beneficiary_shares_bps:config.beneficiary_shares_bps, // TODO: Update this to optional value 
            investor_bps:config.investor_bps,
            max_slash_bps:config.max_slash_bps,
            nft_collection:self.nft_collection.key(),
            reward_distributor:self.provider.key(), // TODO: update this to Optional value if none then the provider is the reward_distributor
            node_operator:*self.provider.key,
            lock_phase_start_at:config.lock_phase_start_time,
            lock_phase_duration:config.lock_phase_duration,
            is_dispute_active:false,
            dispute_start_time:0,
            pending_slash_amount:0,
            slash_claimant:*self.provider.key, // TODO: update this to agent 
            bump:bumps.staking_vault
        });
        Ok(())
    }

    pub fn create_collection(&mut self) -> Result<()>{
        let create_collection_accounts = CreateVaultCollection{
            collection:self.nft_collection.to_account_info(),
            update_authority: self.config.to_account_info(),
            config:self.nft_config.to_account_info(),
            payer:self.provider.to_account_info(),
            mpl_core_program:self.mpl_core_program.to_account_info(),
            system_program:self.system_program.to_account_info()
        };
        let signer_seeds: &[&[&[u8]]] = &[&[b"Config", &[self.config.bump]]];

        let create_collection_ctx = CpiContext::new_with_signer(self.nft_marketplace.to_account_info(), create_collection_accounts, signer_seeds);
        nft_marketplace::cpi::create_vault_collection(create_collection_ctx).map_err(|_| error!(CreateVaultError::CPIFail))?;
        Ok(())
    }

    // pub fn mint_asset(&mut self) -> Result<()> {
    //     // let mint_asset_accounts = CreateAsset {
    //     //     asset: self.asset.to_account_info(),
    //     //     payer: self.provider.to_account_info(),
    //     //     owner: Some(self.staking_vault.to_account_info()),
    //     //     system_program: self.system_program.to_account_info(),
    //     //     mpl_core_program: self.mpl_core_program.to_account_info(),
    //     //     collection:self.collection,
    //     // };

    //     // let mint_cpi = CpiContext::new(
    //     //     self.nft_marketplace.to_account_info(),
    //     //     mint_asset_accounts
    //     // );

    //     // let args = CreateAssetArgs {
    //     //     name: String::from("Vault NFT"),
    //     //     uri: String::from("MINT_URI"),
    //     // };

    //     // // call the CPI — now the staking_vault PDA will correctly be treated as a 'signer'
    //     // nft_marketplace::cpi::create_core_asset(mint_cpi, args)
    //     //     .map_err(|_| error!(StakingError::CPIFail))?;

    //     Ok(())
    // }
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitConfig {
    pub min_cap: u64,
    pub max_cap: u64,
    pub beneficiaries: Vec<Pubkey>,
    pub beneficiary_shares_bps: Vec<u16>,
    pub investor_bps: u16,
    pub max_slash_bps: u16,
    pub reward_distributor: Pubkey,
    pub node_operator: Pubkey,
    pub lock_phase_duration: i64,
    pub lock_phase_start_time: i64,
    pub slash_claimant: Pubkey,
}
