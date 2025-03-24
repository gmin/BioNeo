use anchor_lang::prelude::*;

#[account]
pub struct Instance {
    /// 实例 ID
    pub id: u64,
    
    /// 创建者地址
    pub authority: Pubkey,
    
    /// 创建时间
    pub created_at: i64,
    
    /// 是否激活
    pub is_active: bool,
}

impl Instance {
    /// 账户大小
    pub const SIZE: usize = 8 + // discriminator
        8 + // id
        32 + // authority
        8 + // created_at
        1; // is_active
} 