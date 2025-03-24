use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    /// 质押池 ID
    pub id: u64,
    
    /// 质押池创建者
    pub authority: Pubkey,
    
    /// 质押期限（天）
    pub duration: u64,
    
    /// 奖励率（基点，1% = 100）
    pub reward_rate: u64,
    
    /// 最小质押数量
    pub min_stake: u64,
    
    /// 最大质押数量
    pub max_stake: u64,
    
    /// 总质押数量
    pub total_staked: u64,
    
    /// 总奖励数量
    pub total_rewards: u64,
    
    /// 创建时间
    pub created_at: i64,
    
    /// 是否激活
    pub is_active: bool,
}

impl Pool {
    /// 账户大小
    pub const SIZE: usize = 8 + // discriminator
        8 + // id
        32 + // authority
        8 + // duration
        8 + // reward_rate
        8 + // min_stake
        8 + // max_stake
        8 + // total_staked
        8 + // total_rewards
        8 + // created_at
        1; // is_active
} 