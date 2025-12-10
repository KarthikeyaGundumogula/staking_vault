use crate::constants::BASE_BPS;
use crate::state::Vault;
use anchor_lang::prelude::*;

use crate::errors::SlashRequestError;

use crate::state::AuthorityConfig;

#[derive(Accounts)]
pub struct CreateSlashReq<'info> {
    #[account(
        mut,
        address = config.agent @ SlashRequestError::UnauthorizedAgent
    )]
    pub agent: Signer<'info>,
    #[account(
        mut,
        seeds = [b"Vault", vault.node_operator.key().as_ref()],
        bump = vault.bump,
        constraint = !vault.is_dispute_active @ SlashRequestError::VaultUnderDispute
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        seeds = [b"Config"],
        bump = config.bump,
    )]
    pub config: Account<'info, AuthorityConfig>,
}

impl<'info> CreateSlashReq<'info> {
    pub fn create_slas_req(&mut self, slash_bps: u16) -> Result<()> {
        require_gte!(
            self.vault.max_slash_bps,
            slash_bps,
            SlashRequestError::SlashReqExceedsMaxBps
        );
        let slash_amount = (slash_bps as u64)
            .checked_mul(self.vault.total_capital_collected)
            .ok_or(SlashRequestError::ArithmeticOverflow)?
            .checked_div(BASE_BPS as u64)
            .ok_or(SlashRequestError::ArithmeticOverflow)?;
        self.vault.is_dispute_active = true;
        self.vault.pending_slash_amount = slash_amount;
        let clock = Clock::get()?;
        self.vault.dispute_start_time = clock.unix_timestamp;
        Ok(())
    }
}
