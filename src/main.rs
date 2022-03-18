#![feature(proc_macro_hygiene, decl_macro)]
mod database;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;
#[macro_use]
extern crate serde_derive;

// in v0.5.1-rc Json is part of rocket
// https://stackoverflow.com/questions/68682054/how-to-return-json-as-a-response-in-rust-rocket-with-auto-field-deserialising
// but we are still using 0.4.10 for now
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use crate::database::models::Score;


#[derive(Responder, Debug)]
pub enum BackendErr {
    #[response(status = 403)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
}


#[get("/list")]
fn list() -> Result<Json<Vec<Score>>, BackendErr> {
    Ok(Json(vec![Score {
        id: 0,
        high_score: 100000,
        username: "BestPlayer".to_string(),
    }]))
}

#[get("/user/<name>")]
fn get_by_username(name: String) -> Result<Json<Vec<Score>>, BackendErr> {
    Ok(Json(vec![Score {
        id: 0,
        high_score: 100000,
        username: "BestPlayer".to_string(),
    }]))
}

fn main() {
    rocket::ignite()
        .mount("/highscores", routes![list, get_by_username])
        .launch();
}
