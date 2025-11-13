use crate::state::*;
use anchor_lang::prelude::*;
use mpl_core::{instructions::TransferV1CpiBuilder, ID as MPL_CORE_ID};

#[derive(Accounts)]
pub struct DepositAsset<'info> {
    /// CHECK: This is just for look-up
    #[account(mut)]
    pub asset: AccountInfo<'info>,
    #[account(
        init,
        payer = payer,
        space = 8+ Vault::INIT_SPACE,
        seeds = [b"vault",new_owner.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: this account will be checked by the mpl_core program
    #[account(address = vault.key())]
    pub new_owner: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_ID)]
    /// CHECK: this account is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> DepositAsset<'info> {
    pub fn deposit(&mut self, receiver: Pubkey, bumps: DepositAssetBumps) -> Result<()> {
        self.vault.bump = bumps.vault;
        self.vault.receiver = receiver;
        
        TransferV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(None)
            .authority(Some(&self.payer.to_account_info()))
            .new_owner(&self.new_owner.to_account_info())
            .system_program(Some(&self.system_program.to_account_info()))
            .payer(&self.payer.to_account_info())
            .invoke()?;
        Ok(())
    }
}
