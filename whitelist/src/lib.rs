use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

declare_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");

/// 白名单地址 1，2.5%
pub const WHITELIST_ADDRESS_1: Pubkey = pubkey!("11111111111111111111111111111111");

/// 白名单地址 2，1.5%
pub const WHITELIST_ADDRESS_2: Pubkey = pubkey!("22222222222222222222222222222222");

/// 白名单地址 3，1%
pub const WHITELIST_ADDRESS_3: Pubkey = pubkey!("33333333333333333333333333333333");

/// 释放周期（月）
pub const RELEASE_PERIOD: u8 = 36;

/// 时间常量
pub const SECONDS_PER_MONTH: i64 = 30 * 24 * 60 * 60;

/// BioNeo 白名单合约
/// 负责白名单代币的释放管理
/// 
/// 功能：
/// 1. 支持三个等级的白名单地址
/// 2. 分36个月逐步释放代币
/// 3. 每个白名单地址有固定的释放比例
#[program]
pub mod whitelist {
    use super::*;

    /// 初始化白名单
    /// 
    /// 功能：
    /// 1. 计算每个白名单的释放金额
    /// 2. 设置开始释放时间为当前时间
    pub fn initialize_whitelist(
        ctx: Context<InitializeWhitelist>,
    ) -> Result<()> {
        let whitelist_state = &mut ctx.accounts.whitelist_state;
        
        // 检查是否已经初始化
        require!(
            whitelist_state.authority == Pubkey::default(),
            WhitelistError::AlreadyInitialized
        );

        whitelist_state.authority = ctx.accounts.authority.key();
        whitelist_state.whitelist1 = WHITELIST_ADDRESS_1;
        whitelist_state.whitelist2 = WHITELIST_ADDRESS_2;
        whitelist_state.whitelist3 = WHITELIST_ADDRESS_3;
        whitelist_state.total_amount = ctx.accounts.token_account.amount;
        whitelist_state.start_time = Clock::get()?.unix_timestamp;

        // 计算每个白名单的释放金额
        whitelist_state.whitelist1_amount = whitelist_state.total_amount
            .checked_mul(5)
            .and_then(|v| v.checked_div(10))
            .ok_or(WhitelistError::ArithmeticOverflow)?; // 2.5%
        whitelist_state.whitelist2_amount = whitelist_state.total_amount
            .checked_mul(3)
            .and_then(|v| v.checked_div(10))
            .ok_or(WhitelistError::ArithmeticOverflow)?; // 1.5%
        whitelist_state.whitelist3_amount = whitelist_state.total_amount
            .checked_mul(2)
            .and_then(|v| v.checked_div(10))
            .ok_or(WhitelistError::ArithmeticOverflow)?; // 1.0%

        emit!(WhitelistInitialized {
            authority: whitelist_state.authority,
            total_amount: whitelist_state.total_amount,
            start_time: whitelist_state.start_time,
        });

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
    /// 3. 数量控制：最多释放36个月
    /// 4. 溢出保护：使用 checked_add 防止溢出
    pub fn release_tokens(ctx: Context<ReleaseTokens>) -> Result<()> {
        let whitelist_state = &mut ctx.accounts.whitelist_state;
        let current_time = Clock::get()?.unix_timestamp;

        // 验证调用者是否为白名单地址
        let user_address = ctx.accounts.user.key();
        require!(
            user_address == whitelist_state.whitelist1 ||
            user_address == whitelist_state.whitelist2 ||
            user_address == whitelist_state.whitelist3,
            WhitelistError::UnauthorizedRelease
        );

        // 验证代币账户所有者
        require!(
            ctx.accounts.token_account.owner == whitelist_state.key(),
            WhitelistError::InvalidTokenAccount
        );

        // 验证释放时间
        require!(
            current_time >= whitelist_state.start_time,
            WhitelistError::ReleaseTimeNotReached
        );

        // 计算已经过去的月份
        let months_passed = ((current_time - whitelist_state.start_time) / SECONDS_PER_MONTH) as u64;
        require!(months_passed >= 1, WhitelistError::NoMoreReleases);

        // 获取用户的释放金额和已领取金额
        let (release_amount, claimed_amount) = if user_address == whitelist_state.whitelist1 {
            (whitelist_state.whitelist1_amount, &mut whitelist_state.whitelist1_claimed)
        } else if user_address == whitelist_state.whitelist2 {
            (whitelist_state.whitelist2_amount, &mut whitelist_state.whitelist2_claimed)
        } else {
            (whitelist_state.whitelist3_amount, &mut whitelist_state.whitelist3_claimed)
        };

        // 计算月度释放金额
        let monthly_release = release_amount
            .checked_div(RELEASE_PERIOD as u64)
            .ok_or(WhitelistError::ArithmeticOverflow)?;

        // 计算应领取的总金额
        let total_should_claim = monthly_release
            .checked_mul(months_passed)
            .ok_or(WhitelistError::ArithmeticOverflow)?;

        // 计算本次可领取金额
        let current_claim = total_should_claim
            .checked_sub(*claimed_amount)
            .ok_or(WhitelistError::ArithmeticOverflow)?;

        require!(current_claim > 0, WhitelistError::NoMoreReleases);

        // 转移代币
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: whitelist_state.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        );

        token::transfer(transfer_ctx, current_claim)?;

        // 更新状态
        *claimed_amount = total_should_claim;
        whitelist_state.last_release_time = current_time;
        whitelist_state.total_released = whitelist_state.total_released
            .checked_add(current_claim)
            .ok_or(WhitelistError::ArithmeticOverflow)?;

        emit!(TokensReleased {
            user: user_address,
            amount: current_claim,
            total_claimed: *claimed_amount,
        });

        Ok(())
    }

