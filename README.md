# About

Hyper-dirty MVP integrating solana into an exchange.

Built following [these guidelines](https://docs.solana.com/integrations/exchange).

# Setup
```shell
# install solana cli first if you don't have it
# install vue cli first if you don't have it

# configure solana cli
solana config set --url https://api.devnet.solana.com

# start the backend
cargo run

# start the frontend (in a separate terminal window)
cd solex-front
yarn
yarn serve
```
Then go to `http://localhost:8080` and click buttons. 

# Usage
Sending tokens:
1. enter the destination address above. It must be the SOL address, even for tokens (ie not derived address)
2. enter the amount in lamports / raw u128 for tokens
3. hit send next to the sol account / token FROM which the tx should happen

To create / mint tokens use [this UI](https://www.spl-token-ui.com/#/).

Rest should be self-explanatory.

# Codebase

There's a bunch of to-dos scattered throughout the code with questions that I had.

At some point I thought about hooking up postgres, but then decided against it. The code is still there but commented out.

The big question to be solved is about updating token balances. 

[Solana's docs](https://docs.solana.com/integrations/exchange#poll-for-blocks) recommend that you poll for blocks and extract relevant addresses, then update their balance. I wasn't sure how to handle this on the backend (where do you store the updated balance? in postgres? but then you're duplicating the blockchain in another database?). 

You'd also need to implement a background worker to actually scan the blocks (and do it quickly enough). For now I've implemented a simple multi-threaded solution using the [clokwerk crate](https://crates.io/crates/clokwerk). Uncomment line 55 in `main` to see it in action.

Then there's the issue of async calls. Solana's sdk basically doesn't support async (I was told so in discord) so all the calls have to be blocking. Because I'm using an async actix runtime here, I had to constantly spawn a fresh thread to get around this limitation. Otherwise you get an error from `tokio` saying you can't do a blocking call inside an async runtime.

Finally, for this implementation I just connected to Solana's node for devnet. Setting up your own node isn't too hard (explained in the guidelines) and it catches up pretty quickly (15min or so). You'll need a powerful enough machine (I used `m5.8xlarge` on aws with no problems) + you'll have to update the urls in the codebase. You'd do that in the `connect` function in `utils`.