import express from "express";
import cors from "cors";
import {
	BlockChain,
	init_panic_hook,
} from "../../wasm_blockchain/pkg/rusty_chain";
import { Transaction } from "./types";

const app = express();
//Initialize logging of rust panics
init_panic_hook();
//create a new block chain on launch and seed with genesis block

//required express middlewares
app.use(cors());
app.use(express.json());

const chain = new BlockChain();

//Endpoint to the view the blockchain -> this does not persist so it will be empty on launch
app.get("/", (_, res) => {
	res.json(chain.get_chain());
});

//Endpoint to create a new transaction -> for simplicity, this endpoint
//also mines a new block via our proof of work algorithm
app.post("/", (req, res) => {
	const { sender, recipient, amount }: Transaction = req.body;
	const newTransaction = chain.new_transaction(sender, recipient, amount);
	const nonce = chain.proof_of_work(chain.last_block().get_nonce());
	chain.new_block(nonce);
	res.status(201).json(newTransaction);
});

app.listen(4000, () => console.log("live"));
