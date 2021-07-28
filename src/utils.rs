use std::fs;
use std::fs::File;
use std::io::Write;
use std::net::SocketAddr;
use std::time::Duration;

use solana_client::rpc_client::RpcClient;
use solana_client::thin_client;
use solana_client::thin_client::ThinClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signer::keypair::{Keypair, read_keypair_file};
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::create_account;
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;
use solana_sdk::client::SyncClient;

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
    Config {json_rpc_url, payer}
}

pub fn connect(json_rpc_url: &String) -> RpcClient {
    //todo this is where I'd specify a local node
    let client = RpcClient::new_with_commitment(json_rpc_url.clone(), CommitmentConfig::confirmed());
    let version = client.get_version().unwrap();
    println!("Client version is: {}", version);
    client
}

pub fn connect_thin_client() -> ThinClient {
    // taken from https://brson.github.io/2021/06/08/rust-on-solana?utm_source=Rust+in+Blockchain

    //todo this is where I'd specify a local node

    // trusted validator - dv1LfzJvDF7S1fBKpFgKoKXK5yoSosmkAdfbxBo1GqJ
    // as per https://docs.solana.com/clusters

    // curl https://api.devnet.solana.com -X POST -H "Content-Type: application/json" -d '
    //   {"jsonrpc":"2.0", "id":1, "method":"getClusterNodes"}
    // '

    let rpc_addr = "35.199.181.141:8899";
    let tpu_addr = "35.199.181.141:8004";
    let tx_port_range = (10_000_u16, 20_000_u16);
    let timeout = 1000;

    println!("connecting to solana node, RPC: {}, TPU: {}, tx range: {}-{}, timeout: {}ms",
          rpc_addr, tpu_addr, tx_port_range.0, tx_port_range.1, timeout);

    let rpc_addr: SocketAddr = rpc_addr.parse().unwrap();
    let tpu_addr: SocketAddr = tpu_addr.parse().unwrap();

    let client = thin_client::create_client_with_timeout(
        (rpc_addr, tpu_addr),
        tx_port_range,
        Duration::from_millis(timeout));

    let epoch = client.get_epoch_info().unwrap();
    println!("Epoch is: {:?}", epoch);
    client
}