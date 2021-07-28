use std::sync::Arc;

use solex::{utils::load_config, web::server::run_server};

#[actix_web::main]
async fn main() {
    solana_logger::setup_with_default("solana=debug");

    let config = load_config();
    let arc_config = Arc::new(config);

    println!("listening on 5000");
    run_server("localhost:5000", arc_config)
        .unwrap()
        .await
        .unwrap();
}
