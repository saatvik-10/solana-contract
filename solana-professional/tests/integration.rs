use borsh::BorshDeserialize;
use solana_contract::{instruction::CounterInstruction, state::Counter};
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
    Ok(())
}
