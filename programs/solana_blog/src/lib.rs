use anchor_lang::prelude::*;

pub mod constants;
pub mod states;

use crate::{states::*, constants::*};
declare_id!("7AsEAzLnD54sggAimpqqqyyuyW7WGaSQXzszvKLgVBJx");

#[program]
pub mod solana_blog {
    use super::*;

    pub fn initialize_user (
        ctx: Context<InitializeUser>,
        _name: String,
        _image: String,
    ) -> Result<()> {

        let user_profile = &mut ctx.accounts.user_profile;

        user_profile.authority = ctx.accounts.authority.key();
        user_profile.name = _name;
        user_profile.image = _image;
        user_profile.last_blog = 0;
        user_profile.blog_count = 0;

        Ok(())
    }

    pub fn add_blog (
        ctx: Context<AddBlog>,
        _title: String,
        _content: String,
        _image: String,
    ) -> Result<()> {

        let user_profile = &mut ctx.accounts.user_profile;
        let blog_account = &mut ctx.accounts.blog_account;

        blog_account.authority = ctx.accounts.authority.key();
        blog_account.idx = user_profile.last_blog;
        blog_account.title = _title;
        blog_account.content = _content;
        blog_account.image = _image;

        user_profile.last_blog = user_profile.last_blog
        .checked_add(1)
        .unwrap();
        user_profile.blog_count = user_profile.blog_count
        .checked_add(1)
        .unwrap();

        Ok(())
    }

    pub fn update_blog (
        ctx: Context<UpdateBlog>,
        _idx: u8,
        _title: String,
        _content: String,
        _image: String,
    ) -> Result<()> {

        let blog_account = &mut ctx.accounts.blog_account;

        blog_account.title = _title;
        blog_account.content = _content;
        blog_account.image = _image;

        Ok(())
    }

    pub fn delete_blog (
        ctx: Context<DeleteBlog>,
        _idx: u8,
    ) -> Result<()> {
        
        let user_profile = &mut ctx.accounts.user_profile;

        user_profile.blog_count = user_profile.blog_count
        .checked_sub(1)
        .unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser <'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct AddBlog <'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [BLOG_TAG, authority.key().as_ref(), &[user_profile.last_blog]],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<BlogAccount>(),
    )]
    pub blog_account: Box<Account<'info, BlogAccount>>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(_idx: u8)]
pub struct UpdateBlog <'info> {
    
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [BLOG_TAG, authority.key().as_ref(), &[_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub blog_account: Box<Account<'info, BlogAccount>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_idx: u8)]
pub struct DeleteBlog <'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        close = authority,
        seeds = [BLOG_TAG, authority.key().as_ref(), &[_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub blog_account: Box<Account<'info, BlogAccount>>,

    pub system_program: Program<'info, System>,
}
