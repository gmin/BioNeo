use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::errors::NftStakingError;
use crate::structures::{Pool, User};

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    /// 质押池账户
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    
    /// 质押记录账户
    #[account(mut)]
    pub stake: Account<'info, User>,
    
    /// 奖励代币账户
    #[account(mut)]
    pub reward_token_account: Account<'info, TokenAccount>,
    
    /// 用户的代币账户
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    /// 用户签名
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// 代币程序
    pub token_program: Program<'info, Token>,
}

pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let stake = &mut ctx.accounts.stake;
    let current_time = Clock::get()?.unix_timestamp;

    require!(pool.is_active, NftStakingError::PoolInactive);

    // 计算可领取奖励
    let rewards = calculate_rewards(stake, pool, current_time)?;
    require!(rewards > 0, NftStakingError::NoRewardsToClaim);

    // 转移奖励代币
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.reward_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.pool.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        },
    );

    token::transfer(transfer_ctx, rewards)?;

    // 更新状态
    stake.last_claim_time = current_time;
    stake.rewards_claimed = stake.rewards_claimed.checked_add(rewards)
        .ok_or(NftStakingError::ArithmeticOverflow)?;

    let pool = &mut ctx.accounts.pool;
    pool.total_rewards = pool.total_rewards.checked_add(rewards)
        .ok_or(NftStakingError::ArithmeticOverflow)?;

    Ok(())
}

fn calculate_rewards(
    stake: &User,
    pool: &Pool,
    current_time: i64,
) -> Result<u64> {
    let time_staked = current_time - stake.last_claim_time;
    let daily_reward = stake.amount
        .checked_mul(pool.reward_rate)
        .ok_or(NftStakingError::ArithmeticOverflow)?
        .checked_div(10000)
        .ok_or(NftStakingError::ArithmeticOverflow)?;
    
    let rewards = daily_reward
        .checked_mul((time_staked / (24 * 60 * 60)) as u64)
        .ok_or(NftStakingError::ArithmeticOverflow)?;

    Ok(rewards)
} 