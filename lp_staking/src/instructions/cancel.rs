use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::errors::LpStakingError;
use super::{initialize::Pool, enter::Stake};

#[derive(Accounts)]
pub struct Unstake<'info> {
    /// 质押池账户
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    
    /// 质押记录账户
    #[account(
        mut,
        close = user,
        seeds = [b"stake", pool.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub stake: Account<'info, Stake>,
    
    /// 质押池的代币账户
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    
    /// 用户的代币账户
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    /// 用户签名
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// 代币程序
    pub token_program: Program<'info, Token>,
}

pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let stake = &ctx.accounts.stake;
    let current_time = Clock::get()?.unix_timestamp;

    require!(pool.is_active, LpStakingError::PoolInactive);
    require!(
        current_time >= stake.start_time + (pool.duration * 24 * 60 * 60) as i64,
        LpStakingError::StakePeriodNotEnded
    );

    // 计算未领取奖励
    let rewards = calculate_rewards(stake, pool, current_time)?;

    // 转移质押代币
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.pool_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.pool.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        },
    );

    token::transfer(transfer_ctx, stake.amount)?;

    // 更新质押池状态
    let pool = &mut ctx.accounts.pool;
    pool.total_staked = pool.total_staked.checked_sub(stake.amount)
        .ok_or(LpStakingError::ArithmeticOverflow)?;

    Ok(())
}

fn calculate_rewards(
    stake: &Stake,
    pool: &Pool,
    current_time: i64,
) -> Result<u64> {
    let time_staked = current_time - stake.last_claim_time;
    let daily_reward = stake.amount
        .checked_mul(pool.reward_rate)
        .ok_or(LpStakingError::ArithmeticOverflow)?
        .checked_div(10000)
        .ok_or(LpStakingError::ArithmeticOverflow)?;
    
    let rewards = daily_reward
        .checked_mul((time_staked / (24 * 60 * 60)) as u64)
        .ok_or(LpStakingError::ArithmeticOverflow)?;

    Ok(rewards)
} 