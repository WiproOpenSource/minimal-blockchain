use std::time::SystemTime;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash)]
pub struct Transaction {
    account: String,
    timestamp: SystemTime,
    data: Option<String>
}

impl Transaction {
    pub fn new( account: &str, data: Option<&str>) -> Self
    {
        Transaction{
            account: account.to_string(),
            timestamp: SystemTime::now(),
            data: Some(data.unwrap().to_string())
        }
    }
}


// one block will only have one transaction
#[derive(Debug)]
pub struct Block<'a> {
    timestamp: SystemTime,
    transaction: Option<&'a Transaction>,
    current_block_hash: Option<String>,
    previous_block_hash: Option<String>
}

impl<'t> Block<'t> {
    pub fn new(previous_block_hash: Option<String>, transaction: Option<&'t Transaction>) -> Block {
        Block {
            timestamp: SystemTime::now(),
            transaction,
            current_block_hash: Block::calculate_hash(transaction.unwrap()),
            previous_block_hash
        }
    }

    fn calculate_hash(transaction: &Transaction) -> Option<String> {
        let mut hash = DefaultHasher::new();
        transaction.hash(&mut hash);
        return Some(hash.finish().to_string());
    }
}
