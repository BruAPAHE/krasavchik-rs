use actix_web::{get, post, HttpResponse, patch, delete};

#[get("/")]
async fn all_developers()->HttpResponse{
    HttpResponse::Ok().body("Ok")
}

#[get("/{id}")]
async fn find_developer()->HttpResponse{
    HttpResponse::Ok().body("Ok")
}


#[post("/")]
async fn create_developer()->HttpResponse{
    HttpResponse::Ok().body("Ok")
}

#[patch("/{id}")]
async fn update_developer()->HttpResponse{
    HttpResponse::Ok().body("Ok")
}

#[delete("/{id}")]
async fn delete_developer()->HttpResponse{
    HttpResponse::Ok().body("Ok")
}