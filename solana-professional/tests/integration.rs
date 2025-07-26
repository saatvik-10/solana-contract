use borsh::BorshDeserialize;
use solana_contract::{instruction::CounterInstruction, process_instruction, state::Counter};
use solana_program_test::*;
use solana_sdk::{
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
    system_instruction,
    transaction::Transaction,
    transport::TransportError,
};
use std::str::FromStr;

#[tokio::test]
async fn test_initialize_and_increment_counter() -> Result<(), TransportError> {
    let program_id = Pubkey::from_str("8D7tbfehdxNVXAJWj1odYWfdYtAVB3WH82diEJExAsw4").unwrap();

    //initializing test environment
    let mut program_test = ProgramTest::new(
        "solana_contract",
        program_id,
        processor!(process_instruction), //the entrypoint
    );

    //starting local test-validator environment
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    //generating a new keypir representing the counter account on-chain
    let counter_keypair = Keypair::new();

    //requesting info to make the new account rent-exempt
    let rent = banks_client.get_rent().await.unwrap();
    let space = 4;
    let lamports = rent.minimum_balance(space);

    //transaction instruction to create the counter account owned by your program
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),           //payer funding account creation
        &counter_keypair.pubkey(), //new counter public key
        lamports,
        space as u64,
        &program_id, //owner of new account (our program)
    );

    //transaction with the create_account instruction and payer as the fee payer
    let mut transaction = Transaction::new_with_payer(
        &[create_account_ix],  //instruction to include in this transaction
        Some(&payer.pubkey()), //transaction fee payer
    );

    //sign the transaction with payer and counter account keypairs
    transaction.sign(&[&payer, &counter_keypair], recent_blockhash);

    //send the transaction to the rest validator and wait of the confirmation
    banks_client.process_transaction(transaction).await?;

    Ok(())
}
