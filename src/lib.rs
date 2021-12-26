use console_error_panic_hook;
use js_sys;
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use wasm_bindgen::prelude::*;

#[repr(C)]
#[derive(Hash, Clone)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: u8,
}
//ensure compiler does not reorder struct
#[repr(C)]
#[wasm_bindgen]
#[derive(Hash, Clone)]
pub struct Block {
    index: usize,
    timestamp: String,
    transactions: Vec<Transaction>,
    previous_hash: u64,
    proof: u8,
}

pub trait ExposeDetails {
    fn get_index(self) -> usize;
    fn get_proof(self) -> u8;
}

impl ExposeDetails for Block {
    fn get_index(self) -> usize {
        return self.index;
    }
    fn get_proof(self) -> u8 {
        return self.proof;
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    return s.finish();
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct BlockChain {
    //current_transactions is a temp vector queue for holding a transaction object until a new block is mined
    current_transactions: VecDeque<Transaction>,
    chain: Vec<Block>,
}
#[allow(dead_code)]
#[wasm_bindgen]
impl BlockChain {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut chain = Vec::new();
        //generate our initial block
        let genesis_block = Block {
            index: 0,
            timestamp: BlockChain::build_js_date(),
            transactions: Vec::new(),
            previous_hash: 0000000000,
            proof: 1,
        };
        chain.push(genesis_block);
        return BlockChain {
            chain,
            current_transactions: VecDeque::new(),
        };
    }
    fn build_js_date() -> String {
        return String::from(js_sys::Date::to_iso_string(&js_sys::Date::new_0()));
    }

    pub fn new_transaction(&mut self, sender: String, recipient: String, amount: u8) -> usize {
        let transaction = Transaction {
            sender,
            recipient,
            amount,
        };
        self.current_transactions.push_back(transaction);
        return self.last_block().clone().get_index();
    }
    pub fn new_block(&mut self, proof: u8) {
        let last_block = self.chain.last();
        let block = Block {
            index: self.chain.len(),
            timestamp: BlockChain::build_js_date(),
            transactions: Vec::from(self.current_transactions.clone()),
            proof,
            previous_hash: calculate_hash(&last_block),
        };
        //Set current transactions to a new empty queue
        self.current_transactions = VecDeque::new();
        self.chain.push(block);
    }
    pub fn last_block(&self) -> Block {
        let block_copy = self.chain.last().unwrap().clone();
        return block_copy;
    }
    //our proof of work algo requires a guess, that, when added to previous proof and hashed, generates a digit with 2 leading 1's
    pub fn proof_of_work(&self, last_proof: u8) -> u8 {
        let mut proof = 0;
        while self.validate_proof(last_proof, proof) == false {
            proof += 1;
        }
        return proof;
    }
    fn validate_proof(&self, last_proof: u8, proof: u8) -> bool {
        let guess = last_proof + proof;
        let guess_hash = calculate_hash(&guess);
        if &guess_hash.to_string()[..2] == "11" {
            return true;
        } else {
            return false;
        }
    }
}
//log rust panics to browser console
#[allow(dead_code)]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
