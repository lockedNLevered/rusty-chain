import { useEffect, useState } from "react";
import axios from "axios";
import { Block } from "../../wasm_blockchain/pkg/rusty_chain";
import "./index.css";

const API_URL = "http://localhost:4000";

function App() {
	const [chain, setChain] = useState<Block[] | null>(null);
	const [sender, setSender] = useState("");
	const [recipient, setRecipient] = useState("");
	const [amount, setAmount] = useState(0);
	async function fetchApi() {
		const res = await axios.get(API_URL);
		setChain(res.data);
	}
	useEffect(() => {
		fetchApi();
	}, []);

	async function handleSubmit() {
		const data = {
			recipient,
			sender,
			amount,
		};

		const res = await axios.post(API_URL, data);
		console.log(res);
		return fetchApi();
	}

	console.log(chain);
	return (
		<div className="app">
			<form className="form" onSubmit={handleSubmit}>
				<label htmlFor="sender">From</label>
				<input
					value={sender}
					onChange={(e) => setSender(e.target.value)}
					name="sender"
				/>
				<label htmlFor="recipient">To</label>
				<input
					value={recipient}
					onChange={(e) => setRecipient(e.target.value)}
					name="recipient"
				/>
				<label htmlFor="amount">Amount</label>
				<input
					value={amount}
					onChange={(e) => setAmount(parseInt(e.target.value))}
					name="amount"
					type="number"
				/>
				<button type="submit">Add Transaction to the BlockChain</button>
			</form>
			<div className="data">
				{chain && <pre>{JSON.stringify(chain, null, 4)}</pre>}
			</div>
		</div>
	);
}

export default App;
