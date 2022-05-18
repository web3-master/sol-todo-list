use anchor_lang::prelude::*;

#[error_code]
pub enum TodoListError {
    #[msg("This list is full")]
    ListFull,

    #[msg("Specified list owner does not match the pubkey in the list")]
    WrongListOwner,

    #[msg("Bounty must be enough to mark account rent-exempt")]
    BountyTooSmall,

    #[msg("Only list owner or item creator can cancel item")]
    WrongCancelPermission,

    #[msg("Item not found")]
    ItemNotFound,
    
    #[msg("Item creator is not correct")]
    WrongItemCreator
}