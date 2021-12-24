use time::OffsetDateTime;
#[derive(Debug)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: u8,
}

#[derive(Debug)]
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
    println!("{:?}", chain.last_block());
    println! {"{:?}", chain}
}
