use std::fs::File;
//use std::io::prelude::*;
use std::path::Path;

use super::{game_list, game};

use actix_web::{get, web, Responder, Result, error, http::{StatusCode, header::ContentType}, HttpResponse};
use derive_more::{Display, Error};
use regex::Regex;
use serde::{Serialize, Deserialize};

const GAME_LIST_FILE_PATH: &'static str = "json/games/game_list.json";

#[derive(Debug, Display, Error, Serialize, Deserialize)]
enum Errors {
    #[display(fmt = "Game not found")]
    NotFound,
    #[display(fmt = "Internal error")]
    InternalServerError,
}

impl error::ResponseError for Errors {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Errors::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Errors::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

// Get list of games
#[get("/")]
async fn index() -> Result<impl Responder, Errors> {
    let path_to_file = Path::new(GAME_LIST_FILE_PATH);
    let game_list = File::open(&path_to_file);
    let game_list = match game_list {
        Ok(file) => file,
        Err(_) => return Err(Errors::InternalServerError),
    };
    let game_list = serde_json::from_reader(game_list);
    let game_list: game_list::GameList = match game_list {
        Ok(f) => f,
        Err(_) => return Err(Errors::InternalServerError),
    };

    Ok(web::Json(game_list))
}

#[get("/{id}")]
async fn get_game(id: web::Path<String>) -> Result<impl Responder, Errors> {
    let path_to_file = Path::new(GAME_LIST_FILE_PATH);
    let game_list = File::open(&path_to_file);
    let game_list = match game_list {
        Ok(file) => file,
        Err(_) => return Err(Errors::InternalServerError),
    };
    let game_list = serde_json::from_reader(game_list);
    let game_list: game_list::GameList = match game_list {
        Ok(f) => f,
        Err(_) => return Err(Errors::InternalServerError),
    };

    let id = id.into_inner();
    let game = game_list.games.iter().find(|g| g.id == id).clone();
    let game = match game {
        Some(g) => g.clone(),
        None => return Err(Errors::NotFound),
    };

    let re = Regex::new(r"^(json/games/data/)(\w{1,})(.json)$");
    let re = match re {
        Ok(r) => r,
        Err(_) => return Err(Errors::InternalServerError),
    };

    if re.is_match(&game.data) {
        let path_to_file = Path::new(&game.data);
        let file = File::open(path_to_file);
        let file = match file {
            Ok(f) => f,
            Err(_) => return Err(Errors::InternalServerError),
        };
        let game = serde_json::from_reader(file);
        let game: game::Game = match game {
            Ok(g) => g,
            Err(_) => return Err(Errors::InternalServerError),
        };

        Ok(web::Json(game))
    } else {
        Err(Errors::InternalServerError)
    }

    
}

pub fn games_cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/games")
        .service(index)
        .service(get_game)
    );
}
