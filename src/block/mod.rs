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
    pub timestamp: SystemTime,
    pub hash: Option<String>,
    pub nonce: u128,
    pub prev_hash: Option<String>,
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// create a new block
    pub fn new() -> Block {
        Block {
            timestamp: SystemTime::now(),
            hash: None,
            nonce: 0,
            // todo: the prev_hash will be populated when the block gets added to the chain.
            // this will be done in the `chain` module
            prev_hash: None,
            transactions: Vec::new() 
        }
    }

    /// add new transaction
    pub fn add_transaction(self: &mut Self, trx: Transaction) {
        // now this is where the magic happens:
        // every transaction has data in it and we convert it into a hash, this is the transactions
        // hash.
        // then we take the hash and of the block and update it with the new transaction's hash
        self.transactions.push(trx);
        self.calculate_hash();
    }

    fn calculate_hash(self: &mut Self) {
        let mut hasher = DefaultHasher::new();
        let mut temp_block_str = String::new();

        for trx in self.transactions.iter_mut() {
            // calculate hash for the transaction
            trx.calculate_hash();
            temp_block_str.push_str(trx.hash.as_ref().unwrap());
        }
        // calculate hash for the block
        temp_block_str.hash(&mut hasher);
        self.hash = Some(hasher.finish().to_string());
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn check_add_new_blocks() {
//         let test_block = Block::new();
//     }
// }
