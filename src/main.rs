mod block_chain;
use crate::block_chain::BlockChain;
use crate::block_chain::ExposeDetails;

fn main() {
    let mut chain = BlockChain::new();
    chain.new_transaction(String::from("shayne"), String::from("bob"), 5);
    let last_proof = chain.last_block().clone().get_proof();
    let proof = chain.proof_of_work(last_proof);
    println! {"{}", proof}
    chain.new_block(proof);
    println! {"{:?}", chain}
}
