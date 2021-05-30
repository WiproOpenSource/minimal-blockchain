use super::block::Block;
use std::vec::Vec;

#[derive(Debug)]
pub struct BlockChain {
    block: Vec<Block>
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            block: Vec::new()
        }
    }

    pub fn add_block(self: &mut Self, block: Block) -> Result<(), String> {
        // need to add some validations here
        // before push
        self.block.push(block);
        Ok(())
    }
}
