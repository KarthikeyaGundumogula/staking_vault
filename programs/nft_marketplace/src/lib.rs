use anchor_lang::prelude::*;
declare_id!("GtngBj7fzxga8jZxRSFZZYagKULxLQaWgne4a4a8NZgi");

pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod nft_marketplace {

    use super::*;

    pub fn create_core_asset(ctx: Context<CreateAsset>, args: CreateAssetArgs) -> Result<()> {
        ctx.accounts.create_asset(args)?;
        Ok(())
    }

    pub fn deposit_asset(ctx: Context<DepositAsset>, receiver: Pubkey) -> Result<()> {
        ctx.accounts.deposit(receiver, ctx.bumps)?;
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
