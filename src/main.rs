mod games;

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .configure(crate::games::games_cfg)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
