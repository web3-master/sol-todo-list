mod instructions;
mod states;
mod error;

use anchor_lang::prelude::*;
use instructions::create_list::*;
use instructions::add::*;
use instructions::cancel::*;
use instructions::finish::*;

declare_id!("5V9m3pCcyrWWVsQftZdWEP4GQVLfivAioqawUqjpJzDF");

#[program]
pub mod sol_todo_list {
    use super::*;

    pub fn create_list(ctx: Context<CreateList>, name: String, capacity: u16, account_bump: u8) -> Result<()> {
        instructions::create_list::create_list(ctx, name, capacity, account_bump)
    }

    pub fn add(ctx: Context<Add>, list_name: String, item_name: String, bounty: u64) -> Result<()> {
        instructions::add::add(ctx, list_name, item_name, bounty)
    }

    pub fn cancel(ctx: Context<Cancel>, list_name: String) -> Result<()> {
        instructions::cancel::cancel(ctx, list_name)
    }

    pub fn finish(ctx: Context<Finish>, list_name: String) -> Result<()> {
        instructions::finish::finish(ctx, list_name)
    }
}
