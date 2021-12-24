#[derive(Debug)]
struct Block {
    index: u8,
    timestamp: u8,
}
#[derive(Debug)]
struct BlockChain {
    chain: Vec<Block>
}

impl BlockChain {
    fn new() -> Self {
        let mut chain = Vec::new();
        chain.push(Block {
            index: 1,
            timestamp: 2,
        });
        BlockChain {
            chain
        }
        
    }

   
}

fn main() {
    let chain = BlockChain::new();
    println! {"{:?}", chain}
}
