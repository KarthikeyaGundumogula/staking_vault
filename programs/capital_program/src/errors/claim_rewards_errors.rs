use anchor_lang::prelude::*;

#[error_code]
pub enum ClaimRewardsError {
    #[msg("The signer is not the owner of the asset")]
    InvalidAssetOwner,

    #[msg("The vault is currently under dispute and claims are disabled")]
    VaultUnderDispute,

    #[msg("The position does not belong to this vault")]
    PositionVaultMismatch,

    #[msg("Invalid asset address")]
    InvalidAsset,

    #[msg("Invalid reward mint address")]
    InvalidRewardMint,

    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,

    #[msg("Arithmetic underflow occurred")]
    ArithmeticUnderflow,

    #[msg("The vault has no Reward collected")]
    NoRewardsInVault,

    #[msg("The position has no capital locked")]
    NoCapitalLocked,

    #[msg("Cannot claim zero rewards")]
    ZeroClaimAmount,

    #[msg("Insufficient balance in vault to process claim")]
    InsufficientVaultBalance,

    #[msg("No rewards available to claim")]
    NoRewardsToClaim,

    #[msg("Invalid calculation result")]
    InvalidCalculation,

    #[msg("Given Index is out of Beneficiary")]
    InvalidBeneficiaryIndex,

    #[msg("Beneficiary not found in the Beneficary Array")]
    UnauthorizedBeneficiary
}
