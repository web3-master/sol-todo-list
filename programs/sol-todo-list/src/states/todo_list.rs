use anchor_lang::prelude::*;

#[account]
pub struct TodoList {
    pub list_owner: Pubkey,
    pub bump: u8,
    pub capacity: u16,
    pub name: String,
    pub lines: Vec<Pubkey>
}

impl TodoList {
    pub fn space(name: &str, capacity: u16) -> usize {
        8 + 32 + 1 + 2 + 
        // name string
        4 + name.len() + 
        // vec of item pubkeys
        4 + (capacity as usize) * std::mem::size_of::<Pubkey>()
    }
}