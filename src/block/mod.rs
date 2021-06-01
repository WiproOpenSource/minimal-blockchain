use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;
use super::transaction::Transaction;

// we will define our block here
//
// the block will be based on the following assumptions:
// 1. one block will have multiple transactions

/// Structure for a new block.
/// One block can have multiple transactions. I'm not really sure if we need `nonce` here, but I
/// will keep it here for now
#[derive(Debug)]
pub struct Block {
    timestamp: SystemTime,
    hash: Option<String>,
    nonce: u128,
    prev_hash: Option<String>,
    data: Vec<Transaction>,
}

impl Block {
    /// create a new block
    pub fn new() -> Self {
        Block {
            timestamp: SystemTime::now(),
            hash: None,
            nonce: 0,
            // todo: the prev_hash will be populated when the block gets added to the chain.
            // this will be done in the `chain` module
            prev_hash: None,
            data: Vec::new() 
        }
    }

    
    pub fn add_transaction() -> Self {

        todo!()
    }

    // fn calculate_hash(data: &str) -> Option<String> {
    //     let mut hash = DefaultHasher::new();
    //     data.hash(&mut hash);
    //     return Some(hash.finish().to_string());
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_add_new_blocks() {
        let test_block = Block::new("test_string");
        assert_eq!(test_block.data.unwrap(), String::from("test_string"));
    }
}
