use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");

#[program]
pub mod bioneo_token {
    use super::*;

    // 初始化代币
    pub fn initialize_token(
        ctx: Context<InitializeToken>,
        total_supply: u64,
    ) -> Result<()> {
        // 验证总供应量是否为 2100 万
        require!(
            total_supply == 21_000_000,
            TokenError::InvalidTotalSupply
        );

        let token_state = &mut ctx.accounts.token_state;
        token_state.authority = ctx.accounts.authority.key();
        token_state.total_supply = total_supply;
        
        // 分配方案
        token_state.lp_staking_amount = total_supply * 20 / 100; // 20% for LP staking
        token_state.nft_staking_amount = total_supply * 60 / 100; // 60% for NFT staking
        token_state.ido_amount = total_supply * 10 / 100; // 10% for IDO
        token_state.whitelist_amount = total_supply * 5 / 100; // 5% for whitelist
        token_state.liquidity_amount = total_supply * 5 / 100; // 5% for liquidity

        // 铸造代币
        let mint_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );

        token::mint_to(mint_ctx, total_supply)?;

        // 转移代币到各个业务模块的代币账户
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        );

        // 转移 LP 挖矿代币
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_account.to_account_info(),
                    to: ctx.accounts.lp_staking_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                &[&[b"token_state".as_ref(), &[*ctx.bumps.get("token_state").unwrap()]]],
            ),
            token_state.lp_staking_amount,
        )?;

        // 转移 NFT 挖矿代币
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_account.to_account_info(),
                    to: ctx.accounts.nft_staking_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                &[&[b"token_state".as_ref(), &[*ctx.bumps.get("token_state").unwrap()]]],
            ),
            token_state.nft_staking_amount,
        )?;

        // 转移 IDO 代币
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_account.to_account_info(),
                    to: ctx.accounts.ido_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                &[&[b"token_state".as_ref(), &[*ctx.bumps.get("token_state").unwrap()]]],
            ),
            token_state.ido_amount,
        )?;

        // 转移白名单代币
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_account.to_account_info(),
                    to: ctx.accounts.whitelist_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                &[&[b"token_state".as_ref(), &[*ctx.bumps.get("token_state").unwrap()]]],
            ),
            token_state.whitelist_amount,
        )?;

        // 转移流动性代币
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_account.to_account_info(),
                    to: ctx.accounts.liquidity_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                &[&[b"token_state".as_ref(), &[*ctx.bumps.get("token_state").unwrap()]]],
            ),
            token_state.liquidity_amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + TokenState::LEN,
        seeds = [b"token_state"],
        bump
    )]
    pub token_state: Account<'info, TokenState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub lp_staking_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub nft_staking_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub ido_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub whitelist_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub liquidity_account: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct TokenState {
    pub authority: Pubkey,
    pub total_supply: u64,
    pub lp_staking_amount: u64,
    pub nft_staking_amount: u64,
    pub ido_amount: u64,
    pub whitelist_amount: u64,
    pub liquidity_amount: u64,
}

impl TokenState {
    pub const LEN: usize = 32 + 8 + 8 + 8 + 8 + 8 + 8;
}

#[error_code]
pub enum TokenError {
    #[msg("总供应量必须为 2100 万")]
    InvalidTotalSupply,
    
    #[msg("代币余额不足")]
    InsufficientTokens,
} 