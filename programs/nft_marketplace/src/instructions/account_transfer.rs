use crate::state::*;
use anchor_lang::prelude::*;
use mpl_core::{instructions::TransferV1CpiBuilder, ID as MPL_CORE_ID};

#[derive(Accounts)]
pub struct TransferAsset<'info> {
    /// CHECK: This is just for look-up
    #[account(mut)]
    pub asset: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8+ Vault::INIT_SPACE,
        seeds = [b"vault"],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    /// CHECK: This account will be checked by the mpl_core program
    pub collection: Option<UncheckedAccount<'info>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Option<Signer<'info>>,
    /// CHECK: this account will be checked by the mpl_core program
    pub new_owner: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_ID)]
    /// CHECK: this account is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> TransferAsset<'info> {
    pub fn transfer(&mut self) -> Result<()> {
        let collection = match self.collection.as_ref() {
            Some(collection) => Some(collection.to_account_info()),
            None => None,
        };
        let authority = match self.authority.as_ref() {
            Some(authority) => Some(authority.to_account_info()),
            None => None,
        };

        TransferV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(collection.as_ref())
            .authority(authority.as_ref())
            .new_owner(&self.new_owner.to_account_info())
            .system_program(Some(&self.system_program.to_account_info()))
            .payer(&self.payer.to_account_info())
            .invoke()?;
        Ok(())
    }
}
