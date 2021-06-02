mod block;
mod chain;
mod transaction;
mod account;

use account::Account;
use transaction::Transaction;
use transaction::TransactionType;
use block::Block;
use chain::BlockChain;

fn main() {
    // create blockchain
    let block_chain = BlockChain::new();
    
    // create a new block
    let block = Block::new();

    // create account
    let user1 = Account::new("abc", "123");
    let user2 = Account::new("def", "456");

    // register user with blockchain
    let mut trx = TransactionType::AccountCreate{ created_at: std::time::SystemTime::now(), user: user1.clone() };
    let trx_create_user1 = Transaction::new(user1.clone(), trx );

    trx = TransactionType::AccountCreate{ created_at: std::time::SystemTime::now(), user: user2.clone() };
    let trx_create_user2 = Transaction::new(user2.clone(), trx );
    
    // create coins for user1
    trx = TransactionType::CreateCoins{ address: user1.address.clone(), amount: 10_000 };
    let trx_create_coins = Transaction::new(user1.clone(), trx );

    // TODO:
    // 1. add execute method for transaction
    // 2. the execute method will be executed before it is added to the block -> use result or
    //    something
    // 3. skip user verification if trx type is create user
}
