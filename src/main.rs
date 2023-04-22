#[macro_use]
extern crate rocket;

use log::info;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::{json, Value};
use std::env;

use crate::request::GameState;

mod auth;
mod fifo_queue;
mod graph;
mod logic;
mod movement_set;
mod request;
mod response;
mod snakes;

#[get("/")]
fn handle_index_rusty(_key: auth::ApiKey<'_>) -> Json<Value> {
    Json(json!(snakes::rusty::info()))
}

#[post("/start", format = "json", data = "<start_req>")]
fn handle_start_rusty(start_req: Json<GameState>, _key: auth::ApiKey<'_>) -> Status {
    snakes::rusty::start(
        &start_req.game,
        &start_req.turn,
        &start_req.board,
        &start_req.you,
    );

    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move_rusty(move_req: Json<GameState>, _key: auth::ApiKey<'_>) -> Json<Value> {
    let response = snakes::rusty::get_move(
        &move_req.game,
        &move_req.turn,
        &move_req.board,
        &move_req.you,
    );

    Json(json!(response))
}

#[post("/end", format = "json", data = "<end_req>")]
fn handle_end_rusty(end_req: Json<GameState>, _key: auth::ApiKey<'_>) -> Status {
    snakes::rusty::end(&end_req.game, &end_req.turn, &end_req.board, &end_req.you);

    Status::Ok
}

#[get("/")]
fn handle_index_righty(_key: auth::ApiKey<'_>) -> Json<Value> {
    Json(json!(snakes::righty::info()))
}

#[post("/start", format = "json", data = "<start_req>")]
fn handle_start_righty(start_req: Json<GameState>, _key: auth::ApiKey<'_>) -> Status {
    snakes::righty::start(
        &start_req.game,
        &start_req.turn,
        &start_req.board,
        &start_req.you,
    );

    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move_righty(move_req: Json<GameState>, _key: auth::ApiKey<'_>) -> Json<Value> {
    let response = snakes::righty::get_move(
        &move_req.game,
        &move_req.turn,
        &move_req.board,
        &move_req.you,
    );

    Json(json!(response))
}

#[post("/end", format = "json", data = "<end_req>")]
fn handle_end_righty(end_req: Json<GameState>, _key: auth::ApiKey<'_>) -> Status {
    snakes::righty::end(&end_req.game, &end_req.turn, &end_req.board, &end_req.you);

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

    match env_logger::try_init() {
        Ok(_) => (),
        Err(_) => (),
    }

    info!("Starting Battlesnake Server...");

    rocket::build()
        .attach(AdHoc::on_response("Server ID Middleware", |_, res| {
            Box::pin(async move {
                res.set_raw_header("Server", "zachatoo/battle-snake-rust");
            })
        }))
        .mount(
            "/",
            routes![
                handle_index_rusty,
                handle_start_rusty,
                handle_move_rusty,
                handle_end_rusty
            ],
        )
        .mount(
            "/rusty",
            routes![
                handle_index_rusty,
                handle_start_rusty,
                handle_move_rusty,
                handle_end_rusty
            ],
        )
        .mount(
            "/righty",
            routes![
                handle_index_righty,
                handle_start_righty,
                handle_move_righty,
                handle_end_righty
            ],
        )
}
