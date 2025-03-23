use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub total_deposited_amount: u64, // 用户总存入的质押金额
    pub total_reward_debt: u64,      // 用户总奖励债务
    pub total_accumulated_reward: u64, // 用户累计获得的奖励
    pub total_claimed_reward: u64,    // 用户已领取的奖励
    pub total_referral_reward: u64,   // 用户推荐奖励
    pub referral_count: u64,          // 用户推荐人数
    pub referrer: Pubkey,             // 推荐人地址
    pub staking_count: u64,           // 用户质押次数
    pub staking_records: [StakingRecord; 10], // 用户质押记录，最多10个
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct StakingRecord {
    pub stake_type: u64,              // 质押类型
    pub amount: u64,                  // 质押金额
    pub shares: u64,                  // 质押份额
    pub reward_debt: u64,             // 奖励债务
    pub start_time: u64,              // 开始时间
    pub end_time: u64,                // 结束时间
    pub is_active: bool,              // 是否激活
} 