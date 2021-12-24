use time::OffsetDateTime;
#[derive(Debug)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: u8,
}

#[derive(Debug)]
struct Block {
    index: u8,
    timestamp: OffsetDateTime,
    transactions: Vec<Transaction>,
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

    fn new_block(self) {}
}

fn main() {
    let chain = BlockChain::new();
    println! {"{:?}", chain}
}
