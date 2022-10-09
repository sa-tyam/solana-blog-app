use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile {
    pub authority: Pubkey,
    pub name: String,
    pub image: String,
    pub last_blog: u8,
    pub blog_count: u8,
}

#[account]
#[derive(Default)]
pub struct BlogAccount {
    pub authority: Pubkey,
    pub idx: u8,
    pub title: String,
    pub content: String,
    pub image: String,
}