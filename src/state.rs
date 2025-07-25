use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Counter {
    pub valur: u32,  // The actual number we will increment
}
