mod games;

use actix_web::{App, HttpServer};
use actix_cors::Cors;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET"]);

        App::new()
        .wrap(cors)
        .configure(crate::games::api::games_cfg)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
