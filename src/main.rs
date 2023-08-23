mod config;
mod database;
mod dto;
mod handlers;
mod models;

use config::Config;
use database::Database;
use std::sync::Arc;

use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer};
use mongodb::options::ClientOptions;
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();

    //let connector = Database::new(config.clone());

    let mongodb_client = Client::with_uri_str(config.mongo.dsn)
        .await
        .expect("Failed to create MongoDB client")
        .database(&config.mongo.database);

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(HttpResponse::Ok))
            .service(
                web::scope("/developers")
                    .service(handlers::all_developers)
                    .service(handlers::find_developer)
                    .service(handlers::create_developer)
                    .service(handlers::update_developer)
                    .service(handlers::delete_developer),
            )
            .app_data(Data::new(Arc::new(mongodb_client.clone())))
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
