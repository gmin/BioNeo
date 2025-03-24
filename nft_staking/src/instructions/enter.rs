use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::errors::NftStakingError;
use crate::structures::{Pool, User};

#[derive(Accounts)]
pub struct Stake<'info> {
    /// 质押池账户
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    
    /// 质押记录账户
    #[account(
        init,
        payer = user,
        space = User::SIZE,
        seeds = [b"stake", pool.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub stake: Account<'info, User>,
    
    /// 质押池代币账户
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    
    /// 用户的 NFT 代币账户
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    /// 用户签名
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// 系统程序
    pub system_program: Program<'info, System>,
    
    /// 代币程序
    pub token_program: Program<'info, Token>,
    
    /// 租金系统
    pub rent: Sysvar<'info, Rent>,
}

pub fn stake(
    ctx: Context<Stake>,
    amount: u64,
) -> Result<()> {
    let pool = &ctx.accounts.pool;
    require!(pool.is_active, NftStakingError::PoolInactive);
    require!(amount >= pool.min_stake, NftStakingError::StakeTooSmall);
    require!(amount <= pool.max_stake, NftStakingError::StakeTooLarge);

    // 转移 NFT
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
    stake.address = ctx.accounts.user.key();
    stake.pool = pool.key();
    stake.amount = amount;
    stake.start_time = Clock::get()?.unix_timestamp;
    stake.last_claim_time = stake.start_time;
    stake.rewards_claimed = 0;

    // 更新质押池状态
    let pool = &mut ctx.accounts.pool;
    pool.total_staked = pool.total_staked.checked_add(amount)
        .ok_or(NftStakingError::ArithmeticOverflow)?;

    Ok(())
} 