use anchor_lang::prelude::*;

#[error_code]
pub enum ArithmeticError {
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,

    #[msg("Arithmetic underflow occurred")]
    ArithmeticUnderflow,

    #[msg("amount -ve for withdraw or +ve for deposit")]
    UpdateAmountCannotBeZero,

    #[msg("Amount must be greater than zero")]
    AmountMustBePositive,

    #[msg("Amount is below minimum required")]
    AmountBelowMinimum,

    #[msg("Division by zero attempted")]
    DivisionByZero,

    #[msg("Invalid arithmetic operation")]
    InvalidArithmeticOperation,

    #[msg("Invalid calculation result")]
    InvalidCalculation,
}
