use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

// we will define our block here
//
// the block will be based on the following assumptions:
// 1. one block will hold only one piece of information

// basic struct for a Block
#[derive(Debug)]
pub struct Block {
    timestamp: SystemTime,
    hash: Option<String>,
    nonce: u128,
    prev_hash: Option<String>,
    data: Option<String>,
}

// utility methods we need to support the main operations in a block
impl Block {
    pub fn new(data: &str) -> Self {
        Block {
            timestamp: SystemTime::now(),
            hash: Block::calculate_hash(data),
            nonce: 0,
            prev_hash: None,
            data: Some(data.to_string()),
        }
    }

    // we need one more method to check if the hash produced is valid
    // basically, check if the first N characters are zeros
    pub fn calculate_hash(data: &str) -> Option<String> {
        let mut hash = DefaultHasher::new();
        data.hash(&mut hash);
        return Some(hash.finish().to_string());
    }
}
