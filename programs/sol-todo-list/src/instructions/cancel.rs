use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::states::todo_list::*;
use crate::states::list_item::*;
use crate::error::*;
use crate::instructions::create_list::*;

pub fn cancel(ctx: Context<Cancel>, _list_name: String) -> Result<()> {
    let list = &mut ctx.accounts.list;
    let item = &mut ctx.accounts.item;
    let user_key = ctx.accounts.user.key();
    let item_key = item.to_account_info().key;

    // Check if user is list_owner or item creator.
    if user_key != list.list_owner && user_key != item.creator {
        return Err(TodoListError::WrongCancelPermission.into());
    }

    // Check if item is contained in the list.
    if !list.lines.contains(item_key) {
        return Err(TodoListError::ItemNotFound.into());
    }

    // Return the bounty to the item creator.
    invoke(
        &transfer(
            item_key,
            &item.creator,
            item.to_account_info().lamports()
        ),
        &[
            item.to_account_info(),
            ctx.accounts.item_creator.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ]
    )?;

    list.lines.retain(|key| key != item_key);

    Ok(())
}

#[derive(Accounts)]
#[instruction(list_name: String)]
pub struct Cancel<'info> {
    #[account(
        mut,
        has_one = list_owner @ TodoListError::WrongListOwner,
        seeds = [
            b"todolist",
            list_owner.key().as_ref(),
            name_seed(&list_name)
        ],
        bump = list.bump
    )]
    pub list: Account<'info, TodoList>,
    
    /// CHECK: Only this account's key is used.
    pub list_owner: AccountInfo<'info>,
    
    #[account(mut, signer)]
    pub item: Account<'info, ListItem>,

    /// CHECK: Only this accounts' key is used.
    #[account(mut, address = item.creator @ TodoListError::WrongItemCreator)]
    pub item_creator: AccountInfo<'info>,

    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}