use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");

#[program]
pub mod bioneo_ido {
    use super::*;

    // 初始化众筹
    pub fn initialize_ido(
        ctx: Context<InitializeIdo>,
        start_time: i64,
        end_time: i64,
        price_per_share: u64,
        total_shares: u64,
    ) -> Result<()> {
        let ido_state = &mut ctx.accounts.ido_state;
        ido_state.authority = ctx.accounts.authority.key();
        ido_state.token_mint = ctx.accounts.token_mint.key();
        ido_state.start_time = start_time;
        ido_state.end_time = end_time;
        ido_state.price_per_share = price_per_share;
        ido_state.total_shares = total_shares;
        ido_state.sold_shares = 0;
        ido_state.total_raised = 0;
        ido_state.token_amount_per_share = ctx.accounts.token_state.ido_amount / total_shares;

        Ok(())
    }

    // 参与众筹
    pub fn participate_ido(
        ctx: Context<ParticipateIdo>,
        shares: u64,
    ) -> Result<()> {
        let ido_state = &mut ctx.accounts.ido_state;
        let participation = &mut ctx.accounts.participation;

        // 检查众筹时间
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            current_time >= ido_state.start_time && current_time <= ido_state.end_time,
            IdoError::IdoNotActive
        );

        // 检查剩余份额
        require!(
            ido_state.sold_shares.checked_add(shares).unwrap() <= ido_state.total_shares,
            IdoError::InsufficientShares
        );

        // 计算支付金额
        let payment_amount = ido_state.price_per_share.checked_mul(shares)
            .ok_or(IdoError::ArithmeticOverflow)?;

        // 转移 USDC
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_usdc_account.to_account_info(),
                to: ctx.accounts.treasury_usdc_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        );

        token::transfer(transfer_ctx, payment_amount)?;

        // 更新参与记录
        participation.user = ctx.accounts.user.key();
        participation.shares = shares;
        participation.payment_amount = payment_amount;
        participation.claimed_amount = 0;
        participation.last_claim_time = ido_state.start_time;

        // 更新众筹状态
        ido_state.sold_shares = ido_state.sold_shares.checked_add(shares)
            .ok_or(IdoError::ArithmeticOverflow)?;
        ido_state.total_raised = ido_state.total_raised.checked_add(payment_amount)
            .ok_or(IdoError::ArithmeticOverflow)?;

        Ok(())
    }

    // 领取代币
    pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
        let participation = &mut ctx.accounts.participation;
        let ido_state = &ctx.accounts.ido_state;

        // 计算可领取数量
        let current_time = Clock::get()?.unix_timestamp;
        let months_passed = ((current_time - participation.last_claim_time) / (30 * 24 * 60 * 60)) as u64;
        
        require!(months_passed > 0, IdoError::NoTokensToClaim);

        let claim_amount = ido_state.token_amount_per_share
            .checked_mul(participation.shares)
            .ok_or(IdoError::ArithmeticOverflow)?
            .checked_mul(months_passed)
            .ok_or(IdoError::ArithmeticOverflow)?;

        // 转移代币
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.treasury_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.ido_state.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        );

        token::transfer(transfer_ctx, claim_amount)?;

        // 更新状态
        participation.claimed_amount = participation.claimed_amount.checked_add(claim_amount)
            .ok_or(IdoError::ArithmeticOverflow)?;
        participation.last_claim_time = current_time;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeIdo<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + IdoState::LEN,
        seeds = [b"ido_state"],
        bump
    )]
    pub ido_state: Account<'info, IdoState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub token_mint: Account<'info, Mint>,
    pub token_state: Account<'info, crate::TokenState>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ParticipateIdo<'info> {
    #[account(mut)]
    pub ido_state: Account<'info, IdoState>,
    
    #[account(
        init,
        payer = user,
        space = 8 + Participation::LEN,
    )]
    pub participation: Account<'info, Participation>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub user_usdc_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub treasury_usdc_account: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub ido_state: Account<'info, IdoState>,
    
    #[account(mut)]
    pub participation: Account<'info, Participation>,
    
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct IdoState {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub price_per_share: u64,
    pub total_shares: u64,
    pub sold_shares: u64,
    pub total_raised: u64,
    pub token_amount_per_share: u64,
}

impl IdoState {
    pub const LEN: usize = 32 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8;
}

#[account]
pub struct Participation {
    pub user: Pubkey,
    pub shares: u64,
    pub payment_amount: u64,
    pub claimed_amount: u64,
    pub last_claim_time: i64,
}

impl Participation {
    pub const LEN: usize = 32 + 8 + 8 + 8 + 8;
}

#[error_code]
pub enum IdoError {
    #[msg("众筹未开始或已结束")]
    IdoNotActive,
    
    #[msg("剩余份额不足")]
    InsufficientShares,
    
    #[msg("没有可领取的代币")]
    NoTokensToClaim,
    
    #[msg("算术溢出")]
    ArithmeticOverflow,
} 