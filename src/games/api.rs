use std::fs::File;
//use std::io::prelude::*;
use std::path::Path;

use super::{game_list, game};

use actix_web::{get, web, Responder, Result};

// Get list of games
#[get("/")]
async fn index() -> Result<impl Responder> {
    let game_list = get_game_list().await;

    Ok(web::Json(game_list))
}

#[get("/{id}")]
async fn get_game(id: web::Path<String>) -> Result<impl Responder> {
    let game_list = get_game_list().await;
    let id = id.into_inner();
    let game = game_list.games.iter().find(|g| g.id == id).unwrap().clone();

    let path_to_file = Path::new(&game.data);
    let file = File::open(path_to_file).unwrap();
    let game: game::Game = serde_json::from_reader(file).unwrap();

    Ok(web::Json(game))
}

pub fn games_cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/games")
        .service(index)
        .service(get_game)
    );
}

async fn get_game_list() -> game_list::GameList {
    let path_to_file = Path::new("json/games/game_list.json");
    let game_list = File::open(&path_to_file).unwrap();
    let game_list: game_list::GameList = serde_json::from_reader(game_list).unwrap();

    game_list
}