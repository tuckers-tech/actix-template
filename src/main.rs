mod server_config;
mod errors;
mod models;
mod db;
mod handlers;

use ::config::Config;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::add_user;
use tokio_postgres::NoTls;

use crate::server_config::DatabaseConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: DatabaseConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/users").route(web::post().to(add_user)))
    })
        .bind(config.server_addr.clone())?
        .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}