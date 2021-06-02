use super::Account;
use super::Block;
use std::vec::Vec;

#[derive(Debug)]
pub struct BlockChain {
    hash: Option<String>,
    block: Vec<Block>,
    accounts: Vec<Account>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            hash: None,
            block: Vec::new(),
            accounts: Vec::new(),
        }
    }

    pub fn add_block(self: &mut Self, block: Block) -> Result<(), String> {
        // need to add some validations here
        // before push
        self.block.push(block);
        Ok(())
    }
}
