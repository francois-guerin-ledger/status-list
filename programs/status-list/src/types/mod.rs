use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, Debug, Eq, PartialEq)]
pub enum ListPurpose {
    Revocation,
    Suspension,
}
