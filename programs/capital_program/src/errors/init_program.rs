use anchor_lang::prelude::*;

#[error_code]
pub enum InitError {
    // ============================================================================
    // Authority & Address Errors (6000-6009)
    // ============================================================================
    #[msg("Invalid agent address provided")]
    InvalidAgentAddress,

    #[msg("Agent address cannot be the system program")]
    AgentCannotBeSystemProgram,

    #[msg("Admin address cannot be the system program")]
    AdminCannotBeSystemProgram,

    #[msg("Invalid administrator address")]
    InvalidAdminAddress,

    #[msg("Unauthorized: only admin can initialize")]
    UnauthorizedInitializer,

    #[msg("Configuration already initialized")]
    AlreadyInitialized,

    // ============================================================================
    // Fee Configuration Errors (6010-6019)
    // ============================================================================
    #[msg("Early unlock fee exceeds maximum allowed value")]
    EarlyUnlockFeeTooHigh,

    #[msg("Early unlock fee is below minimum allowed value")]
    EarlyUnlockFeeTooLow,

    #[msg("Invalid early unlock fee configuration")]
    InvalidEarlyUnlockFee,

    #[msg("Fee must be specified in valid units")]
    InvalidFeeUnits,

    // ============================================================================
    // Dispute Window Errors (6020-6029)
    // ============================================================================
    #[msg("Dispute window duration is too short")]
    DisputeWindowTooShort,

    #[msg("Dispute window duration exceeds maximum allowed")]
    DisputeWindowTooLong,

    #[msg("Invalid dispute window configuration")]
    InvalidDisputeWindow,

    #[msg("Dispute window must be positive")]
    DisputeWindowMustBePositive,

    // ============================================================================
    // Lock Duration Errors (6030-6039)
    // ============================================================================
    #[msg("Minimum lock duration must be greater than zero")]
    MinLockDurationMustBePositive,

    #[msg("Maximum lock duration must be greater than minimum")]
    MaxLockDurationTooSmall,

    #[msg("Minimum lock duration is below allowed threshold")]
    MinLockDurationTooShort,

    #[msg("Maximum lock duration exceeds allowed threshold")]
    MaxLockDurationTooLong,

    #[msg("Lock duration range is too narrow for practical use")]
    LockDurationRangeTooNarrow,

    #[msg("Invalid lock duration configuration")]
    InvalidLockDuration,

    #[msg("Lock duration constraints are contradictory")]
    ConflictingLockDurations,

    // ============================================================================
    // NFT Program Errors (6040-6049)
    // ============================================================================
    #[msg("Invalid NFT program address")]
    InvalidNFTProgram,

    #[msg("NFT configuration is already initialized")]
    NFTConfigAlreadyInitialized,

    #[msg("Invalid NFT configuration owner")]
    InvalidNFTConfigOwner,

    #[msg("NFT program is not executable")]
    NFTProgramNotExecutable,

    #[msg("NFT configuration account mismatch")]
    NFTConfigMismatch,

    #[msg("Invalid NFT program configuration")]
    InvalidNFTConfig,

    // ============================================================================
    // Capital Program Errors (6050-6059)
    // ============================================================================
    #[msg("Invalid capital program ID")]
    InvalidCapitalProgramId,

    #[msg("Capital program cannot be the system program")]
    CapitalProgramCannotBeSystemProgram,

    #[msg("Capital program is not executable")]
    CapitalProgramNotExecutable,

    #[msg("Capital program already registered")]
    CapitalProgramAlreadyRegistered,

    // ============================================================================
    // CPI & Program Interaction Errors (6060-6069)
    // ============================================================================
    #[msg("Cross-program invocation failed")]
    CPIFailed,

    #[msg("CPI call failed (deprecated - use CPIFailed)")]
    CPIFail,

    #[msg("Invalid CPI signer seeds")]
    InvalidCPISeeds,

    #[msg("CPI account validation failed")]
    CPIAccountValidationFailed,

    #[msg("Insufficient accounts for CPI")]
    InsufficientCPIAccounts,

    // ============================================================================
    // Arithmetic Errors (6070-6079)
    // ============================================================================
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,

    #[msg("Arithmetic underflow occurred")]
    ArithmeticUnderflow,

    #[msg("Division by zero attempted")]
    DivisionByZero,

    #[msg("Invalid arithmetic operation")]
    InvalidArithmeticOperation,

    // ============================================================================
    // Configuration Validation Errors (6080-6089)
    // ============================================================================
    #[msg("Invalid program configuration")]
    InvalidConfiguration,

    #[msg("Configuration parameter out of valid range")]
    ConfigParameterOutOfRange,

    #[msg("Missing required configuration parameter")]
    MissingConfigParameter,

    #[msg("Conflicting configuration parameters")]
    ConflictingParameters,

    #[msg("Configuration validation failed")]
    ConfigValidationFailed,

    #[msg("Invalid configuration state")]
    InvalidConfigState,

    // ============================================================================
    // Account & PDA Errors (6090-6099)
    // ============================================================================
    #[msg("Invalid PDA derivation")]
    InvalidPDADerivation,

    #[msg("Account already exists")]
    AccountAlreadyExists,

    #[msg("Invalid account owner")]
    InvalidAccountOwner,

    #[msg("Account not initialized")]
    AccountNotInitialized,

    #[msg("Invalid account data")]
    InvalidAccountData,

    #[msg("Account size mismatch")]
    AccountSizeMismatch,
}