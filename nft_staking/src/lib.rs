use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

declare_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");

/// BioNeo NFT 挖矿合约
/// 负责 NFT 的质押和奖励发放
/// 
/// 功能：
/// 1. 支持多个质押池
/// 2. 支持不同期限的质押
/// 3. 支持批量质押和赎回
/// 4. 支持奖励计算和领取
#[program]
pub mod nft_staking {
    use super::*;

    /// 初始化质押池
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

    /// 质押 NFT
    pub fn stake(
        ctx: Context<Stake>,
        amount: u64,
    ) -> Result<()> {
        instructions::enter::stake(ctx, amount)
    }

    /// 赎回质押的 NFT
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        instructions::cancel::unstake(ctx)
    }

    /// 领取奖励
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::claim::claim_rewards(ctx)
    }
}

pub use instructions::*;
pub use errors::*;
pub use structures::*; 