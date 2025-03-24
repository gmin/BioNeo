use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::errors::LpStakingError;
use super::initialize::Pool;

#[derive(Accounts)]
pub struct Stake<'info> {
    /// 质押池账户
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    
    /// 质押记录账户
    #[account(
        init,
        payer = user,
        space = 8 + Stake::LEN,
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

pub fn stake(
    ctx: Context<Stake>,
    amount: u64,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    require!(pool.is_active, LpStakingError::PoolInactive);
    require!(amount >= pool.min_stake, LpStakingError::StakeTooSmall);
    require!(amount <= pool.max_stake, LpStakingError::StakeTooLarge);

    // 转移代币
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.pool_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        },
    );

    token::transfer(transfer_ctx, amount)?;

    // 创建质押记录
    let stake = &mut ctx.accounts.stake;
    stake.user = ctx.accounts.user.key();
    stake.pool = pool.key();
    stake.amount = amount;
    stake.start_time = Clock::get()?.unix_timestamp;
    stake.last_claim_time = stake.start_time;
    stake.rewards_claimed = 0;

    // 更新质押池状态
    pool.total_staked = pool.total_staked.checked_add(amount)
        .ok_or(LpStakingError::ArithmeticOverflow)?;

    Ok(())
}

#[account]
pub struct Stake {
    /// 用户地址
    pub user: Pubkey,
    
    /// 质押池地址
    pub pool: Pubkey,
    
    /// 质押数量
    pub amount: u64,
    
    /// 开始时间
    pub start_time: i64,
    
    /// 最后领取时间
    pub last_claim_time: i64,
    
    /// 已领取奖励
    pub rewards_claimed: u64,
}

impl Stake {
    /// 账户大小
    pub const LEN: usize = 32 + 32 + 8 + 8 + 8 + 8;
} 