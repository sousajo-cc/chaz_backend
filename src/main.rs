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
use crate::database::models::score::Score;
use database::establish_connection;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Responder, Debug, Serialize)]
pub enum BackendErr {
    DatabaseErr(String),
    #[response(status = 403)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
}

impl From<diesel::result::Error> for BackendErr {
    fn from(error: diesel::result::Error) -> Self {
        BackendErr::DatabaseErr(error.to_string())
    }
}

#[get("/list")]
fn list() -> Result<Json<Vec<Score>>, BackendErr> {
    let connection = establish_connection();
    let list = Score::list(&connection)?;
    Ok(Json(list))
}

#[get("/user/<name>")]
fn get_by_username(name: String) -> Result<Json<Vec<Score>>, BackendErr> {
    let connection = establish_connection();
    let scores = Score::find(&connection, name)?;
    Ok(Json(scores))
}

// TESTS
pub fn fake_scores_example() -> Result<(), BackendErr> {
    let connection = establish_connection();
    let fake_data = vec![
        Score {
            id: 0,
            high_score: 100000,
            username: "th3gr34tw4rr10r".to_string(),
        },
        Score {
            id: 1,
            high_score: 100000,
            username: "thEBeSTPLAyaaAA".to_string(),
        },
        Score {
            id: 2,
            high_score: 100000,
            username: "0111111_w".to_string(),
        },
        Score {
            id: 3,
            high_score: 100000,
            username: "LEEETZOR".to_string(),
        },
        Score {
            id: 4,
            high_score: 100000,
            username: "n00b".to_string(),
        },
        Score {
            id: 5,
            high_score: 100000,
            username: "bh0t".to_string(),
        },
    ];
    Score::insert_batch(&connection, fake_data)?;
    Ok(())
}
#[post("/test")]
fn insert_batch() -> Result<(), BackendErr> {
    let insert = fake_scores_example();
    insert
}

fn main() {
    rocket::ignite()
        .mount("/highscores", routes![list, get_by_username, insert_batch])
        .launch();
}
