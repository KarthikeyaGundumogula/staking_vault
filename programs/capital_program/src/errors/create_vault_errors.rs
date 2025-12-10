use anchor_lang::prelude::*;

#[error_code]
pub enum CreateVaultError {
    #[msg("Total basis points allocation exceeds maximum (10000 BPS = 100%)")]
    BPSExceedsMaximum,

    #[msg("Slash basis points exceeds maximum allowed")]
    SlashBPSExceedsMaximum,

    #[msg("Number of beneficiaries does not match number of share allocations")]
    BeneficiaryMismatch,

    #[msg("Too many beneficiaries specified (exceeds maximum limit)")]
    TooManyBeneficiaries,

    #[msg("Duplicate beneficiary address detected")]
    DuplicateBeneficiary,

    #[msg("Invalid basis points configuration")]
    InvalidBasisPoints,

    #[msg("Beneficiary shares must sum to allocated BPS")]
    BeneficiarySharesMismatch,

    #[msg("Individual beneficiary share exceeds maximum")]
    BeneficiaryShareTooHigh,

    #[msg("Investor basis points must be greater than zero")]
    InvestorBPSTooLow,

    #[msg("Maximum cap must be greater than minimum cap")]
    InvalidCapitalRange,

    #[msg("Minimum cap must be greater than zero")]
    MinCapMustBePositive,

    #[msg("Minimum lock amount must be greater than zero")]
    MinLockAmountMustBePositive,

    #[msg("Maximum cap is unreasonably high")]
    MaxCapTooHigh,

    #[msg("Minimum lock amount exceeds minimum cap")]
    MinLockExceedsMinCap,

    #[msg("Capital range is too narrow for practical use")]
    CapitalRangeTooNarrow,

    #[msg("Invalid capital configuration")]
    InvalidCapitalConfig,

    #[msg("Beneficiary shares must greater than zero")]
    BeneficiaryShareMustBePositive,

    #[msg("Lock phase duration is below minimum required period")]
    LockPhaseTooShort,

    #[msg("Lock phase start time is too soon (insufficient fundraising period)")]
    LockPhaseStartsTooSoon,

    #[msg("Lock phase start time is in the past")]
    LockPhaseStartInPast,

    #[msg("Lock phase duration exceeds maximum allowed period")]
    LockPhaseTooLong,

    #[msg("Invalid reward token mint")]
    InvalidRewardMint,

    #[msg("Invalid staking token mint")]
    InvalidStakingMint,

    #[msg("NFT collection already exists")]
    CollectionAlreadyExists,

    #[msg("Invalid MPL Core program")]
    InvalidMplCoreProgram,

    #[msg("CPI call failed")]
    CPIFailed,

    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,

    #[msg("Arithmetic underflow occurred")]
    ArithmeticUnderflow,
}
