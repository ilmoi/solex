//! actix runs on tokio's async runtime
//! but I was told in solana discord that there's no async interface for RpcClient
//! I was also told not to use ThinClient, ever, even though it seems to have an async interface
//!
//! That's why you see thread::spawn() scattered throughout the code,
//! otherwise RpcClient tries to do a blocking request on the only thread actix has
//! Which leads to this error `can call blocking only when running on the multi-threaded runtime`

use std::sync::Arc;

use core::mem;
use solana_sdk::pubkey::Pubkey;
use solex::utils::{clokwerk_sync_scheduler, connect, get_confirmed_block};
use solex::{utils::load_config, web::server::run_server};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::str::FromStr;

#[actix_web::main]
async fn main() {
    // ----------------------------------------------------------------------------- psql

    /*
    todo Storing balances in local db vs not?
    Pro
    - Available for matching engine, fast
    - Available for FE, fast
    Con
    - replicating the bc, seems stupid
    - need to make sure the two are in sync, probably need some sort of repair mechanism (what if you missed a block?)
     */

    // // configure sqlx connection
    // let conn_options = PgConnectOptions::new()
    //     .host("localhost")
    //     .username("postgres")
    //     .password("dbpw")
    //     .port(5432)
    //     .database("dolex");
    //
    // // get a connection pool
    // let pg_pool = PgPoolOptions::new()
    //     .connect_timeout(std::time::Duration::from_secs(60)) //on purpose setting longer to avoid sqlx PoolTimedOut
    //     .connect_with(conn_options)
    //     .await
    //     .expect("failed to connect to Postgres");
    // let arc_pg_pool = Arc::new(pg_pool);

    // ----------------------------------------------------------------------------- sol-specific
    solana_logger::setup_with_default("solana=debug");

    //todo not sure if I should be Arc'ing the client or the config - for mvp keep as is
    let config = load_config();
    let arc_config = Arc::new(config);

    // clokwerk_sync_scheduler(arc_config.clone());

    // ----------------------------------------------------------------------------- server

    println!("listening on 5000");
    run_server("localhost:5000", arc_config)
        .unwrap()
        .await
        .unwrap();
}

// fn main() {
//     let valid_key = "12KKNE2g3Tajwe3ogAaoG63swV8GXpaxrxJyBSeMs648";
//     let invalid_key = "12KKNE2g3Tajwe3ogAaoG63swV8GXpaxrxJyBSeMs63";
//
//     println!("{}", mem::size_of::<Pubkey>());
//
//     let valid_key2 = "CR1dEeFZVnFiNmZCekREGdT3N7dxVDFZe3kr44oX8TpQ";
//     let p = Pubkey::from_str(invalid_key).unwrap();
//     println!("{:?}, {}", p, p.is_on_curve());
// }
