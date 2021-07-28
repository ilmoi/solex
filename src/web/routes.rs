use std::{ops::Deref, str::FromStr, sync::Arc, thread};

use actix_web::{get, post, web, HttpResponse, Responder};
use solana_sdk::{
    pubkey::Pubkey,
    signer::{keypair::read_keypair_file, Signer},
};

use crate::{
    sol::{create_new_account, load_all_accounts, transfer_lamports},
    spl::{
        get_associated_token_account, get_token_balance, get_token_mint_accounts, transfer_tokens,
    },
    utils::{connect, Config},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Balances {
    sol_pubkey: String,
    sol_balance: u64,
    token_balances: Vec<Token>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Token {
    token_mint_addr: String,
    token_assoc_addr: String,
    token_balance: u64,
}

#[get("/health")]
pub async fn health(config: web::Data<Arc<Config>>) -> impl Responder {
    let config = config.as_ref().deref();
    println!("config: {:?}", config);
    HttpResponse::Ok().body("health ok!")
}

#[get("/accounts")]
pub async fn accounts(config: web::Data<Arc<Config>>) -> impl Responder {
    //todo asked for a better solution in discord
    let h = thread::spawn(move || {
        let config = config.as_ref().deref();
        let client = connect(&config.json_rpc_url);
        let accounts = load_all_accounts();

        accounts
            .iter()
            .map(|a| {
                let mut token_balances = vec![];

                let token_mints = get_token_mint_accounts();
                for token_mint_addr in token_mints {
                    let token_assoc_addr =
                        get_associated_token_account(&token_mint_addr, &a.pubkey());
                    let token_balance = get_token_balance(&client, &token_assoc_addr);
                    token_balances.push(Token {
                        token_mint_addr: token_mint_addr.to_string(),
                        token_assoc_addr: token_assoc_addr.to_string(),
                        token_balance,
                    })
                }

                Balances {
                    sol_pubkey: a.pubkey().to_string(),
                    sol_balance: client.get_balance(&a.pubkey()).unwrap(),
                    token_balances,
                }
            })
            .collect::<Vec<Balances>>()
    });
    let balances = h.join().unwrap();

    HttpResponse::Ok().json(balances)
}

#[get("/create")]
pub async fn create(config: web::Data<Arc<Config>>) -> impl Responder {
    //todo asked for a better solution in discord
    let h = thread::spawn(move || {
        let config = config.as_ref().deref();
        let client = connect(&config.json_rpc_url);
        create_new_account(&client, &config.payer);
    });
    h.join().unwrap();

    HttpResponse::Ok()
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Info {
    from_pubkey: String,
    to_pubkey: String,
    amount: u64,
}

#[post("/transfer")]
pub async fn transfer(config: web::Data<Arc<Config>>, info: web::Json<Info>) -> impl Responder {
    //todo asked for a better solution in discord
    let h = thread::spawn(move || {
        let config = config.as_ref().deref();
        let client = connect(&config.json_rpc_url);
        let dest = Pubkey::from_str(&info.to_pubkey).unwrap();
        let source = read_keypair_file(format!("./keys/{}.json", info.from_pubkey)).unwrap();
        transfer_lamports(&client, &source, &dest, info.amount);
    });
    h.join().unwrap();

    HttpResponse::Ok()
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TokenInfo {
    mint_addr: String,
    from_pubkey: String,
    to_pubkey: String,
    amount: u64,
}

#[post("/transfer_tokens")]
pub async fn transfer_spl_tokens(
    config: web::Data<Arc<Config>>,
    info: web::Json<TokenInfo>,
) -> impl Responder {
    //todo asked for a better solution in discord
    let h = thread::spawn(move || {
        let config = config.as_ref().deref();
        let client = connect(&config.json_rpc_url);
        let dest = Pubkey::from_str(&info.to_pubkey).unwrap();
        let token_mint = Pubkey::from_str(&info.mint_addr).unwrap();
        let source = read_keypair_file(format!("./keys/{}.json", info.from_pubkey)).unwrap();
        transfer_tokens(&client, &token_mint, &source, &dest, info.amount);
    });
    h.join().unwrap();

    HttpResponse::Ok()
}
