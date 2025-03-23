use anchor_lang::prelude::*;
use crate::structures::nft_staking::record::NftStakeRecord;

#[account]
pub struct NftUser {
    pub total_staked_nfts: u64,       // 用户总质押 NFT 数量
    pub total_reward_debt: u64,       // 用户总奖励债务
    pub total_accumulated_reward: u64, // 用户累计获得的奖励
    pub total_claimed_reward: u64,     // 用户已领取的奖励
    pub staking_count: u64,           // 用户质押次数
    pub staking_records: [NftStakeRecord; 20], // 用户质押记录，最多20个
} 