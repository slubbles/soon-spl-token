use anchor_lang::prelude::*;
use soon_spl_token::*;
use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
};

#[tokio::test]
async fn test_initialize_token() {
    let program_id = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "soon_spl_token",
        program_id,
        None,
    )
    .start()
    .await;

    let token_mint = Keypair::new();
    let token_authority = Keypair::new();

    let args = InitializeTokenArgs {
        name: "Test Token".to_string(),
        symbol: "TEST".to_string(),
        decimals: 9,
        total_supply: 1_000_000_000,
    };

    let ix = soon_spl_token::instruction::initialize_token(
        &program_id,
        &token_mint.pubkey(),
        &token_authority.pubkey(),
        &payer.pubkey(),
        args,
    );

    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &token_mint, &token_authority],
        recent_blockhash,
    );

    banks_client.process_transaction(tx).await.unwrap();

    // Verify token was initialized correctly
    let token_account = banks_client
        .get_account(token_mint.pubkey())
        .await
        .unwrap();
    assert!(token_account.owner == program_id);
}

#[tokio::test]
async fn test_mint_to() {
    // Similar setup as above
    let program_id = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "soon_spl_token",
        program_id,
        None,
    )
    .start()
    .await;

    let token_mint = Keypair::new();
    let token_authority = Keypair::new();
    let recipient = Keypair::new();

    // Initialize token first
    // ... (similar to test_initialize_token)

    // Test minting
    let args = MintToArgs {
        amount: 1000,
    };

    let ix = soon_spl_token::instruction::mint_to(
        &program_id,
        &token_mint.pubkey(),
        &recipient.pubkey(),
        &token_authority.pubkey(),
        args,
    );

    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &token_authority],
        recent_blockhash,
    );

    banks_client.process_transaction(tx).await.unwrap();

    // Verify minting was successful
    let recipient_account = banks_client
        .get_account(recipient.pubkey())
        .await
        .unwrap();
    assert!(recipient_account.owner == program_id);
}

#[tokio::test]
async fn test_transfer() {
    // Setup similar to above
    // Test transfer functionality
}

#[tokio::test]
async fn test_burn() {
    // Setup similar to above
    // Test burn functionality
}

#[tokio::test]
async fn test_vest() {
    // Setup similar to above
    // Test vesting functionality
} 