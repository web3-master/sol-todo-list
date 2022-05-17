use anchor_lang::prelude::*;
use crate::states::todo_list::TodoList;

pub fn create_list(ctx: Context<CreateList>, name: String, capacity: u16, account_bump: u8) -> Result<()> {
    let list = &mut ctx.accounts.list;
    list.list_owner = ctx.accounts.user.key();
    list.bump = account_bump;
    list.name = name;
    list.capacity = capacity;
    return Ok(());
}

#[derive(Accounts)]
#[instruction(name: String, capacity: u16)]
pub struct CreateList<'info> {
    #[account(
        init, 
        payer = user, 
        space = TodoList::space(&name, capacity),
        seeds = [
            b"todolist",
            user.key.as_ref(),
            name_seed(&name)
        ],
        bump
    )]
    pub list: Account<'info, TodoList>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn name_seed(name: &str) -> &[u8] {
    let b = name.as_bytes();
    if b.len() > 32 {
        return &b[0..32];
    } else {
        return b;
    }
}