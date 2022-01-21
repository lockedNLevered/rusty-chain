WASM/Rust Proof of Work Blockchain served throught Node.js


# How to run

# Compiling Web Assmebly

You need Rust installed on your computer [Install Rust](https://www.rust-lang.org/tools/install)

from the root directory

``` ruby
cd wasm_blockchain
wasm-bindgen --target nodejs 
```
This should generate a pkg folder with WASM and JS Bindings

# Server

from the root directory
``` ruby 
cd server
npm i
npm run dev
```
This should start an Express server on port 4000

# Client
from the root directory

``` ruby 
cd client
npm i
npm start
```

This should start a React application on port 3000

## Tech Stack
- Rust
- TypeScript
- React
- Express

# Future 

Currently there is no node registration, this is something I would like to add at some point in the future
