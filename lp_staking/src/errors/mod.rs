use anchor_lang::prelude::*;

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