use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::errors::LpStakingError;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    /// 质押池账户
    #[account(
        init,
        payer = authority,
        space = 8 + Pool::LEN,
        seeds = [b"pool", &[pool_id]],
        bump
    )]
    pub pool: Account<'info, Pool>,
    
    /// 质押池的代币账户
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    
    /// 奖励代币账户
    #[account(mut)]
    pub reward_token_account: Account<'info, TokenAccount>,
    
    /// 合约管理员
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// 系统程序
    pub system_program: Program<'info, System>,
}

pub fn initialize_pool(
    ctx: Context<InitializePool>,
    pool_id: u64,
    duration: u64,
    reward_rate: u64,
    min_stake: u64,
    max_stake: u64,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    pool.id = pool_id;
    pool.authority = ctx.accounts.authority.key();
    pool.duration = duration;
    pool.reward_rate = reward_rate;
    pool.min_stake = min_stake;
    pool.max_stake = max_stake;
    pool.total_staked = 0;
    pool.total_rewards = 0;
    pool.created_at = Clock::get()?.unix_timestamp;
    pool.is_active = true;

    Ok(())
}

#[account]
pub struct Pool {
    /// 质押池ID
    pub id: u64,
    
    /// 合约管理员地址
    pub authority: Pubkey,
    
    /// 质押期限（天）
    pub duration: u64,
    
    /// 奖励率（每10000个代币的奖励）
    pub reward_rate: u64,
    
    /// 最小质押数量
    pub min_stake: u64,
    
    /// 最大质押数量
    pub max_stake: u64,
    
    /// 总质押数量
    pub total_staked: u64,
    
    /// 总发放奖励
    pub total_rewards: u64,
    
    /// 创建时间
    pub created_at: i64,
    
    /// 是否活跃
    pub is_active: bool,
}

impl Pool {
    /// 账户大小
    pub const LEN: usize = 8 + 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 1;
} 