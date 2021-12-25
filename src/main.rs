use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use time::OffsetDateTime;
#[derive(Debug)]
#[derive(Hash)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: u8,
}

#[derive(Debug)]
#[derive(Hash)]
struct Block {
    index: usize,
    timestamp: OffsetDateTime,
    transactions: Vec<Transaction>,
}

trait ExposeDetails {
    fn get_index(self) -> usize;
}

impl ExposeDetails for Block {
    fn get_index(self) -> usize {
        self.index
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
#[derive(Debug)]
struct BlockChain {
    chain: Vec<Block>,
}

impl BlockChain {
    fn new() -> Self {
        let mut chain = Vec::new();
        chain.push(Block {
            index: 1,
            timestamp: OffsetDateTime::now_utc(),
            transactions: Vec::new(),
        });
        BlockChain { chain }
    }
    fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }
}

fn main() {
    let chain = BlockChain::new();
    let block = chain.last_block();
    if let Some(block) = block {
        println!("Hello");
        
        println!("{:?}", calculate_hash(&block));
    }
    println! {"{:?}", chain}
}
