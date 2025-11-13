use anchor_lang::prelude::*;
use mpl_core::{
    instructions::{CreateV2CpiBuilder, TransferV1CpiBuilder},
    ID as MPL_CORE_ID,
};

declare_id!("GtngBj7fzxga8jZxRSFZZYagKULxLQaWgne4a4a8NZgi");

pub mod state;
pub mod instructions;

use instructions::*;
use state::*;



#[program]
pub mod nft_marketplace {

    use super::*;

    pub fn create_core_asset(ctx: Context<CreateAsset>, args: CreateAssetArgs) -> Result<()> {
        ctx.accounts.create_asset(args)?;
        Ok(())
    }

    pub fn deposit_asset(ctx: Context<TransferAsset>, receiver: Pubkey) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.receiver = receiver;
        require!(
            vault.key() == ctx.accounts.new_owner.key(),
            VaultError::InvalidOwner
        );
        ctx.accounts.transfer()?;
        Ok(())
    }
    
    pub fn claim_asset(ctx: Context<TransferAsset>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(
            vault.receiver == ctx.accounts.new_owner.key(),
            VaultError::InvalidOwner
        );
        ctx.accounts.transfer_pda(ctx.bumps)?;

        Ok(())
    }
}


impl<'info> TransferAsset<'info> {
    
    pub fn transfer_pda(&mut self, bumps: TransferAssetBumps) -> Result<()> {
        let collection = match self.collection.as_ref() {
            Some(collection) => Some(collection.to_account_info()),
            None => None,
        };
        let seeds = &[b"vault" as &[u8], &[bumps.vault]];
        let signers = &[&seeds[..]];

        TransferV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(collection.as_ref())
            .authority(Some(&self.vault.to_account_info()))
            .new_owner(&self.new_owner.to_account_info())
            .system_program(Some(&self.system_program.to_account_info()))
            .payer(&self.payer.to_account_info())
            .invoke_signed(signers)?;
        Ok(())
    }
}

#[error_code]
pub enum VaultError {
    #[msg("Vault should be the owner of NFT")]
    InvalidOwner,
}