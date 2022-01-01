mod helpers;
use serde::{Deserialize, Serialize};

use std::collections::VecDeque;
use wasm_bindgen::prelude::*;
use helpers::{hash_field_or_struct};
//ensure compiler does not reorder struct
#[repr(C)]
#[derive(Clone, Serialize, Deserialize)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: u16,
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
    nonce: u8,
}
#[allow(dead_code)]
#[wasm_bindgen]
impl Block {
    pub fn get_index(self) -> usize {
        return self.index;
    }
    pub fn get_nonce(self) -> u8 {
        return self.nonce;
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
            nonce: 0,
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

    pub fn new_transaction(&mut self, sender: String, recipient: String, amount: u16) -> usize {
        let transaction = Transaction {
            sender: sender.into(),
            recipient: recipient.into(),
            amount,
        };
        self.current_transactions.push_back(transaction);
        return self.last_block().clone().get_index();
    }
    pub fn new_block(&mut self, nonce: u8) {
        let block = Block {
            index: self.chain.len(),
            timestamp: BlockChain::build_js_date(),
            transactions: Vec::from(self.current_transactions.clone()),
            nonce,
            previous_hash: hash_field_or_struct(&self.chain.last()),
        };
        //Remove newly set transaction from queue
        self.current_transactions.pop_front();
        self.chain.push(block);
    }
    pub fn last_block(&self) -> Block {
        let block_copy = self.chain.last().unwrap().clone();
        return block_copy;
    }
    //our proof of work algo requires a guess, that, when added to previous proof and hashed, generates a digit with 2 leading 0's
    pub fn proof_of_work(&self, last_nonce: u8) -> u8 {
        let mut nonce = 0;
        while self.validate_nonce(last_nonce, nonce) == false {
            nonce += 1;
        }
        return nonce;
    }
    fn validate_nonce(&self, last_nonce: u8, nonce: u8) -> bool {
        let guess = last_nonce + nonce;
        let guess_hash = hash_field_or_struct(&guess);
        if &guess_hash[..1].to_string() == "0" {
            return true;
        } else {
            return false;
        }
    }
}


