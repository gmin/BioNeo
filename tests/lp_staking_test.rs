use anchor_lang::solana_program::hash::hash;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use bioneo::{
    self,
    instructions::lp_staking::*,
    structures::lp_staking::*,
};

#[tokio::test]
async fn test_initialize_staking() {
    let program = anchor_lang::solana_program::program_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");
    let payer = anchor_lang::solana_program::system_program::id();
    let reward_token_mint = Pubkey::new_unique();
    let staking_token_mint = Pubkey::new_unique();
    let lp_token_account = Pubkey::new_unique();
    let staking_instance = Pubkey::new_unique();

    let accounts = InitializeStaking {
        staking_instance,
        authority: payer,
        reward_token_mint,
        staking_token_mint,
        lp_token_account,
        system_program: anchor_lang::solana_program::system_program::id(),
        token_program: anchor_spl::token::id(),
        rent: anchor_lang::solana_program::sysvar::rent::id(),
    };

    let reward_token_per_sec = 1000u64;

    let result = program
        .instruction(
            bioneo::instruction::InitializeStaking { reward_token_per_sec },
            &accounts,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_initialize_user() {
    let program = anchor_lang::solana_program::program_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");
    let user = Pubkey::new_unique();
    let user_account = Pubkey::new_unique();

    let accounts = InitializeUser {
        user_account,
        user,
        system_program: anchor_lang::solana_program::system_program::id(),
        rent: anchor_lang::solana_program::sysvar::rent::id(),
    };

    let result = program
        .instruction(bioneo::instruction::InitializeUser, &accounts)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_enter_staking() {
    let program = anchor_lang::solana_program::program_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");
    let staking_instance = Pubkey::new_unique();
    let user_account = Pubkey::new_unique();
    let user = Pubkey::new_unique();
    let user_lp_token_account = Pubkey::new_unique();
    let lp_token_account = Pubkey::new_unique();
    let token_program = anchor_spl::token::id();

    let accounts = EnterStaking {
        staking_instance,
        user_account,
        user,
        user_lp_token_account,
        lp_token_account,
        token_program,
    };

    let amount = 1000u64;
    let stake_type = 0u64; // 3个月
    let referrer = None;

    let result = program
        .instruction(
            bioneo::instruction::EnterStaking {
                amount,
                stake_type,
                referrer,
            },
            &accounts,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_cancel_staking() {
    let program = anchor_lang::solana_program::program_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");
    let staking_instance = Pubkey::new_unique();
    let user_account = Pubkey::new_unique();
    let user = Pubkey::new_unique();
    let user_lp_token_account = Pubkey::new_unique();
    let lp_token_account = Pubkey::new_unique();
    let token_program = anchor_spl::token::id();

    let accounts = CancelStaking {
        staking_instance,
        user_account,
        user,
        user_lp_token_account,
        lp_token_account,
        token_program,
    };

    let record_index = 0u64;

    let result = program
        .instruction(
            bioneo::instruction::CancelStaking { record_index },
            &accounts,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_claim_rewards() {
    let program = anchor_lang::solana_program::program_id!("6iadRi4ps7itomsTNa34RikS6hkmx2z5Ls1h9EqLPu1y");
    let staking_instance = Pubkey::new_unique();
    let user_account = Pubkey::new_unique();
    let user = Pubkey::new_unique();
    let user_reward_account = Pubkey::new_unique();
    let staking_reward_account = Pubkey::new_unique();
    let token_program = anchor_spl::token::id();

    let accounts = ClaimRewards {
        staking_instance,
        user_account,
        user,
        user_reward_account,
        staking_reward_account,
        token_program,
    };

    let result = program
        .instruction(bioneo::instruction::ClaimRewards, &accounts)
        .await;

    assert!(result.is_ok());
} 