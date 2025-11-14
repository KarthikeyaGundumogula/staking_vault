use anchor_lang::prelude::*;
use mpl_core::{instructions::CreateV2CpiBuilder, ID as MPL_CORE_ID};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateAssetArgs {
    pub name: String,
    pub uri: String,
}

#[derive(Accounts)]
pub struct CreateAsset<'info> {
    #[account(mut)]
    pub asset: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: this account will be checked at the calling program
    pub owner: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_ID)]
    /// CHECK: this account is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> CreateAsset<'info> {
    pub fn create_asset(&mut self, args: CreateAssetArgs) -> Result<()> {
        let owner = match &self.owner {
            Some(owner) => Some(owner.to_account_info()),
            None => None,
        };
        msg!("asset minted");

        CreateV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(None)
            .authority(Some(&self.payer.to_account_info()))
            .payer(&self.payer.to_account_info())
            .owner(owner.as_ref())
            .update_authority(owner.as_ref())
            .system_program(&self.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .invoke()?;
        Ok(())
    }
}
