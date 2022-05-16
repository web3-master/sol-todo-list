use anchor_lang::prelude::*;

#[error_code]
pub enum TodoListError {
    #[msg("This list is full")]
    ListFull,
}