use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};
use crate::structures::nft_staking::{
    instance::NftStakingInstance,
    pool::NftStakingPool,
    user::NftUser,
    record::NftStakeRecord,
};

#[derive(Accounts)]
pub struct StakeNft<'info> {
    #[account(mut)]
    pub staking_instance: Account<'info, NftStakingInstance>,

    #[account(mut)]
    pub user_account: Account<'info, NftUser>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: 这是用户的 NFT 代币账户
    #[account(mut)]
    pub user_nft_account: AccountInfo<'info>,

    /// CHECK: 这是合约的 NFT 保管账户
    #[account(mut)]
    pub vault: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<StakeNft>, stake_type: u64, rarity: u64) -> Result<()> {
    let staking_instance = &mut ctx.accounts.staking_instance;
    let user_account = &mut ctx.accounts.user_account;
    
    // 检查质押类型是否有效
    require!(stake_type < 3, ErrorCode::InvalidStakeType);
    
    // 检查用户是否还有可用的质押记录槽位
    require!(
        user_account.staking_count < 20,
        ErrorCode::MaxStakingRecordsReached
    );

    // 获取当前时间戳
    let current_timestamp = Clock::get()?.unix_timestamp as u64;

    // 计算质押结束时间
    let end_time = match stake_type {
        0 => current_timestamp + 90 * 24 * 60 * 60, // 3个月
        1 => current_timestamp + 180 * 24 * 60 * 60, // 6个月
        2 => current_timestamp + 365 * 24 * 60 * 60, // 12个月
        _ => return Err(ErrorCode::InvalidStakeType.into()),
    };

    // 创建新的质押记录
    let new_record = NftStakeRecord {
        nft_mint: ctx.accounts.user_nft_account.key(),
        stake_type,
        rarity,
        shares: rarity, // 使用稀有度作为份额
        reward_debt: 0,
        start_time: current_timestamp,
        end_time,
        is_active: true,
    };

    // 更新用户账户
    user_account.staking_records[user_account.staking_count as usize] = new_record;
    user_account.staking_count += 1;
    user_account.total_staked_nfts += 1;

    // 更新质押池
    let pool = &mut staking_instance.pools[stake_type as usize];
    pool.total_shares += rarity;
    pool.total_nfts += 1;
    pool.last_reward_timestamp = current_timestamp;

    // 更新总质押 NFT 数量
    staking_instance.total_staked_nfts += 1;

    // 转移 NFT 到保管账户
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_nft_account.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    anchor_spl::token::transfer(transfer_ctx, 1)?;

    Ok(())
} 