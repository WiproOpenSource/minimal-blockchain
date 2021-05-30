mod block;
mod chain;

use block::Block;
use chain::BlockChain;

fn main() {
    let mut block_chain = BlockChain::new();
    let genesis_block = Block::new("Genesis Block");

    let result = BlockChain::add_block(&mut block_chain, genesis_block);

    println!("{:#?}", result);
    println!("{:#?}", block_chain);
}
