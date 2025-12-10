#[allow(deprecated)]
use anchor_lang::prelude::*;
declare_id!("3kLob38A4tG8m3fP9ZZwSWsjdr417DjQZ4bkqxGFjaUh");

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod nft_program {

    use super::*;

    pub fn initialize_program(ctx: Context<InitNFTProgram>, capital_program: Pubkey) -> Result<()> {
        ctx.accounts.initialize(ctx.bumps, capital_program)?;
        Ok(())
    }

    pub fn create_vault_collection(ctx: Context<CreateVaultCollection>) -> Result<()> {
        ctx.accounts.create_collection()?;
        Ok(())
    }

    pub fn create_core_asset(ctx: Context<CreateAsset>, args: CreateAssetArgs) -> Result<()> {
        ctx.accounts.create_asset(args)?;
        Ok(())
    }

    pub fn deposit_asset(ctx: Context<DepositAsset>) -> Result<()> {
        ctx.accounts.deposit(ctx.bumps)?;
        Ok(())
    }

    pub fn claim_asset(ctx: Context<ClaimAsset>) -> Result<()> {
        ctx.accounts.claim()?;
        Ok(())
    }

    pub fn burn_asset(ctx: Context<BurnAsset>) -> Result<()> {
        ctx.accounts.burn()?;
        Ok(())
    }
}

#[error_code]
pub enum VaultError {
    #[msg("Vault should be the owner of NFT")]
    InvalidOwner,
}
