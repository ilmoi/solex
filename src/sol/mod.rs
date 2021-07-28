use std::{fs, fs::File};

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signer::{
        keypair::{read_keypair_file, Keypair},
        Signer,
    },
    system_instruction::create_account,
    system_program,
    system_transaction::transfer,
    transaction::Transaction,
};

pub fn create_new_account(client: &RpcClient, payer: &Keypair) -> Keypair {
    let new_keypair = solana_sdk::signature::Keypair::new();
    store_keypair(&new_keypair);

    let rent = client.get_minimum_balance_for_rent_exemption(0).unwrap();
    let create_account_ix = create_account(
        &payer.pubkey(),
        &new_keypair.pubkey(),
        rent,
        0,
        &system_program::id(),
    );

    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &new_keypair], //need to pass BOTH payer and new_keypair - https://docs.rs/solana-sdk/1.7.8/solana_sdk/system_instruction/enum.SystemInstruction.html#variant.CreateAccount
        client.get_recent_blockhash().unwrap().0,
    );

    // let tx_hash = client.async_send_transaction(tx).unwrap(); //async
    let tx_hash = client.send_and_confirm_transaction(&tx).unwrap(); //sync
    println!(
        "account {} successfully created, {}",
        new_keypair.pubkey(),
        tx_hash
    );

    let account = client.get_account(&new_keypair.pubkey()).unwrap();
    println!("account is {:#?}", account);

    new_keypair
}

pub fn store_keypair(kp: &Keypair) {
    let file = File::create(format!("keys/{}.json", kp.pubkey())).unwrap();
    serde_json::to_writer(&file, &kp.to_bytes()[..]).unwrap();
}

pub fn load_all_accounts() -> Vec<Keypair> {
    let paths = fs::read_dir("./keys").unwrap();
    paths
        .map(|p| {
            let p = p.unwrap().path();
            read_keypair_file(p).unwrap()
        })
        .collect()
}

pub fn transfer_lamports(client: &RpcClient, source: &Keypair, dest: &Pubkey, lamports: u64) {
    let tx = transfer(
        source,
        dest,
        lamports,
        client.get_recent_blockhash().unwrap().0,
    );

    let tx_hash = client.send_and_confirm_transaction(&tx).unwrap();
    println!(
        "transfer of {} lamports from {} to {} successfully executed, {}",
        lamports,
        source.pubkey(),
        dest,
        tx_hash
    );
}
