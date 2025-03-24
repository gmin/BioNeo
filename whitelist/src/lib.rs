use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

declare_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");

/// BioNeo 白名单合约
/// 负责白名单代币的释放管理
/// 
/// 功能：
/// 1. 支持三个等级的白名单地址
/// 2. 分12个月逐步释放代币
/// 3. 每个白名单地址有固定的释放比例
#[program]
pub mod whitelist {
    use super::*;

    /// 初始化白名单
    /// 
    /// 参数：
    /// - whitelist1: 白名单1地址，获得2.5%代币
    /// - whitelist2: 白名单2地址，获得1.5%代币
    /// - whitelist3: 白名单3地址，获得1.0%代币
    /// - start_time: 开始释放时间
    /// 
    /// 功能：
    /// 1. 设置白名单地址
    /// 2. 计算每个白名单的释放金额
    /// 3. 设置释放开始时间
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

    /// 释放代币
    /// 
    /// 功能：
    /// 1. 验证释放时间
    /// 2. 计算月度释放金额
    /// 3. 转移代币给白名单地址
    /// 
    /// 安全控制：
    /// 1. 时间控制：只能在指定时间后释放
    /// 2. 权限控制：只有白名单地址可以释放
    /// 3. 数量控制：最多释放12个月
    /// 4. 溢出保护：使用 checked_add 防止溢出
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

/// 初始化白名单所需的账户
#[derive(Accounts)]
pub struct InitializeWhitelist<'info> {
    /// 白名单状态账户，存储白名单信息和释放状态
    #[account(
        init,
        payer = authority,
        space = 8 + WhitelistState::LEN,
        seeds = [b"whitelist_state"],
        bump
    )]
    pub whitelist_state: Account<'info, WhitelistState>,
    
    /// 合约管理员
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// 白名单合约的代币账户
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    
    /// 系统程序
    pub system_program: Program<'info, System>,
}

/// 释放代币所需的账户
#[derive(Accounts)]
pub struct ReleaseTokens<'info> {
    /// 白名单状态账户
    #[account(mut)]
    pub whitelist_state: Account<'info, WhitelistState>,
    
    /// 白名单合约的代币账户
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    
    /// 用户的代币账户
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    /// 用户签名
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// 代币程序
    pub token_program: Program<'info, Token>,
}

/// 白名单状态账户
#[account]
pub struct WhitelistState {
    /// 合约管理员地址
    pub authority: Pubkey,
    
    /// 白名单1地址
    pub whitelist1: Pubkey,
    
    /// 白名单2地址
    pub whitelist2: Pubkey,
    
    /// 白名单3地址
    pub whitelist3: Pubkey,
    
    /// 总代币数量
    pub total_amount: u64,
    
    /// 白名单1的释放金额
    pub whitelist1_amount: u64,
    
    /// 白名单2的释放金额
    pub whitelist2_amount: u64,
    
    /// 白名单3的释放金额
    pub whitelist3_amount: u64,
    
    /// 开始释放时间
    pub start_time: i64,
    
    /// 最后释放时间
    pub last_release_time: i64,
    
    /// 总释放数量
    pub total_released: u64,
}

impl WhitelistState {
    /// 账户大小
    pub const LEN: usize = 32 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8;
}

/// 白名单合约错误类型
#[error_code]
pub enum WhitelistError {
    /// 释放时间未到
    #[msg("释放时间未到")]
    ReleaseTimeNotReached,
    
    /// 没有更多释放计划
    #[msg("没有更多释放计划")]
    NoMoreReleases,
    
    /// 超出释放周期
    #[msg("超出释放周期")]
    ExceededReleasePeriod,
    
    /// 未授权的释放
    #[msg("未授权的释放")]
    UnauthorizedRelease,
    
    /// 算术溢出
    #[msg("算术溢出")]
    ArithmeticOverflow,
} 