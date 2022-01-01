use wasm_bindgen::prelude::*;
use console_error_panic_hook;
use sha2::{Digest, Sha256};
//log rust panics to browser console
#[allow(dead_code)]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

//Hash a block with sha256
pub fn hash_field_or_struct<T: serde::Serialize>(block: &T) -> String {
    let encoded: Vec<u8> = bincode::serialize(block).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(encoded);
    return format!("{:X}", hasher.finalize());
}