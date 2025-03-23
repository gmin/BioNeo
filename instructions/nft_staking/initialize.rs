use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::structures::nft_staking::{instance::NftStakingInstance, pool::NftStakingPool};

#[derive(Accounts)]
pub struct InitializeNftStaking<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + NftStakingInstance::LEN,
        seeds = [b"nft_staking_instance"],
        bump
    )]
    pub staking_instance: Account<'info, NftStakingInstance>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: 这是奖励代币的 mint 地址
    pub reward_token_mint: AccountInfo<'info>,

    /// CHECK: 这是 NFT 集合地址
    pub nft_collection: AccountInfo<'info>,

    /// CHECK: 这是 NFT 保管账户
    pub vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeNftStaking>) -> Result<()> {
    let staking_instance = &mut ctx.accounts.staking_instance;
    
    // 初始化质押实例
    staking_instance.authority = ctx.accounts.authority.key();
    staking_instance.reward_token_mint = ctx.accounts.reward_token_mint.key();
    staking_instance.nft_collection = ctx.accounts.nft_collection.key();
    staking_instance.vault = ctx.accounts.vault.key();
    staking_instance.total_staked_nfts = 0;

    // 初始化三个质押池
    for i in 0..3 {
        staking_instance.pools[i] = NftStakingPool {
            stake_type: i as u64,
            reward_token_per_sec: 0,
            accumulated_reward_per_share: 0,
            last_reward_timestamp: 0,
            total_shares: 0,
            total_nfts: 0,
            rarity_multiplier: 1,
        };
    }

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeNftUser<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + NftUser::LEN,
        seeds = [b"nft_user", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, NftUser>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_user(ctx: Context<InitializeNftUser>) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    
    // 初始化用户账户
    user_account.total_staked_nfts = 0;
    user_account.total_reward_debt = 0;
    user_account.total_accumulated_reward = 0;
    user_account.total_claimed_reward = 0;
    user_account.staking_count = 0;

    Ok(())
} 