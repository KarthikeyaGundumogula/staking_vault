use anchor_lang::prelude::*;

#[error_code]
pub enum PositionError {
    #[msg("The signer is not the owner of the asset")]
    InvalidAssetOwner,

    #[msg("Invalid asset address")]
    InvalidAsset,
    #[msg("Vault is not active")]
    VaultInactive,

    #[msg("Vault is under dispute")]
    VaultUnderDispute,

    #[msg("Vault has reached maximum capacity")]
    VaultMaxCapReached,
    #[msg("Position does not belong to this vault")]
    PositionVaultMismatch,

    UpdateAmountCannotBeZero,

    #[msg("Amount must be greater than zero")]
    AmountMustBePositive,

    #[msg("Amount is below minimum required")]
    AmountBelowMinimum,

    #[msg("Insufficient balance in account")]
    InsufficientBalance,

    #[msg("Insufficient balance in vault")]
    InsufficientVaultBalance,

    #[msg("Lock phase has already started")]
    LockPhaseAlreadyStarted,

    #[msg("Invalid locking token mint")]
    InvalidLockingMint,

    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,

    #[msg("Invalid collection address")]
    InvalidCollection,

    #[msg("Amount exceeds vault capacity")]
    AmountExceedsVaultCapacity,

    #[msg("Invalid MPL Core program")]
    InvalidMplCoreProgram,

    #[msg("Arithmetic underflow occurred")]
    ArithmeticUnderflow,
}
