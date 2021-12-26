use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use time::OffsetDateTime;
#[derive(Debug, Hash, Clone)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: u8,
}

#[derive(Debug, Hash, Clone)]
struct Block {
    index: usize,
    timestamp: OffsetDateTime,
    transactions: Vec<Transaction>,
    previous_hash: u64,
    proof: u8,
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
    current_transactions: Vec<Transaction>,
    chain: Vec<Block>,
}

impl BlockChain {
    fn new() -> Self {
        let mut chain = Vec::new();

        let genesis_block = Block {
            index: 0,
            timestamp: OffsetDateTime::now_utc(),
            transactions: Vec::new(),
            previous_hash: 0000000000,
            proof: 1,
        };
        chain.push(genesis_block);
        BlockChain {
            chain,
            current_transactions: Vec::new(),
        }
    }
    fn new_transaction(&mut self, sender: String, recipient: String, amount: u8) -> usize {
        let transaction = Transaction {
            sender,
            recipient,
            amount,
        };
        self.current_transactions.push(transaction);
        let next_block = self.last_block().clone().get_index();
        return next_block;
    }
    fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
}

fn main() {
    let mut chain = BlockChain::new();
    let block = chain.last_block();
    println!("{:?}", calculate_hash(&block));
    chain.new_transaction(String::from("shayne"), String::from("bob"), 5);
    chain.new_transaction(String::from("thomas"), String::from("wayne"), 51);
    println! {"{:?}", chain}
}
