import express from "express";
import * as wasm from "../../pkg/rusty_chain";

const app = express();
//Initialize logging of rust panics
wasm.init_panic_hook();
//create a new block chain on launch
const chain = new wasm.BlockChain();

console.log(chain.get_chain());
console.log(chain.proof_of_work(chain.last_block().get_proof()));
app.get("/", (_, res) => {
	res.json(chain.get_chain());
});

app.listen(3000, () => console.log("live"));
