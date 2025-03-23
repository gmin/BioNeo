pub mod constants;
pub mod structures;
pub mod tools;
pub mod errors;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::token;
use constants::*;
use structures::{
    cancel_staking::*, claim_rewards::*, enter_staking::*, initialize_staking::*,
    initialize_user::*, Staked, StakingInstance, StakingPool, User,
    lp_staking::*,
    nft_staking::*,
};
use tools::{generate_release_timestamps, test_generate_release_timestamp};

declare_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");

pub fn update_reward_pool(current_timestamp: u64, staking_instance: &mut StakingInstance) {
    // 遍历每个质押池
    for pool in staking_instance.pools.iter_mut() {
        // 如果没有份额，跳过此池
        if pool.total_shares == 0 {
            continue;
        }

        // 计算时间差（当前时间戳 - 上次奖励时间戳）
        let time_diff = current_timestamp
            .checked_sub(pool.last_reward_timestamp)
            .unwrap_or(0);

        // 如果时间差为 0，跳过此池
        if time_diff == 0 {
            continue;
        }

        // 计算池子的总奖励收入（奖励速率 * 时间差）
        let income = pool
            .reward_token_per_sec
            .checked_mul(time_diff)
            .unwrap_or(0);

        // 更新 `accumulated_reward_per_share`
        if pool.total_shares > 0 {
            // 每份奖励计算
            let reward_per_share = income
                .checked_mul(COMPUTATION_DECIMALS) // 精度调整
                .unwrap_or(0)
                .checked_div(pool.total_shares) // 每份奖励
                .unwrap_or(0);

            // 累加每份奖励的累计值
            pool.accumulated_reward_per_share = pool
                .accumulated_reward_per_share
                .checked_add(reward_per_share)
                .unwrap_or(pool.accumulated_reward_per_share); // 防止溢出
        }

        // 更新最后奖励时间戳为当前时间戳
        pool.last_reward_timestamp = current_timestamp;
    }
}

pub fn store_pending_reward(
    staking_instance: &mut StakingInstance,
    user_instance: &mut User,
    staked_info_number: u64, // 修改为索引
) -> Result<()> {
    // 获取用户对应的质押信息
    let staked_info = &mut user_instance.staked_info[staked_info_number as usize];

    // 确保该质押池已被质押
    if !staked_info.is_staked {
        return Ok(()); // 如果该质押池没有质押，直接返回
    }

    // 获取质押类型对应的池子
    let stake_type = staked_info.stake_type as usize;

    // 检查 stake_type 是否为有效池子索引
    // if stake_type >= staking_instance.pools.len() {
    //     return Err(ErrorCode::InvalidStakeType.into()); // 自定义错误类型
    // }

    // 获取对应池子
    let pool = &staking_instance.pools[stake_type];

    // 计算用户在该池子的待领取奖励
    let pending_reward = (staked_info.deposited_amount as u128)
        .checked_mul(pool.accumulated_reward_per_share as u128)
        .and_then(|v| v.checked_div(COMPUTATION_DECIMALS as u128))
        .and_then(|v| v.checked_sub(staked_info.reward_debt as u128))
        .unwrap_or(0) as u64; // 最终将结果转换回 u64 类型，如果需要
                              // 如果待领取奖励为 0，直接返回
    if pending_reward == 0 {
        return Ok(());
    }

    // 更新该质押池的累计奖励
    staked_info.accumulated_reward = staked_info
        .accumulated_reward
        .checked_add(pending_reward)
        .unwrap_or(staked_info.accumulated_reward); // 防止溢出

    // 更新用户的 reward_debt 为最新的池子状态
    staked_info.reward_debt = (staked_info.deposited_amount as u128)
        .checked_mul(pool.accumulated_reward_per_share as u128)
        .and_then(|v| v.checked_div(COMPUTATION_DECIMALS as u128))
        .unwrap_or(staked_info.reward_debt as u128) as u64;
    Ok(())
}

pub fn update_reward_debt(
    staking_instance: &mut StakingInstance,
    user_instance: &mut User,
    staked_info_number: u64, // 用户质押池的索引
) {
    // 获取用户对应的质押信息
    let staked_info = &mut user_instance.staked_info[staked_info_number as usize];

    // 确保该质押池已被质押
    if !staked_info.is_staked {
        return; // 如果该质押池没有质押，直接返回
    }
    // 获取质押类型对应的池子
    let stake_type = staked_info.stake_type as usize;
    // 检查 stake_type 是否为有效池子索引
    if stake_type >= staking_instance.pools.len() {
        return; // 无效的池子索引，直接返回
    }

    // 获取对应池子
    let pool = &staking_instance.pools[stake_type];

    // msg!(
    //     "Hello world!!",
    //     pool.accumulated_reward_per_share,
    //     staked_info.deposited_amount
    // );
    // 更新该质押池的 reward_debt
    // msg!(
    //     "staked_info
    //     .deposited_amount",
    //     staked_info.deposited_amount
    // );
    // msg!("accumulated_reward_per_share", accumulated_reward_per_share);

    staked_info.reward_debt = (staked_info.deposited_amount as u128)
        .checked_mul(pool.accumulated_reward_per_share as u128)
        .and_then(|v| v.checked_div(COMPUTATION_DECIMALS as u128))
        .unwrap_or(0) as u64;
}

