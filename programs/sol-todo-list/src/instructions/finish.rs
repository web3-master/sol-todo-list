use anchor_lang::prelude::*;
use crate::states::todo_list::*;
use crate::states::list_item::*;
use crate::error::*;
use crate::instructions::create_list::*;

pub fn finish(ctx: Context<Finish>, _list_name: String) -> Result<()> {
    let list = &mut ctx.accounts.list;
    let item = &mut ctx.accounts.item;

    // Check if the item is contained in list.
    if !list.lines.contains(&item.key()) {
        return Err(TodoListError::ItemNotFound.into());
    }

    // Check if the item is finished already.
    if item.list_owner_finished && item.creator_finished {
        return Err(TodoListError::ItemAlreadyFinished.into());
    }

    // Check if the user is list owner or item creator.
    let user_key = ctx.accounts.user.key;
    if user_key != &list.list_owner && user_key != &item.creator {
        return Err(TodoListError::WrongFinishPermission.into());
    }

    // Change finish state.
    if user_key == &list.list_owner {
        item.list_owner_finished = true;
    }
    if user_key == &item.creator {
        item.creator_finished = true;
    }

    // Refund bounty to list owner if both of list owner and item creator finished.
    if item.list_owner_finished && item.creator_finished {
        let bounty = item.to_account_info().lamports();
        **item.to_account_info().lamports.borrow_mut() = 0;
        **ctx.accounts.list_owner.lamports.borrow_mut() += bounty;

        // Remove finished item from the list.
        list.lines.retain(|key| key != &item.key());
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(list_name: String)]
pub struct Finish<'info> {
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

    /// CHECK: This account's data is not used.
    #[account(mut)]
    pub list_owner: AccountInfo<'info>,

    #[account(mut)]
    pub item: Account<'info, ListItem>,

    /// CHECK: This account's data is not used.
    #[account(mut, address = item.creator @ TodoListError::WrongItemCreator)]
    pub item_creator: AccountInfo<'info>,
    pub user: Signer<'info>
}