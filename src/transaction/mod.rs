use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use super::account::Account;
use std::time::SystemTime;

#[derive(Debug)]
pub enum TransactionType {
    AccountCreate {
        created_at: SystemTime,
        user: Account
    },
    CreateCoins {
        address: String,
        amount: u128,
    },
    TransferCoins {
        from: Account,
        to: Account,
        amount: u128,
    },
}

#[derive(Debug)]
pub struct Transaction {
    created_by: Account,
    created_at: SystemTime,
    transaction: TransactionType,
    nonce: u128,
    pub hash: Option<String>
}

impl Transaction {
    // 1. create new/empty transaction.
    // the user takes this and fills it with whatever
    pub fn new(account: Account, trx_type: TransactionType) -> Self {
        Transaction {
            created_by: account,
            created_at: SystemTime::now(),
            transaction: trx_type,
            nonce: 0,
            hash: None
        }
    }

    // calculate the hash of the transaction
    pub fn calculate_hash(&mut self) {
        let mut hasher = DefaultHasher::new();
        let trx_string = format!("{:?}", (&self.transaction, &self.created_at, &self.created_by, &self.nonce) );
        trx_string.hash(&mut hasher);
        self.hash = Some(hasher.finish().to_string());
    }

    // 2. execute transaction
    pub fn execute(&self) -> Result<(), String> {
        todo!()
    }

}
