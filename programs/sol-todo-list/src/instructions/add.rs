use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::error::TodoListError;
use crate::states::todo_list::*;
use crate::states::list_item::*;
use crate::instructions::create_list::*;

pub fn add(ctx: Context<Add>, _list_name: String, item_name: String, bounty: u64) -> Result<()> {
    let user = &ctx.accounts.user;
    let list = &mut ctx.accounts.list;
    let item = &mut ctx.accounts.item;

    // Check that the list isn't already full.
    if list.lines.len() >= list.capacity as usize {
        return Err(TodoListError::ListFull.into());
    }

    // Add item into the list.
    list.lines.push(item.key());
    item.name = item_name;
    item.creator = user.key();
    
    // Add bounty to the item.
    let account_lamports = item.to_account_info().lamports();
    let transfer_amount = bounty
        .checked_sub(account_lamports)
        .ok_or(TodoListError::BountyTooSmall)?;

    if transfer_amount > 0 {
        invoke(
            &transfer(
                &user.key(),
                &item.key(),
                transfer_amount,
            ),
            &[
                user.to_account_info(), 
                item.to_account_info(), 
                ctx.accounts.system_program.to_account_info()
            ]
        )?;
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(list_name: String, item_name: String)]
pub struct Add<'info> {
    #[account(
        mut,
        has_one = list_owner @ TodoListError::WrongListOwner,
        seeds = [
            b"todolist", 
            list_owner.key.as_ref(), 
            name_seed(&list_name)
        ],
        bump = list.bump
    )]
    pub list: Account<'info, TodoList>,
    /// CHECK: Only this account's key is used.
    pub list_owner: AccountInfo<'info>,
    #[account(
        init, payer = user, space = ListItem::space(&item_name)
    )]
    pub item: Account<'info, ListItem>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}