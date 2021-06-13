use super::account::Account;
use super::block::Block;
use super::worldstate::WorldState;
use std::vec::Vec;

/// This struct has three properties:
/// 1. hash
/// 2. blocks
/// 3. accounts
#[derive(Debug)]
pub struct BlockChain {
    pub hash: Option<String>,
    pub blocks: Vec<Block>,
    pub accounts: Vec<Account>,
}

impl WorldState for BlockChain {
    fn create_account(&mut self, account: &Account) -> Result<(), &str> {
        for acc in self.accounts.iter() {
            if acc.public_key.eq(&account.public_key) {
                return Err("This account already exists");
            };
        }

        self.accounts.push(account.clone());

        Ok(())
    }
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            hash: None,
            blocks: Vec::new(),
            accounts: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) -> Result<(), String> {
        // need to add some validations here
        // before push
        // it is here at this point that we need to process the transactions
        for trx in block.transactions.iter() {
           trx.execute(self)?
        };

        self.blocks.push(block);
        Ok(())
    }
}
