use anchor_lang::prelude::*;
use crate::states::todo_list::*;
use crate::states::list_item::*;
use crate::error::*;
use crate::instructions::create_list::*;

pub fn cancel(ctx: Context<Cancel>, _list_name: String) -> Result<()> {
    let list = &mut ctx.accounts.list;
    let item = &mut ctx.accounts.item;
    let user_key = ctx.accounts.user.to_account_info().key;
    let item_key = item.to_account_info().key;

    // Check if user is list_owner or item creator.
    if &list.list_owner != user_key && &item.creator != user_key {
        return Err(TodoListError::WrongCancelPermission.into());
    }

    // Check if item is contained in the list.
    if !list.lines.contains(item_key) {
        return Err(TodoListError::ItemNotFound.into());
    }

    // Return the bounty to the item creator.
    let bounty = item.to_account_info().lamports();
    **item.to_account_info().lamports.borrow_mut() = 0;
    **ctx.accounts.item_creator.lamports.borrow_mut() += bounty;

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
    
    #[account(mut)]
    pub item: Account<'info, ListItem>,

    /// CHECK: This accounts' data is not used.
    #[account(mut, address = item.creator @ TodoListError::WrongItemCreator)]
    pub item_creator: AccountInfo<'info>,

    pub user: Signer<'info>,
}