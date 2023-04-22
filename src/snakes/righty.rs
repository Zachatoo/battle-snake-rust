use crate::{
    request::{Battlesnake, Board, Game},
    response::{InfoResponse, MoveResponse},
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
        color: "#000000".to_string(),
        head: "dead".to_string(),
        tail: "curled".to_string(),
    }
}

pub fn start(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} GAME START", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} GAME OVER", game.id);
}

pub fn get_move(game: &Game, turn: &u32, _board: &Board, _you: &Battlesnake) -> MoveResponse {
    let chosen_move = "right".to_string();
    info!("{} MOVE {}: {}", game.id, turn, chosen_move);
    MoveResponse { chosen_move }
}

#[cfg(test)]
static MOVE_URI: &str = "/righty/move?x-api-key=valid_api_key";

#[test]
fn movement_move_right() {
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
                "turn": 0,
                "board": {
                  "height": 11,
                  "width": 11,
                  "food": [],
                  "hazards": [],
                  "snakes": [
                    {
                      "id": "my-snake",
                      "name": "My Snake",
                      "health": 54,
                      "body": [
                        {"x": 3, "y": 3},
                        {"x": 3, "y": 3},
                        {"x": 3, "y": 3}
                      ],
                      "latency": "111",
                      "head": {"x": 3, "y": 3},
                      "length": 3
                    }
                  ]
                },
                "you": {
                  "id": "my-snake",
                  "name": "My Snake",
                  "health": 54,
                  "body": [
                    {"x": 3, "y": 3},
                    {"x": 3, "y": 3},
                    {"x": 3, "y": 3}
                  ],
                  "latency": "111",
                  "head": {"x": 3, "y": 3},
                  "length": 3
                }
              }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let parsed_body = response
        .into_json::<MoveResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "right");
}
