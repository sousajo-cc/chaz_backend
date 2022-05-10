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
use crate::database::schema::scores::difficulty;
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
fn get_username_global_score_history(name: String) -> Result<Json<Vec<Score>>, BackendErr> {
    let connection = establish_connection();
    let scores = Score::find(&connection, name)?;
    Ok(Json(scores))
}

#[get("/user/<name>/lvl/<level>/difficulty/<diff>")]
fn get_username_scores(
    name: String,
    level: String,
    diff: String,
) -> Result<Json<Vec<Score>>, BackendErr> {
    let connection = establish_connection();
    let mut filtered = Score::find(&connection, name)?
        .into_iter()
        .filter(|score| score.level == level && score.difficulty == diff.to_uppercase())
        .collect::<Vec<Score>>();
    filtered.sort_by(|s1, s2| s1.cmp(s2));
    Ok(Json(filtered))
}

//TODO: needs to be secured, otherwise anyone will be able to post any score
#[post("/", format = "application/json", data = "<score>")]
fn post_score(
    score: Json<Score>,
) -> Result<Json<usize>, BackendErr> {
    let result = score.insert(&establish_connection())?;
    Ok(Json(result))
}

// TESTS
pub fn fake_scores_example() -> Result<(), BackendErr> {
    let connection = establish_connection();
    let fake_data = vec![
        Score {
            id: 0,
            high_score: 100000,
            username: "th3gr34tw4rr10r".to_string(),
            difficulty: "EASY".to_string(),
            level: "1".to_string(),
        },
        Score {
            id: 1,
            high_score: 100000,
            username: "thEBeSTPLAyaaAA".to_string(),
            difficulty: "EASY".to_string(),
            level: "1".to_string(),
        },
        Score {
            id: 2,
            high_score: 100000,
            username: "0111111_w".to_string(),
            difficulty: "HARD".to_string(),
            level: "1".to_string(),
        },
        Score {
            id: 3,
            high_score: 100000,
            username: "LEEETZOR".to_string(),
            difficulty: "MEDIUM".to_string(),
            level: "1".to_string(),
        },
        Score {
            id: 4,
            high_score: 100000,
            username: "n00b".to_string(),
            difficulty: "ZATOICHI".to_string(),
            level: "1".to_string(),
        },
        Score {
            id: 5,
            high_score: 100000,
            username: "bh0t".to_string(),
            difficulty: "EASY".to_string(),
            level: "1".to_string(),
        },
        Score {
            id: 6,
            high_score: 100010,
            username: "thEBeSTPLAyaaAA".to_string(),
            difficulty: "EASY".to_string(),
            level: "1".to_string(),
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
    use rocket::http::Method;
    use rocket_cors::{AllowedOrigins, CorsOptions};

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true)
        .to_cors().unwrap();

    let cfg = rocket::config::Config::build(rocket::config::Environment::Development)
        .address("127.0.0.1")
        .port(8001)
        .unwrap();
    rocket::custom(cfg)
        .attach(cors)
        .mount(
            "/highscores",
            routes![
                list,
                get_username_global_score_history,
                insert_batch,
                get_username_scores,
                post_score,
            ],
        )
        .launch();
}
