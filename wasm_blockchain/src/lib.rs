use console_error_panic_hook;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;
//ensure compiler does not reorder struct
#[repr(C)]
#[derive(Clone, Serialize, Deserialize)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: u8,
}
//ensure compiler does not reorder struct
#[repr(C)]
#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    index: usize,
    timestamp: String,
    transactions: Vec<Transaction>,
    previous_hash: String,
    proof: u8,
}
#[allow(dead_code)]
#[wasm_bindgen]
impl Block {
    pub fn get_index(self) -> usize {
        return self.index;
    }
    pub fn get_proof(self) -> u8 {
        return self.proof;
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
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
            previous_hash: "00000000000000000000000000000".to_string(),
            proof: 1,
        };
        chain.push(genesis_block);
        return BlockChain {
            chain,
            current_transactions: VecDeque::new(),
        };
    }
    pub fn get_chain(&self) -> JsValue {
        let chain = self.chain.clone();
        return JsValue::from_serde(&chain).unwrap();
    }

    fn build_js_date() -> String {
        return String::from(js_sys::Date::to_iso_string(&js_sys::Date::new_0()));
    }

    pub fn new_transaction(&mut self, sender: String, recipient: String, amount: u8) -> usize {
        let transaction = Transaction {
            sender: sender.into(),
            recipient: recipient.into(),
            amount,
        };
        self.current_transactions.push_back(transaction);
        return self.last_block().clone().get_index();
    }
    pub fn new_block(&mut self, proof: u8) {
        let block = Block {
            index: self.chain.len(),
            timestamp: BlockChain::build_js_date(),
            transactions: Vec::from(self.current_transactions.clone()),
            proof,
            previous_hash: hash_field_or_struct(&self.last_block()),
        };
        //Remove newly set transaction from queue
        self.current_transactions.pop_front();
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
        let guess_hash = hash_field_or_struct(&guess);
        if &guess_hash[..1].to_string() == "1" {
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

pub trait HashableObject {}
impl HashableObject for Block {}
impl HashableObject for u8 {}

//Hash a block with sha256
fn hash_field_or_struct<T: serde::Serialize>(block: &T) -> String {
    let encoded: Vec<u8> = bincode::serialize(block).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(encoded);
    return format!("{:X}", hasher.finalize());
}
