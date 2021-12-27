import express from "express";
import * as wasm from "../../pkg/rusty_chain";

interface Transaction {
	sender: string;
	reciever: string;
	amount: number;
}

const app = express();
//Initialize logging of rust panics
wasm.init_panic_hook();
//create a new block chain on launch
const chain = new wasm.BlockChain();
app.use(express.json());

app.get("/", (_, res) => {
	res.json(chain.get_chain());
});
app.post("/", (req, res) => {
	console.log(req);
	const { sender, reciever, amount }: Transaction = req.body;
	const newTransaction = chain.new_transaction(sender, reciever, amount);
	const last_proof = chain.last_block().get_proof();
	const proof = chain.proof_of_work(last_proof);
	chain.new_block(proof);
	res.status(201).json(newTransaction);
});

app.listen(3000, () => console.log("live"));
