use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{dev::Server, http, web, App, HttpServer};

use crate::{
    utils::Config,
    web::routes::{accounts, create, health, transfer, transfer_spl_tokens},
};
use sqlx::{Pool, Postgres};

pub fn run_server(
    addr: &str,
    config: Arc<Config>,
    // pg_pool: Arc<Pool<Postgres>>,
) -> Result<Server, std::io::Error> {
    let config = web::Data::new(config);
    // let pool = web::Data::new(pg_pool);

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(health)
            .service(accounts)
            .service(create)
            .service(transfer)
            .service(transfer_spl_tokens)
            .app_data(config.clone())
        // .app_data(pool.clone())
    })
    .bind(addr)?
    .run();
    Ok(server)
}
