use anchor_lang::prelude::*;

#[error_code]
pub enum StatusListError {
    #[msg("Status list size must be below 512")]
    SizeTooLarge,
    #[msg("Entry out of bounds")]
    OutOfBounds,
    #[msg("Status not reversible")]
    StatusNotReversible,
}
