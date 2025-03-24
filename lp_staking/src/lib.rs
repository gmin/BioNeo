use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

declare_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");

/// BioNeo LP 挖矿合约
/// 负责 LP 代币的质押和奖励发放
/// 
/// 功能：
/// 1. 支持多个质押池
/// 2. 支持不同期限的质押
/// 3. 支持批量质押和赎回
/// 4. 支持奖励计算和领取
#[program]
pub mod lp_staking {
    use super::*;

    /// 初始化质押池
    /// 
    /// 参数：
    /// - pool_id: 质押池ID
    /// - duration: 质押期限（天）
    /// - reward_rate: 奖励率（每10000个代币的奖励）
    /// - min_stake: 最小质押数量
    /// - max_stake: 最大质押数量
    /// 
    /// 功能：
    /// 1. 创建新的质押池
    /// 2. 设置质押参数
    /// 3. 初始化奖励账户
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        pool_id: u64,
        duration: u64,
        reward_rate: u64,
        min_stake: u64,
        max_stake: u64,
    ) -> Result<()> {
        instructions::initialize::initialize_pool(ctx, pool_id, duration, reward_rate, min_stake, max_stake)
    }

    /// 质押 LP 代币
    /// 
    /// 参数：
    /// - amount: 质押数量
    /// 
    /// 功能：
    /// 1. 验证质押参数
    /// 2. 转移代币到质押账户
    /// 3. 创建质押记录
    /// 4. 更新质押池状态
    /// 
    /// 安全控制：
    /// 1. 数量限制：最小和最大质押数量
    /// 2. 余额检查：用户代币余额充足
    /// 3. 状态检查：质押池处于活跃状态
    pub fn stake(
        ctx: Context<Stake>,
        amount: u64,
    ) -> Result<()> {
        instructions::enter::stake(ctx, amount)
    }

    /// 赎回质押的代币
    /// 
    /// 功能：
    /// 1. 验证质押期限
    /// 2. 计算未领取奖励
    /// 3. 转移代币给用户
    /// 4. 更新质押记录
    /// 
    /// 安全控制：
    /// 1. 期限检查：必须达到最小质押期限
    /// 2. 权限检查：只有质押者可以赎回
    /// 3. 状态检查：质押池处于活跃状态
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        instructions::cancel::unstake(ctx)
    }

    /// 领取奖励
    /// 
    /// 功能：
    /// 1. 计算可领取奖励
    /// 2. 转移奖励代币
    /// 3. 更新领取记录
    /// 
    /// 安全控制：
    /// 1. 余额检查：奖励账户余额充足
    /// 2. 权限检查：只有质押者可以领取
    /// 3. 状态检查：质押池处于活跃状态
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::claim::claim_rewards(ctx)
    }
}

pub use instructions::*;
pub use errors::*;

/// 初始化质押池所需的账户
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

/// 质押所需的账户
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

/// 赎回所需的账户
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

/// 领取奖励所需的账户
#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    /// 质押池账户
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    
    /// 质押记录账户
    #[account(mut)]
    pub stake: Account<'info, Stake>,
    
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

/// 质押池账户
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

/// 质押记录账户
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

/// LP 挖矿合约错误类型
#[error_code]
pub enum LpStakingError {
    /// 质押池未激活
    #[msg("质押池未激活")]
    PoolInactive,
    
    /// 质押数量过小
    #[msg("质押数量过小")]
    StakeTooSmall,
    
    /// 质押数量过大
    #[msg("质押数量过大")]
    StakeTooLarge,
    
    /// 质押期限未结束
    #[msg("质押期限未结束")]
    StakePeriodNotEnded,
    
    /// 没有可领取的奖励
    #[msg("没有可领取的奖励")]
    NoRewardsToClaim,
    
    /// 算术溢出
    #[msg("算术溢出")]
    ArithmeticOverflow,
} 