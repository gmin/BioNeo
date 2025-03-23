use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

declare_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");

#[program]
pub mod whitelist {
    use super::*;

    // 初始化白名单
    pub fn initialize_whitelist(
        ctx: Context<InitializeWhitelist>,
        whitelist1: Pubkey,
        whitelist2: Pubkey,
        whitelist3: Pubkey,
        start_time: i64,
    ) -> Result<()> {
        let whitelist_state = &mut ctx.accounts.whitelist_state;
        whitelist_state.authority = ctx.accounts.authority.key();
        whitelist_state.whitelist1 = whitelist1;
        whitelist_state.whitelist2 = whitelist2;
        whitelist_state.whitelist3 = whitelist3;
        whitelist_state.start_time = start_time;
        whitelist_state.total_amount = ctx.accounts.token_account.amount;

        // 计算每个白名单的释放金额
        whitelist_state.whitelist1_amount = whitelist_state.total_amount * 5 / 10; // 2.5%
        whitelist_state.whitelist2_amount = whitelist_state.total_amount * 3 / 10; // 1.5%
        whitelist_state.whitelist3_amount = whitelist_state.total_amount * 2 / 10; // 1.0%

        Ok(())
    }

    // 释放代币
    pub fn release_tokens(ctx: Context<ReleaseTokens>) -> Result<()> {
        let whitelist_state = &mut ctx.accounts.whitelist_state;
        let current_time = Clock::get()?.unix_timestamp;

        // 验证释放时间
        require!(
            current_time >= whitelist_state.start_time,
            WhitelistError::ReleaseTimeNotReached
        );

        // 计算已经过去的月份
        let months_passed = ((current_time - whitelist_state.start_time) / (30 * 24 * 60 * 60)) as u64;
        require!(months_passed > 0, WhitelistError::NoMoreReleases);
        require!(months_passed <= 12, WhitelistError::ExceededReleasePeriod);

        // 计算每个白名单的月度释放金额
        let whitelist1_release = whitelist_state.whitelist1_amount / 12;
        let whitelist2_release = whitelist_state.whitelist2_amount / 12;
        let whitelist3_release = whitelist_state.whitelist3_amount / 12;

        // 根据用户类型转移对应数量的代币
        let release_amount = if ctx.accounts.user.key() == whitelist_state.whitelist1 {
            whitelist1_release
        } else if ctx.accounts.user.key() == whitelist_state.whitelist2 {
            whitelist2_release
        } else if ctx.accounts.user.key() == whitelist_state.whitelist3 {
            whitelist3_release
        } else {
            return Err(WhitelistError::UnauthorizedRelease.into());
        };

        // 转移代币
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.whitelist_state.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        );

        token::transfer(transfer_ctx, release_amount)?;

        // 更新状态
        whitelist_state.last_release_time = current_time;
        whitelist_state.total_released = whitelist_state.total_released.checked_add(release_amount)
            .ok_or(WhitelistError::ArithmeticOverflow)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeWhitelist<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + WhitelistState::LEN,
        seeds = [b"whitelist_state"],
        bump
    )]
    pub whitelist_state: Account<'info, WhitelistState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseTokens<'info> {
    #[account(mut)]
    pub whitelist_state: Account<'info, WhitelistState>,
    
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct WhitelistState {
    pub authority: Pubkey,
    pub whitelist1: Pubkey,
    pub whitelist2: Pubkey,
    pub whitelist3: Pubkey,
    pub total_amount: u64,
    pub whitelist1_amount: u64,
    pub whitelist2_amount: u64,
    pub whitelist3_amount: u64,
    pub start_time: i64,
    pub last_release_time: i64,
    pub total_released: u64,
}

impl WhitelistState {
    pub const LEN: usize = 32 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8;
}

#[error_code]
pub enum WhitelistError {
    #[msg("释放时间未到")]
    ReleaseTimeNotReached,
    
    #[msg("没有更多释放计划")]
    NoMoreReleases,
    
    #[msg("超出释放周期")]
    ExceededReleasePeriod,
    
    #[msg("未授权的释放")]
    UnauthorizedRelease,
    
    #[msg("算术溢出")]
    ArithmeticOverflow,
} 