    /// 查询可领取代币数量
    /// 
    /// 功能：
    /// 1. 验证地址是否为白名单地址
    /// 2. 计算当前可领取的代币数量
    #[view]
    pub fn get_claimable_amount(
        ctx: Context<GetClaimableAmount>,
    ) -> Result<u64> {
        let whitelist_state = &ctx.accounts.whitelist_state;
        let current_time = Clock::get()?.unix_timestamp;

        // 验证查询地址是否为白名单地址
        let user_address = ctx.accounts.user.key();
        require!(
            user_address == whitelist_state.whitelist1 ||
            user_address == whitelist_state.whitelist2 ||
            user_address == whitelist_state.whitelist3,
            WhitelistError::UnauthorizedRelease
        );

        // 验证释放时间
        require!(
            current_time >= whitelist_state.start_time,
            WhitelistError::ReleaseTimeNotReached
        );

        // 计算已经过去的月份
        let months_passed = ((current_time - whitelist_state.start_time) / SECONDS_PER_MONTH) as u64;
        require!(months_passed >= 1, WhitelistError::NoMoreReleases);

        // 获取用户的释放金额和已领取金额
        let (release_amount, claimed_amount) = if user_address == whitelist_state.whitelist1 {
            (whitelist_state.whitelist1_amount, whitelist_state.whitelist1_claimed)
        } else if user_address == whitelist_state.whitelist2 {
            (whitelist_state.whitelist2_amount, whitelist_state.whitelist2_claimed)
        } else {
            (whitelist_state.whitelist3_amount, whitelist_state.whitelist3_claimed)
        };

        // 计算月度释放金额
        let monthly_release = release_amount
            .checked_div(RELEASE_PERIOD as u64)
            .ok_or(WhitelistError::ArithmeticOverflow)?;

        // 计算应领取的总金额
        let total_should_claim = monthly_release
            .checked_mul(months_passed)
            .ok_or(WhitelistError::ArithmeticOverflow)?;

        // 计算本次可领取金额
        let current_claim = total_should_claim
            .checked_sub(claimed_amount)
            .ok_or(WhitelistError::ArithmeticOverflow)?;

        require!(current_claim > 0, WhitelistError::NoMoreReleases);

        Ok(current_claim)
    }
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

    /// 白名单1已领取数量
    pub whitelist1_claimed: u64,

    /// 白名单2已领取数量
    pub whitelist2_claimed: u64,

    /// 白名单3已领取数量
    pub whitelist3_claimed: u64,
}

impl WhitelistState {
    /// 账户大小
    pub const LEN: usize = 32 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8;
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

    /// 无效的代币账户
    #[msg("无效的代币账户")]
    InvalidTokenAccount,

    /// 合约已经初始化
    #[msg("合约已经初始化")]
    AlreadyInitialized,
}

/// 白名单初始化事件
#[event]
pub struct WhitelistInitialized {
    /// 合约管理员地址
    pub authority: Pubkey,
    
    /// 总代币数量
    pub total_amount: u64,
    
    /// 开始释放时间
    pub start_time: i64,
}

/// 代币释放事件
#[event]
pub struct TokensReleased {
    /// 用户地址
    pub user: Pubkey,
    
    /// 释放数量
    pub amount: u64,
    
    /// 总领取数量
    pub total_claimed: u64,
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
    /// 这个账户应该已经包含了所有要分配给白名单用户的代币
    /// 在初始化时，我们会读取这个账户的代币数量作为总分配量
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

/// 查询可领取代币数量所需的账户
#[derive(Accounts)]
pub struct GetClaimableAmount<'info> {
    /// 白名单状态账户
    pub whitelist_state: Account<'info, WhitelistState>,
    
    /// 查询用户地址
    pub user: Signer<'info>,
} 