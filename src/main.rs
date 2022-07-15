mod games;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .configure(crate::games::api::games_cfg)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
