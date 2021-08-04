use std::ops::Deref;
use std::sync::Arc;
use std::{net::SocketAddr, thread, time::Duration};

use clokwerk::{AsyncScheduler, Scheduler, TimeUnits};
use solana_client::{rpc_client::RpcClient, thin_client, thin_client::ThinClient};
use solana_sdk::signer::Signer;
use solana_sdk::{
    client::SyncClient,
    commitment_config::CommitmentConfig,
    signer::keypair::{read_keypair_file, Keypair},
};
use solana_transaction_status::{EncodedTransaction, UiMessage};

use crate::sol::load_all_accounts;

#[derive(Debug)]
pub struct Config {
    pub json_rpc_url: String,
    pub payer: Keypair,
}

pub fn load_config() -> Config {
    // returns standard path to config file
    let config_file_path = solana_cli_config::CONFIG_FILE.as_ref().unwrap();
    let cli_config = solana_cli_config::Config::load(&config_file_path).unwrap(); //alternatively Config::default()
    let json_rpc_url = cli_config.json_rpc_url;
    let payer = read_keypair_file(&cli_config.keypair_path).unwrap();
    Config {
        json_rpc_url,
        payer,
    }
}

pub fn connect(json_rpc_url: &String) -> RpcClient {
    //todo this is where I'd specify a local node
    let client =
        RpcClient::new_with_commitment(json_rpc_url.clone(), CommitmentConfig::confirmed());
    let version = client.get_version().unwrap();
    println!("Client version is: {}", version);
    client
}

// was told not to use
// pub fn connect_thin_client() -> ThinClient {
//     // taken from https://brson.github.io/2021/06/08/rust-on-solana?utm_source=Rust+in+Blockchain
//
//     // trusted validator - dv1LfzJvDF7S1fBKpFgKoKXK5yoSosmkAdfbxBo1GqJ
//     // as per https://docs.solana.com/clusters
//
//     // curl https://api.devnet.solana.com -X POST -H "Content-Type: application/json" -d '
//     //   {"jsonrpc":"2.0", "id":1, "method":"getClusterNodes"}
//     // '
//
//     let rpc_addr = "35.199.181.141:8899";
//     let tpu_addr = "35.199.181.141:8004";
//     let tx_port_range = (10_000_u16, 20_000_u16);
//     let timeout = 1000;
//
//     println!(
//         "connecting to solana node, RPC: {}, TPU: {}, tx range: {}-{}, timeout: {}ms",
//         rpc_addr, tpu_addr, tx_port_range.0, tx_port_range.1, timeout
//     );
//
//     let rpc_addr: SocketAddr = rpc_addr.parse().unwrap();
//     let tpu_addr: SocketAddr = tpu_addr.parse().unwrap();
//
//     let client = thin_client::create_client_with_timeout(
//         (rpc_addr, tpu_addr),
//         tx_port_range,
//         Duration::from_millis(timeout),
//     );
//
//     let epoch = client.get_epoch_info().unwrap();
//     println!("Epoch is: {:?}", epoch);
//     client
// }

pub fn get_confirmed_block(config: Arc<Config>) {
    let client = connect(&config.as_ref().json_rpc_url);

    // todo this is to be replaced by the last processed slot stored in db
    let current_slot = client.get_slot().unwrap();
    println!("Current slot is {}", current_slot);

    // todo this is to be replaced by a call to db to get all relevant accounts
    let relevant_accounts = load_all_accounts();
    let relevant_accounts_str = relevant_accounts
        .iter()
        .map(|a| a.pubkey().to_string())
        .collect::<Vec<String>>();

    // get all the blocks since a given slot
    let blocks = client
        // assuming slot happens every 500ms, we call this function every 5s, so need to capture at least 10, to be safe 12
        // for some weird reason if you specify start_slot too close to end slot it returns ~130 instead of required amount
        // 50 seems to work correctly, but I still trim the array to just last 12
        .get_blocks_with_commitment(
            current_slot - 50,
            Some(current_slot),
            CommitmentConfig::finalized(),
        )
        .unwrap();
    let blocks = &blocks[blocks.len() - 12..];
    println!("Blocks since last slot: {:?}", blocks);

    let mut handles = vec![];
    let arc_relevant_accounts_str = Arc::new(relevant_accounts_str);
    let arc_client = Arc::new(client);

    // for each block pull out all the accounts involved in transactions
    for &b in blocks {
        // copy the pointer for each block
        let relevant_accounts_clone = arc_relevant_accounts_str.clone();
        let client_clone = arc_client.clone();

        // 2 problems here:
        // 1)when using solana's node, if you query it too often you get panics - so I had to add a min delay
        // 2)since the blocking part here is the network requrest, ideally we'd using tokio, but RpcClient doesn't support async
        thread::sleep(Duration::new(0, 10_000_000));
        let h = thread::spawn(move || {
            // unpack the pointer
            let relevant_accounts_str = relevant_accounts_clone.as_ref();
            let client = client_clone.as_ref();

            let block = client.get_block(b).unwrap();
            println!("{}, {}", b, block.blockhash);

            for transaction in &block.transactions {
                //this holds the accounts involved in transactions in a given block
                let mut account_keys = &vec![];

                if let EncodedTransaction::Json(ui_tx) = &transaction.transaction {
                    if let UiMessage::Raw(raw_msg) = &ui_tx.message {
                        account_keys = &raw_msg.account_keys;
                    }
                }

                // if any of the accounts match our relevant accounts, then update balances
                for key in account_keys {
                    if relevant_accounts_str.contains(key) {
                        let pre_balances = &transaction.meta.as_ref().unwrap().pre_balances;
                        let post_balances = &transaction.meta.as_ref().unwrap().post_balances;
                        println!(
                            "{:#?}, {:#?}, {:#?}",
                            account_keys, pre_balances, post_balances
                        );
                        // todo
                        //  1 - confirm with the guys if using the db is indeed right here
                        //  2 - test with normal accounts
                        //  3 - test with spl accounts
                    }
                }
            }
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }
}

pub fn clokwerk_sync_scheduler(config: Arc<Config>) {
    let handle = thread::spawn(move || {
        let mut scheduler = Scheduler::new();

        scheduler.every(5.seconds()).run(move || {
            println!("working!");
            get_confirmed_block(config.clone())
        });

        loop {
            scheduler.run_pending();
            thread::sleep(Duration::from_millis(10));
        }
    });

    // handle.join().unwrap(); DO NOT JOIN! OR IT WILL BLOCK THE SERVER
}

pub fn validate_sol_address() {}
