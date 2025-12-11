use anchor_lang::prelude::*;

#[error_code]
pub enum PhaseError {
    #[msg("Lock phase duration is below minimum required period")]
    LockPhaseTooShort,

    #[msg("Lock phase start time is too soon (insufficient fundraising period)")]
    LockPhaseStartsTooSoon,

    #[msg("Lock phase start time is in the past")]
    LockPhaseStartInPast,

    #[msg("Lock phase duration exceeds maximum allowed period")]
    LockPhaseTooLong,

    #[msg("Lock phase has already started")]
    LockPhaseAlreadyStarted,

    #[msg("This Operation Cannot be performed in the current phase")]
    InvalidPhase,

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
}
