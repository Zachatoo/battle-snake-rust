#[macro_use]
extern crate rocket;

use log::info;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::Value;
use std::env;

use crate::request::GameState;

mod auth;
mod graph;
mod logic;
mod movement_set;
mod request;

#[get("/")]
fn handle_index(_key: auth::ApiKey<'_>) -> Json<Value> {
    Json(logic::info())
}

#[post("/start", format = "json", data = "<start_req>")]
fn handle_start(start_req: Json<GameState>, _key: auth::ApiKey<'_>) -> Status {
    logic::start(
        &start_req.game,
        &start_req.turn,
        &start_req.board,
        &start_req.you,
    );

    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move(move_req: Json<GameState>, _key: auth::ApiKey<'_>) -> Json<Value> {
    let response = logic::get_move(
        &move_req.game,
        &move_req.turn,
        &move_req.board,
        &move_req.you,
    );

    Json(response)
}

#[post("/end", format = "json", data = "<end_req>")]
fn handle_end(end_req: Json<GameState>, _key: auth::ApiKey<'_>) -> Status {
    logic::end(&end_req.game, &end_req.turn, &end_req.board, &end_req.you);

    Status::Ok
}

#[launch]
fn rocket() -> _ {
    // Lots of web hosting services expect you to bind to the port specified by the `PORT`
    // environment variable. However, Rocket looks at the `ROCKET_PORT` environment variable.
    // If we find a value for `PORT`, we set `ROCKET_PORT` to that value.
    if let Ok(port) = env::var("PORT") {
        env::set_var("ROCKET_PORT", &port);
    }

    // We default to 'info' level logging. But if the `RUST_LOG` environment variable is set,
    // we keep that value instead.
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    info!("Starting Battlesnake Server...");

    rocket::build()
        .attach(AdHoc::on_response("Server ID Middleware", |_, res| {
            Box::pin(async move {
                res.set_raw_header("Server", "zachatoo/battle-snake-rust");
            })
        }))
        .mount(
            "/",
            routes![handle_index, handle_start, handle_move, handle_end],
        )
}
