mod config;
mod database;
mod handlers;

use std::sync::Arc;
use config::Config;
use database::Database;

use actix_web::{web, App, HttpResponse, HttpServer, guard};
use mongodb::Client;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();

    let connector = Database::new(config.clone());
    let client_options =
        ClientOptions::parse(config.mongo.dsn)
            .unwrap();
    let client = Client::with_options(client_options)
        .unwrap()
        .database(&config.mongo.database);

    HttpServer::new(move || App::new()
        .route("/", web::get().to(HttpResponse::Ok))
        .service(
            web::scope("/developers")
                .service(handlers::all_developers)
                .service(handlers::find_developer)
                .service(handlers::create_developer)
                .service(handlers::update_developer)
                .service(handlers::delete_developer)
        )
        .app_data(Arc::new(client.clone()))
    )
        .bind((config.app.hostname, config.app.port))?
        .run()
        .await
}
