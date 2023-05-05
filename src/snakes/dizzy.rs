use crate::{
    logic::{avoid_bounds, avoid_snake_bodies, scan_tail},
    movement_set::WeightedMovementSet,
    request::{Battlesnake, Board, Game},
    response::{InfoResponse, MoveShoutResponse},
};

#[cfg(test)]
use crate::rocket;
#[cfg(test)]
use rocket::{
    http::{ContentType, Status},
    local::blocking::Client,
};

pub fn info() -> InfoResponse {
    info!("INFO");
    InfoResponse {
        api_version: "1".to_string(),
        author: "Zachatoo".to_string(),
        color: "#fff947".to_string(),
        head: "caffeine".to_string(),
        tail: "curled".to_string(),
    }
}

pub fn start(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} GAME START", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} GAME OVER", game.id);
}

pub fn get_move(game: &Game, turn: &u32, board: &Board, you: &Battlesnake) -> MoveShoutResponse {
    let mut movement_set = WeightedMovementSet::new();

    avoid_bounds(board.width, board.height, you, &mut movement_set);
    avoid_snake_bodies(&board.snakes, you, &mut movement_set);
    scan_tail(&board, you, &mut movement_set);

    info!("Safe moves: {:?}", movement_set.moves);
    let chosen_move = movement_set.pick_movement().as_str().to_string();
    info!("{} MOVE {}: {}", game.id, turn, chosen_move);
    MoveShoutResponse {
        chosen_move,
        shout: movement_set
            .moves
            .into_iter()
            .map(|x| x.movement.as_str().to_owned())
            .collect::<Vec<String>>()
            .join(","),
    }
}

#[cfg(test)]
static MOVE_URI: &str = "/dizzy/move?x-api-key=valid_api_key";

#[test]
fn movement_scan_tail_down() {
    let client = Client::untracked(rocket()).expect("Failed to create client instance");
    let response = client
        .post(MOVE_URI)
        .header(ContentType::JSON)
        .body(
            r#"{
              "game": {
                "id": "unique-game-id",
                "ruleset": {
                  "name": "standard"
                },
                "timeout": 500
              },
              "turn": 10,
              "board": {
                "height": 11,
                "width": 11,
                "food": [],
                "hazards": [],
                "snakes": [
                  {
                    "id": "my-snake",
                    "name": "My Snake",
                    "health": 80,
                    "body": [
                      {"x": 3, "y": 3},
                      {"x": 2, "y": 3},
                      {"x": 2, "y": 2},
                      {"x": 3, "y": 2}
                    ],
                    "latency": "111",
                    "head": {"x": 3, "y": 3},
                    "length": 4
                  }
                ]
              },
              "you": {
                "id": "my-snake",
                "name": "My Snake",
                "health": 80,
                "body": [
                  {"x": 3, "y": 3},
                  {"x": 2, "y": 3},
                  {"x": 2, "y": 2},
                  {"x": 3, "y": 2}
                ],
                "latency": "111",
                "head": {"x": 3, "y": 3},
                "length": 4
              }
            }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let parsed_body = response
        .into_json::<MoveShoutResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "down");
}

#[test]
fn movement_scan_tail_left() {
    let client = Client::untracked(rocket()).expect("Failed to create client instance");
    let response = client
        .post(MOVE_URI)
        .header(ContentType::JSON)
        .body(
            r#"{
              "game": {
                "id": "unique-game-id",
                "ruleset": {
                  "name": "standard"
                },
                "timeout": 500
              },
              "turn": 10,
              "board": {
                "height": 11,
                "width": 11,
                "food": [],
                "hazards": [],
                "snakes": [
                  {
                    "id": "my-snake",
                    "name": "My Snake",
                    "health": 80,
                    "body": [
                      {"x": 3, "y": 2},
                      {"x": 3, "y": 3},
                      {"x": 2, "y": 3},
                      {"x": 2, "y": 2}
                    ],
                    "latency": "111",
                    "head": {"x": 3, "y": 2},
                    "length": 4
                  }
                ]
              },
              "you": {
                "id": "my-snake",
                "name": "My Snake",
                "health": 80,
                "body": [
                  {"x": 3, "y": 2},
                  {"x": 3, "y": 3},
                  {"x": 2, "y": 3},
                  {"x": 2, "y": 2}
                ],
                "latency": "111",
                "head": {"x": 3, "y": 2},
                "length": 4
              }
            }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let parsed_body = response
        .into_json::<MoveShoutResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "left");
}

#[test]
fn movement_scan_tail_up() {
    let client = Client::untracked(rocket()).expect("Failed to create client instance");
    let response = client
        .post(MOVE_URI)
        .header(ContentType::JSON)
        .body(
            r#"{
              "game": {
                "id": "unique-game-id",
                "ruleset": {
                  "name": "standard"
                },
                "timeout": 500
              },
              "turn": 10,
              "board": {
                "height": 11,
                "width": 11,
                "food": [],
                "hazards": [],
                "snakes": [
                  {
                    "id": "my-snake",
                    "name": "My Snake",
                    "health": 80,
                    "body": [
                      {"x": 2, "y": 2},
                      {"x": 3, "y": 2},
                      {"x": 3, "y": 3},
                      {"x": 2, "y": 3}
                    ],
                    "latency": "111",
                    "head": {"x": 2, "y": 2},
                    "length": 4
                  }
                ]
              },
              "you": {
                "id": "my-snake",
                "name": "My Snake",
                "health": 80,
                "body": [
                  {"x": 2, "y": 2},
                  {"x": 3, "y": 2},
                  {"x": 3, "y": 3},
                  {"x": 2, "y": 3}
                ],
                "latency": "111",
                "head": {"x": 2, "y": 2},
                "length": 4
              }
            }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let parsed_body = response
        .into_json::<MoveShoutResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "up");
}

#[test]
fn movement_scan_tail_right() {
    let client = Client::untracked(rocket()).expect("Failed to create client instance");
    let response = client
        .post(MOVE_URI)
        .header(ContentType::JSON)
        .body(
            r#"{
              "game": {
                "id": "unique-game-id",
                "ruleset": {
                  "name": "standard"
                },
                "timeout": 500
              },
              "turn": 10,
              "board": {
                "height": 11,
                "width": 11,
                "food": [],
                "hazards": [],
                "snakes": [
                  {
                    "id": "my-snake",
                    "name": "My Snake",
                    "health": 80,
                    "body": [
                      {"x": 2, "y": 3},
                      {"x": 2, "y": 2},
                      {"x": 3, "y": 2},
                      {"x": 3, "y": 3}
                    ],
                    "latency": "111",
                    "head": {"x": 2, "y": 3},
                    "length": 4
                  }
                ]
              },
              "you": {
                "id": "my-snake",
                "name": "My Snake",
                "health": 80,
                "body": [
                  {"x": 2, "y": 3},
                  {"x": 2, "y": 2},
                  {"x": 3, "y": 2},
                  {"x": 3, "y": 3}
                ],
                "latency": "111",
                "head": {"x": 2, "y": 3},
                "length": 4
              }
            }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let parsed_body = response
        .into_json::<MoveShoutResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "right");
}
