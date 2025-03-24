use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::errors::NftStakingError;
use crate::structures::{Pool, Instance};

#[derive(Accounts)]
pub struct InitializePool<'info> {
    /// 质押池账户
    #[account(
        init,
        payer = authority,
        space = Pool::SIZE,
        seeds = [b"pool", &[pool_id]],
        bump
    )]
    pub pool: Account<'info, Pool>,
    
    /// 质押池代币账户
    #[account(
        init,
        payer = authority,
        token::mint = pool_token_mint,
        token::authority = pool,
        token::token_program = token_program,
    )]
    pub pool_token_account: Account<'info, TokenAccount>,
    
    /// 奖励代币账户
    #[account(
        init,
        payer = authority,
        token::mint = reward_token_mint,
        token::authority = pool,
        token::token_program = token_program,
    )]
    pub reward_token_account: Account<'info, TokenAccount>,
    
    /// 质押池代币铸币账户
    pub pool_token_mint: Account<'info, token::Mint>,
    
    /// 奖励代币铸币账户
    pub reward_token_mint: Account<'info, token::Mint>,
    
    /// 质押池创建者
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// 系统程序
    pub system_program: Program<'info, System>,
    
    /// 代币程序
    pub token_program: Program<'info, Token>,
    
    /// 租金系统
    pub rent: Sysvar<'info, Rent>,
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