use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};

// Get list of games
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{id}")]
async fn get_game(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Game {}", id))
}

pub fn games_cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/games")
        .service(index)
        .service(get_game)
    );
}