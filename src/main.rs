use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
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
#[derive(Debug, Clone)]
struct BlockChain {
    //current_transactions is a temp vector queue for holding a transaction object until a new block is mined
    current_transactions: VecDeque<Transaction>,
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
            current_transactions: VecDeque::new(),
        }
    }
    fn new_transaction(&mut self, sender: String, recipient: String, amount: u8) -> usize {
        let transaction = Transaction {
            sender,
            recipient,
            amount,
        };
        self.current_transactions.push_back(transaction);
        let next_block = self.last_block().clone().get_index();
        return next_block;
    }

    fn new_block(&mut self, proof: u8) {
        let last_block = self.chain.last();
        let block = Block {
            index: self.chain.len(),
            timestamp: OffsetDateTime::now_utc(),
            transactions: Vec::from(self.current_transactions.clone()),
            proof,
            previous_hash: calculate_hash(&last_block),
        };
        //Set current transactions to a new empty queue
        self.current_transactions = VecDeque::new();
        self.chain.push(block);
    }
    fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
}

fn main() {
    let mut chain = BlockChain::new();
    chain.new_transaction(String::from("shayne"), String::from("bob"), 5);
    chain.new_block(1);
    println! {"{:?}", chain}
}
