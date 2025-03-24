use anchor_lang::prelude::*;

#[error_code]
pub enum NftStakingError {
    /// 质押池未激活
    #[msg("Pool is not active")]
    PoolInactive,
    
    /// 质押数量过小
    #[msg("Stake amount is too small")]
    StakeTooSmall,
    
    /// 质押数量过大
    #[msg("Stake amount is too large")]
    StakeTooLarge,
    
    /// 质押期限未到
    #[msg("Stake period has not ended")]
    StakePeriodNotEnded,
    
    /// 没有可领取的奖励
    #[msg("No rewards to claim")]
    NoRewardsToClaim,
    
    /// 算术溢出
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
} 