use borsh::{BorshDeserialize, BorshSerialize}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum CounterInstruction {
    Initialize,
    Increment
}