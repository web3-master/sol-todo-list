use anchor_lang::prelude::*;

#[account]
pub struct ListItem {
    pub creator: Pubkey,
    pub creator_finished: bool,
    pub list_owner_finished: bool,
    pub name: String,
}

impl ListItem {
    pub fn space(name: &str) -> usize {
        8 + 32 + 1 + 1 +
        4 + name.len()
    }
}