pub fn is_authorized(user: &Pubkey, authority: &Pubkey) -> bool {
    user == authority
}
pub fn can_unstake(staked: &Staked, current_timestamp: u64) -> bool {
    staked.is_staked && staked.stake_end_time <= current_timestamp
}

pub fn calculate_referral_reward(user: &User, amount: u64) -> u64 {
    // 计算推荐奖励，假设为10%
    let referral_reward = amount * 10 / 100;
    referral_reward
}

#[program]
pub mod bioneo {
    use super::*;

    // LP 质押相关指令
    pub fn initialize_staking(
        ctx: Context<InitializeStaking>,
        reward_token_per_sec: u64,
    ) -> Result<()> {
        instructions::lp_staking::initialize::handler(ctx, reward_token_per_sec)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        instructions::lp_staking::initialize_user::handler(ctx)
    }

    pub fn enter_staking(
        ctx: Context<EnterStaking>,
        amount: u64,
        stake_type: u64,
        referrer: Option<Pubkey>,
    ) -> Result<()> {
        instructions::lp_staking::enter::handler(ctx, amount, stake_type, referrer)
    }

    pub fn cancel_staking(ctx: Context<CancelStaking>, record_index: u64) -> Result<()> {
        instructions::lp_staking::cancel::handler(ctx, record_index)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::lp_staking::claim::handler(ctx)
    }

    // NFT 质押相关指令
    pub fn initialize_nft_staking(ctx: Context<InitializeNftStaking>) -> Result<()> {
        instructions::nft_staking::initialize::handler(ctx)
    }

    pub fn initialize_nft_user(ctx: Context<InitializeNftUser>) -> Result<()> {
        instructions::nft_staking::initialize_user::handler(ctx)
    }

    pub fn stake_nft(
        ctx: Context<StakeNft>,
        stake_type: u64,
        rarity: u64,
    ) -> Result<()> {
        instructions::nft_staking::stake::handler(ctx, stake_type, rarity)
    }

    pub fn unstake_nft(ctx: Context<UnstakeNft>, record_index: u64) -> Result<()> {
        instructions::nft_staking::unstake::handler(ctx, record_index)
    }

    pub fn claim_nft_rewards(ctx: Context<ClaimNftRewards>) -> Result<()> {
        instructions::nft_staking::claim::handler(ctx)
    }
}

// 导出所有指令的上下文结构
pub use instructions::lp_staking::{
    initialize::InitializeStaking,
    initialize_user::InitializeUser,
    enter::EnterStaking,
    cancel::CancelStaking,
    claim::ClaimRewards,
};

pub use instructions::nft_staking::{
    initialize::InitializeNftStaking,
    initialize_user::InitializeNftUser,
    stake::StakeNft,
    unstake::UnstakeNft,
    claim::ClaimNftRewards,
};

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid stake type provided. The stake type must correspond to an existing pool.")]
    InvalidStakeType,

    #[msg("Invalid stake staked info index.")]
    InvalidStakedInfoIndex,

    #[msg("Insufficient token account balance.")]
    TokenAccountBalanceInsufficient,

    #[msg("Failed to fetch system clock.")]
    ClockUnavailable,

    #[msg("User token account mint does not match staking token mint.")]
    MintAccountIsNotMatch,

    #[msg("Arithmetic overflow occurred.")]
    Overflow,

    #[msg("Arithmetic underflow occurred.")]
    Underflow,

    #[msg("User has already staked and cannot stake again.")]
    UserAlreadyStaked,

    #[msg("User has no staking to cancel.")]
    NoStakingToCancel,

    #[msg("Staking period has not matured yet.")]
    StakingNotMatured,

    #[msg("No rewards available to claim.")]
    NoRewardsToClaim,

    #[msg("Insufficient reward account balance.")]
    InsufficientRewardBalance, // 奖励账户余额不足

    #[msg("No Staking available to claim.")]
    NoStakingToClaimRewards,

    #[msg("UserSuperiorTokenAccount  does not match.")]
    UserSuperiorTokenAccountIsNotMatch,

    #[msg("User address  does not match.")]
    UserAccountIsNotMatch,

    #[msg("User need cliam rewards.")]
    NeedCliamRewards,
}
