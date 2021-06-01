use super::account::Account;

#[derive(Debug)]
pub enum TransactionType {
    AccountCreate { public_key: String, private_key: String },
    CreateCoins { address: String, amount: u128 },
    TransferCoins { from: Account, to: Account, amount: u128 }
}

#[derive(Debug)]
pub struct Transaction {
    created_by: Account,
    created_at: std::time::SystemTime,
    transaction_type: TransactionType,
    nonce: u128
}

impl Transaction {
    // we need methods for
    // 1. create new transaction
    // 2. verify transaction
    //   2.1. check balance
    //   2.2. check nonce
    // 3. execute transaction
}
