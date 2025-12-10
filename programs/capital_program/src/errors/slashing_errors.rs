use anchor_lang::prelude::*;

#[error_code]
pub enum SlashRequestError{
  UnauthorizedAgent,
  VaultUnderDispute,
  SlashReqExceedsMaxBps,
  ArithmeticOverflow,
  ArithmeticUnderflow,
  InvalidLockingMint,
  VaultReachedMinCap,
}