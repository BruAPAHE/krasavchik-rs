use std::sync::Arc;

use actix_web::{delete, get, HttpResponse, patch, post, web};
use actix_web::http::StatusCode;
use bson::doc;
use mongodb::Database;

use crate::dto::developer::Developer;
use crate::models::developer::Developer as DeveloperModel;

#[get("")]
async fn all_developers() -> HttpResponse {
    HttpResponse::Ok().body("Ok")
}

#[get("/{id}")]
async fn find_developer() -> HttpResponse {
    HttpResponse::Ok().body("Ok")
}

#[post("")]
async fn create_developer(
    developer: web::Json<Developer>,
    data: web::Data<Arc<Database>>,
) -> HttpResponse {
    let object_id = data
        .collection("developers")
        .insert_one(developer, None)
        .await
        .unwrap();
    match data
        .collection::<DeveloperModel>("developers")
        .find_one(doc! {"_id": &object_id.inserted_id}, None)
        .await
    {
        Ok(Some(result)) => HttpResponse::build(StatusCode::CREATED).json(result),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[patch("/{id}")]
async fn update_developer(_data: web::Data<Arc<Database>>) -> HttpResponse {
    HttpResponse::Ok().body("Ok")
}

#[delete("/{id}")]
async fn delete_developer() -> HttpResponse {
    HttpResponse::Ok().body("Ok")
}
