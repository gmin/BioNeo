use anchor_lang::prelude::*;

#[error_code]
pub enum LpStakingError {
    #[msg("无效的质押类型")]
    InvalidStakeType,

    #[msg("代币账户余额不足")]
    TokenAccountBalanceInsufficient,

    #[msg("无效的质押记录索引")]
    InvalidStakedInfoIndex,

    #[msg("用户已经质押")]
    UserAlreadyStaked,

    #[msg("质押未到期")]
    StakingNotMatured,

    #[msg("需要先领取奖励")]
    NeedCliamRewards,

    #[msg("没有可取消的质押")]
    NoStakingToCancel,

    #[msg("没有可领取的奖励")]
    NoRewardsToClaim,

    #[msg("奖励账户余额不足")]
    InsufficientRewardBalance,

    #[msg("时钟不可用")]
    ClockUnavailable,

    #[msg("代币账户不匹配")]
    MintAccountIsNotMatch,

    #[msg("用户账户不匹配")]
    UserAccountIsNotMatch,

    #[msg("算术溢出")]
    Overflow,

    #[msg("算术下溢")]
    Underflow,
} 