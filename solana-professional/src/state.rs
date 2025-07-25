use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Counter {
    pub value: u32,  // The actual number we will increment
}
