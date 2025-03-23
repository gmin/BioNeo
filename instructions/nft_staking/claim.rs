use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};
use crate::structures::nft_staking::{
    instance::NftStakingInstance,
    pool::NftStakingPool,
    user::NftUser,
    record::NftStakeRecord,
};

#[derive(Accounts)]
pub struct ClaimNftRewards<'info> {
    #[account(mut)]
    pub staking_instance: Account<'info, NftStakingInstance>,

    #[account(mut)]
    pub user_account: Account<'info, NftUser>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: 这是用户的奖励代币账户
    #[account(mut)]
    pub user_reward_account: AccountInfo<'info>,

    /// CHECK: 这是合约的奖励代币账户
    #[account(mut)]
    pub staking_reward_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ClaimNftRewards>) -> Result<()> {
    let staking_instance = &mut ctx.accounts.staking_instance;
    let user_account = &mut ctx.accounts.user_account;
    
    // 获取当前时间戳
    let current_timestamp = Clock::get()?.unix_timestamp as u64;
    
    // 计算总奖励
    let mut total_reward = 0u64;
    
    // 遍历所有质押记录
    for record in user_account.staking_records.iter_mut() {
        if !record.is_active {
            continue;
        }

        // 获取对应的质押池
        let pool = &mut staking_instance.pools[record.stake_type as usize];
        
        // 更新池子的奖励
        let time_diff = current_timestamp
            .checked_sub(pool.last_reward_timestamp)
            .unwrap_or(0);
            
        if time_diff > 0 {
            let reward = pool
                .reward_token_per_sec
                .checked_mul(time_diff)
                .unwrap_or(0);
                
            pool.accumulated_reward_per_share = pool
                .accumulated_reward_per_share
                .checked_add(
                    reward
                        .checked_mul(COMPUTATION_DECIMALS)
                        .unwrap_or(0)
                        .checked_div(pool.total_shares)
                        .unwrap_or(0),
                )
                .unwrap_or(0);
                
            pool.last_reward_timestamp = current_timestamp;
        }
        
        // 计算用户奖励
        let reward = record
            .shares
            .checked_mul(pool.accumulated_reward_per_share)
            .unwrap_or(0)
            .checked_div(COMPUTATION_DECIMALS)
            .unwrap_or(0)
            .checked_sub(record.reward_debt)
            .unwrap_or(0);
            
        total_reward = total_reward.checked_add(reward).unwrap_or(0);
        
        // 更新奖励债务
        record.reward_debt = record
            .shares
            .checked_mul(pool.accumulated_reward_per_share)
            .unwrap_or(0)
            .checked_div(COMPUTATION_DECIMALS)
            .unwrap_or(0);
    }
    
    // 检查是否有可领取的奖励
    require!(total_reward > 0, ErrorCode::NoRewardsToClaim);
    
    // 更新用户账户
    user_account.total_accumulated_reward = user_account
        .total_accumulated_reward
        .checked_add(total_reward)
        .unwrap_or(0);
    user_account.total_claimed_reward = user_account
        .total_claimed_reward
        .checked_add(total_reward)
        .unwrap_or(0);
    
    // 转移奖励代币
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.staking_reward_account.to_account_info(),
            to: ctx.accounts.user_reward_account.to_account_info(),
            authority: ctx.accounts.staking_instance.to_account_info(),
        },
        &[&[
            b"nft_staking_instance".as_ref(),
            &[*ctx.bumps.get("staking_instance").unwrap()],
        ]],
    );
    anchor_spl::token::transfer(transfer_ctx, total_reward)?;

    Ok(())
} 