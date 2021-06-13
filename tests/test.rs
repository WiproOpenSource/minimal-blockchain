extern crate minimal_blockchain as minimal;

use minimal::account::Account;
use minimal::block::Block;
use minimal::chain::BlockChain;
use minimal::transaction::{Transaction, TransactionType};

use std::time::SystemTime;

#[test]
fn duplicate_accounts() {
    let mut block = Block::new();

    let user1 = Account::new("123", "abc");
    let trx1 = Transaction::new(user1.clone(), TransactionType::AccountCreate{ created_at: SystemTime::now(), user: user1 });
    
    block.add_transaction(trx1);

    let user2 = Account::new("123", "abc");
    let trx2 = Transaction::new(user2.clone(), TransactionType::AccountCreate{ created_at: SystemTime::now(), user: user2 });

    block.add_transaction(trx2);

    let mut blockchain = BlockChain::new();

    println!("{:#?}", blockchain.add_block(block) );
}
