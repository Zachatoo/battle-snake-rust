use crate::{
    logic::{
        avoid_bounds, avoid_hazards, avoid_small_spaces, avoid_snake_bodies, handle_opponent_heads,
        scan_food,
    },
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
        color: "#00AA33".to_string(),
        head: "gamer".to_string(),
        tail: "round-bum".to_string(),
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
    scan_food(&board, you, &mut movement_set);
    avoid_small_spaces(&board, you, &mut movement_set);
    handle_opponent_heads(&board.snakes, you, &mut movement_set);
    avoid_hazards(&board.hazards, you, &mut movement_set);

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
static MOVE_URI: &str = "/rusty/move?x-api-key=valid_api_key";

#[test]
fn movement_avoid_moving_out_of_bounds() {
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
                        {"x": 0, "y": 0},
                        {"x": 1, "y": 0},
                        {"x": 2, "y": 0}
                      ],
                      "latency": "111",
                      "head": {"x": 0, "y": 0},
                      "length": 3
                    }
                  ]
                },
                "you": {
                  "id": "my-snake",
                  "name": "My Snake",
                  "health": 54,
                  "body": [
                    {"x": 0, "y": 0},
                    {"x": 1, "y": 0},
                    {"x": 2, "y": 0}
                  ],
                  "latency": "111",
                  "head": {"x": 0, "y": 0},
                  "length": 3
                }
              }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let parsed_body = response
        .into_json::<MoveShoutResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "up");
    assert_eq!(parsed_body.shout, "up");
}

#[test]
fn movement_tail_is_safe() {
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
                        {"x": 0, "y": 0},
                        {"x": 1, "y": 0},
                        {"x": 1, "y": 1},
                        {"x": 0, "y": 1}
                      ],
                      "latency": "111",
                      "head": {"x": 0, "y": 0},
                      "length": 4
                    }
                  ]
                },
                "you": {
                  "id": "my-snake",
                  "name": "My Snake",
                  "health": 54,
                  "body": [
                    {"x": 0, "y": 0},
                    {"x": 1, "y": 0},
                    {"x": 1, "y": 1},
                    {"x": 0, "y": 1}
                  ],
                  "latency": "111",
                  "head": {"x": 0, "y": 0},
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
    assert_eq!(parsed_body.shout, "up");
}

#[test]
fn movement_avoid_snake_bodies() {
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
                        {"x": 5, "y": 5},
                        {"x": 6, "y": 5},
                        {"x": 7, "y": 5}
                      ],
                      "latency": "111",
                      "head": {"x": 5, "y": 5},
                      "length": 3
                    },
                    {
                      "id": "snake-1",
                      "name": "Snake 1",
                      "health": 54,
                      "body": [
                        {"x": 5, "y": 6},
                        {"x": 6, "y": 6},
                        {"x": 7, "y": 6}
                      ],
                      "latency": "111",
                      "head": {"x": 5, "y": 6},
                      "length": 3
                    },
                    {
                      "id": "snake-2",
                      "name": "Snake 2",
                      "health": 54,
                      "body": [
                        {"x": 5, "y": 4},
                        {"x": 6, "y": 4},
                        {"x": 7, "y": 4}
                      ],
                      "latency": "111",
                      "head": {"x": 5, "y": 4},
                      "length": 3
                    }
                  ]
                },
                "you": {
                  "id": "my-snake",
                  "name": "My Snake",
                  "health": 54,
                  "body": [
                    {"x": 5, "y": 5},
                    {"x": 6, "y": 5},
                    {"x": 7, "y": 5}
                  ],
                  "latency": "111",
                  "head": {"x": 5, "y": 5},
                  "length": 3
                }
              }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let parsed_body = response
        .into_json::<MoveShoutResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "left");
    assert_eq!(parsed_body.shout, "left");
}

#[test]
fn movement_prefer_safe_move_to_semisafe_move() {
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
                        {"x": 6, "y": 5},
                        {"x": 7, "y": 5},
                        {"x": 8, "y": 5}
                      ],
                      "latency": "111",
                      "head": {"x": 6, "y": 5},
                      "length": 3
                    },
                    {
                      "id": "snake-1",
                      "name": "Snake 1",
                      "health": 54,
                      "body": [
                        {"x": 4, "y": 5},
                        {"x": 3, "y": 5},
                        {"x": 2, "y": 5},
                        {"x": 1, "y": 5}
                      ],
                      "latency": "111",
                      "head": {"x": 4, "y": 5},
                      "length": 4
                    },
                    {
                      "id": "snake-2",
                      "name": "Snake 2",
                      "health": 54,
                      "body": [
                        {"x": 6, "y": 4},
                        {"x": 7, "y": 4},
                        {"x": 8, "y": 4}
                      ],
                      "latency": "111",
                      "head": {"x": 6, "y": 4},
                      "length": 3
                    }
                  ]
                },
                "you": {
                  "id": "my-snake",
                  "name": "My Snake",
                  "health": 54,
                  "body": [
                    {"x": 6, "y": 5},
                    {"x": 7, "y": 5},
                    {"x": 8, "y": 5}
                  ],
                  "latency": "111",
                  "head": {"x": 6, "y": 5},
                  "length": 3
                }
              }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let parsed_body = response
        .into_json::<MoveShoutResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "up");
    assert!(parsed_body.shout.contains("up"));
    assert!(parsed_body.shout.contains("left"));
    assert!(!parsed_body.shout.contains("right"));
    assert!(!parsed_body.shout.contains("down"));
}

#[test]
fn movement_prefer_food() {
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
                  "food": [
                    { "x": 5, "y": 9 }
                  ],
                  "hazards": [],
                  "snakes": [
                    {
                      "id": "my-snake",
                      "name": "My Snake",
                      "health": 54,
                      "body": [
                        {"x": 5, "y": 5},
                        {"x": 6, "y": 5},
                        {"x": 7, "y": 5}
                      ],
                      "latency": "111",
                      "head": {"x": 5, "y": 5},
                      "length": 3
                    }
                  ]
                },
                "you": {
                  "id": "my-snake",
                  "name": "My Snake",
                  "health": 54,
                  "body": [
                    {"x": 5, "y": 5},
                    {"x": 6, "y": 5},
                    {"x": 7, "y": 5}
                  ],
                  "latency": "111",
                  "head": {"x": 5, "y": 5},
                  "length": 3
                }
              }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let parsed_body = response
        .into_json::<MoveShoutResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "up");
    assert!(parsed_body.shout.contains("up"));
    assert!(parsed_body.shout.contains("down"));
    assert!(parsed_body.shout.contains("left"));
    assert!(!parsed_body.shout.contains("right"));
}

#[test]
fn movement_prefer_food_avoid_snake() {
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
                  "food": [
                    { "x": 5, "y": 9 }
                  ],
                  "hazards": [],
                  "snakes": [
                    {
                      "id": "my-snake",
                      "name": "My Snake",
                      "health": 54,
                      "body": [
                        {"x": 5, "y": 5},
                        {"x": 6, "y": 5},
                        {"x": 7, "y": 5}
                      ],
                      "latency": "111",
                      "head": {"x": 5, "y": 5},
                      "length": 3
                    },
                    {
                      "id": "snake-1",
                      "name": "Snake 1",
                      "health": 54,
                      "body": [
                        {"x": 4, "y": 6},
                        {"x": 5, "y": 6},
                        {"x": 6, "y": 6},
                        {"x": 7, "y": 6}
                      ],
                      "latency": "111",
                      "head": {"x": 4, "y": 6},
                      "length": 4
                    }
                  ]
                },
                "you": {
                  "id": "my-snake",
                  "name": "My Snake",
                  "health": 54,
                  "body": [
                    {"x": 5, "y": 5},
                    {"x": 6, "y": 5},
                    {"x": 7, "y": 5}
                  ],
                  "latency": "111",
                  "head": {"x": 5, "y": 5},
                  "length": 3
                }
              }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let parsed_body = response
        .into_json::<MoveShoutResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "down");
    assert!(parsed_body.shout.contains("down"));
    assert!(parsed_body.shout.contains("left"));
    assert!(!parsed_body.shout.contains("right"));
    assert!(!parsed_body.shout.contains("up"));
}

#[test]
fn movement_prefer_food_avoid_self() {
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
                  "food": [
                    { "x": 2, "y": 10 }
                  ],
                  "hazards": [],
                  "snakes": [
                    {
                      "id": "my-snake",
                      "name": "My Snake",
                      "health": 54,
                      "body": [
                        {"x": 5, "y": 9},
                        {"x": 4, "y": 9},
                        {"x": 4, "y": 10},
                        {"x": 5, "y": 10}
                      ],
                      "latency": "111",
                      "head": {"x": 5, "y": 9},
                      "length": 4
                    }
                  ]
                },
                "you": {
                  "id": "my-snake",
                  "name": "My Snake",
                  "health": 54,
                  "body": [
                    {"x": 5, "y": 9},
                    {"x": 4, "y": 9},
                    {"x": 4, "y": 10},
                    {"x": 5, "y": 10}
                  ],
                  "latency": "111",
                  "head": {"x": 5, "y": 9},
                  "length": 4
                }
              }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let parsed_body = response
        .into_json::<MoveShoutResponse>()
        .expect("failed to parse response");
    assert_ne!(parsed_body.chosen_move, "left");
    assert!(!parsed_body.shout.contains("left"));
    assert!(parsed_body.shout.contains("right"));
    assert!(parsed_body.shout.contains("up"));
    assert!(parsed_body.shout.contains("down"));
}

#[test]
fn movement_prefer_food_long_scan() {
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
                "food": [
                  { "x": 2, "y": 10 }
                ],
                "hazards": [],
                "snakes": [
                  {
                    "id": "my-snake",
                    "name": "My Snake",
                    "health": 54,
                    "body": [
                      {"x": 6, "y": 0},
                      {"x": 6, "y": 1},
                      {"x": 7, "y": 1},
                      {"x": 8, "y": 1},
                      {"x": 9, "y": 1},
                      {"x": 9, "y": 2},
                      {"x": 9, "y": 3}
                    ],
                    "latency": "111",
                    "head": {"x": 6, "y": 0},
                    "length": 7
                  },
                  {
                    "id": "snake-1",
                    "name": "Snake 1",
                    "health": 54,
                    "body": [
                      {"x": 3, "y": 0},
                      {"x": 4, "y": 0},
                      {"x": 4, "y": 1},
                      {"x": 5, "y": 1},
                      {"x": 5, "y": 2},
                      {"x": 5, "y": 3},
                      {"x": 5, "y": 4},
                      {"x": 5, "y": 5}
                    ],
                    "latency": "111",
                    "head": {"x": 3, "y": 0},
                    "length": 8
                  }
                ]
              },
              "you": {
                "id": "my-snake",
                "name": "My Snake",
                "health": 54,
                "body": [
                  {"x": 6, "y": 0},
                  {"x": 6, "y": 1},
                  {"x": 7, "y": 1},
                  {"x": 8, "y": 1},
                  {"x": 9, "y": 1},
                  {"x": 9, "y": 2},
                  {"x": 9, "y": 3}
                ],
                "latency": "111",
                "head": {"x": 6, "y": 0},
                "length": 7
              }
            }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let parsed_body = response
        .into_json::<MoveShoutResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "right");
    assert!(parsed_body.shout.contains("right"));
    assert!(parsed_body.shout.contains("left"));
    assert!(!parsed_body.shout.contains("up"));
    assert!(!parsed_body.shout.contains("down"));
}

#[test]
fn movement_prefer_food_prefer_closer_food() {
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
                "food": [
                  { "x": 4, "y": 10 },
                  { "x": 7, "y": 10 }
                ],
                "hazards": [],
                "snakes": [
                  {
                    "id": "my-snake",
                    "name": "My Snake",
                    "health": 54,
                    "body": [
                      {"x": 5, "y": 10},
                      {"x": 5, "y": 9},
                      {"x": 5, "y": 8}
                    ],
                    "latency": "111",
                    "head": {"x": 5, "y": 10},
                    "length": 3
                  }
                ]
              },
              "you": {
                "id": "my-snake",
                "name": "My Snake",
                "health": 54,
                "body": [
                  {"x": 5, "y": 10},
                  {"x": 5, "y": 9},
                  {"x": 5, "y": 8}
                ],
                "latency": "111",
                "head": {"x": 5, "y": 10},
                "length": 3
              }
            }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let parsed_body = response
        .into_json::<MoveShoutResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "left");
    assert!(parsed_body.shout.contains("right"));
    assert!(parsed_body.shout.contains("left"));
    assert!(!parsed_body.shout.contains("up"));
    assert!(!parsed_body.shout.contains("down"));
}

#[test]
fn movement_avoid_small_spaces() {
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
                      {"x": 9, "y": 0},
                      {"x": 9, "y": 1},
                      {"x": 9, "y": 2},
                      {"x": 10, "y": 2}
                    ],
                    "latency": "111",
                    "head": {"x": 9, "y": 0},
                    "length": 4
                  }
                ]
              },
              "you": {
                "id": "my-snake",
                "name": "My Snake",
                "health": 54,
                "body": [
                  {"x": 9, "y": 0},
                  {"x": 9, "y": 1},
                  {"x": 9, "y": 2},
                  {"x": 10, "y": 2}
                ],
                "latency": "111",
                "head": {"x": 9, "y": 0},
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
    assert!(parsed_body.shout.contains("right"));
    assert!(parsed_body.shout.contains("left"));
    assert!(!parsed_body.shout.contains("up"));
    assert!(!parsed_body.shout.contains("down"));
}

#[test]
fn movement_prefer_food_prefer_single_closer_food_to_two_further_food() {
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
              "turn": 47,
              "board": {
                "height": 11,
                "width": 11,
                "food": [
                  { "x": 10, "y": 10 },
                  { "x": 0, "y": 0 },
                  { "x": 3, "y": 1 }
                ],
                "hazards": [],
                "snakes": [
                  {
                    "id": "my-snake",
                    "name": "My Snake",
                    "health": 89,
                    "body": [
                      {"x": 9, "y": 10},
                      {"x": 9, "y": 9},
                      {"x": 9, "y": 8},
                      {"x": 9, "y": 7},
                      {"x": 9, "y": 6},
                      {"x": 9, "y": 5},
                      {"x": 9, "y": 4}
                    ],
                    "latency": "111",
                    "head": {"x": 9, "y": 10},
                    "length": 7
                  },
                  {
                    "id": "snake-2",
                    "name": "Snake 2",
                    "health": 93,
                    "body": [
                      {"x": 5, "y": 10},
                      {"x": 5, "y": 9},
                      {"x": 5, "y": 8},
                      {"x": 5, "y": 7},
                      {"x": 5, "y": 6},
                      {"x": 5, "y": 5},
                      {"x": 5, "y": 4},
                      {"x": 5, "y": 3},
                      {"x": 5, "y": 2},
                      {"x": 5, "y": 1}
                    ],
                    "latency": "111",
                    "head": {"x": 5, "y": 10},
                    "length": 10
                  }
                ]
              },
              "you": {
                "id": "my-snake",
                "name": "My Snake",
                "health": 89,
                "body": [
                  {"x": 9, "y": 10},
                  {"x": 9, "y": 9},
                  {"x": 9, "y": 8},
                  {"x": 9, "y": 7},
                  {"x": 9, "y": 6},
                  {"x": 9, "y": 5},
                  {"x": 9, "y": 4}
                ],
                "latency": "111",
                "head": {"x": 9, "y": 10},
                "length": 7
              }
            }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let parsed_body = response
        .into_json::<MoveShoutResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "right");
    assert!(parsed_body.shout.contains("right"));
    assert!(parsed_body.shout.contains("left"));
    assert!(!parsed_body.shout.contains("up"));
    assert!(!parsed_body.shout.contains("down"));
}

#[ignore = "Not implemented yet"]
#[test]
fn movement_trap_enemy_snake_against_wall() {
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
              "turn": 47,
              "board": {
                "height": 11,
                "width": 11,
                "food": [
                  { "x": 9, "y": 9 }
                ],
                "hazards": [],
                "snakes": [
                  {
                    "id": "my-snake",
                    "name": "My Snake",
                    "health": 89,
                    "body": [
                      { "x": 1, "y": 9 },
                      { "x": 1, "y": 8 },
                      { "x": 1, "y": 7 },
                      { "x": 1, "y": 6 },
                      { "x": 1, "y": 5 },
                      { "x": 1, "y": 4 },
                      { "x": 2, "y": 4 },
                      { "x": 3, "y": 4 },
                      { "x": 4, "y": 4 },
                      { "x": 4, "y": 5 },
                      { "x": 4, "y": 6 }
                    ],
                    "latency": "111",
                    "head": {"x": 1, "y": 9},
                    "length": 11
                  },
                  {
                    "id": "snake-2",
                    "name": "Snake 2",
                    "health": 93,
                    "body": [
                      {"x": 0, "y": 4},
                      {"x": 0, "y": 3},
                      {"x": 1, "y": 3},
                      {"x": 2, "y": 3},
                      {"x": 3, "y": 3},
                      {"x": 4, "y": 3},
                      {"x": 4, "y": 2},
                      {"x": 4, "y": 1},
                      {"x": 4, "y": 0},
                      {"x": 5, "y": 0},
                      {"x": 5, "y": 1},
                      {"x": 5, "y": 2},
                      {"x": 5, "y": 3},
                      {"x": 5, "y": 4}
                    ],
                    "latency": "111",
                    "head": {"x": 0, "y": 4},
                    "length": 14
                  }
                ]
              },
              "you": {
                "id": "my-snake",
                "name": "My Snake",
                "health": 89,
                "body": [
                  { "x": 1, "y": 9 },
                  { "x": 1, "y": 8 },
                  { "x": 1, "y": 7 },
                  { "x": 1, "y": 6 },
                  { "x": 1, "y": 5 },
                  { "x": 1, "y": 4 },
                  { "x": 2, "y": 4 },
                  { "x": 3, "y": 4 },
                  { "x": 4, "y": 4 },
                  { "x": 4, "y": 5 },
                  { "x": 4, "y": 6 }
                ],
                "latency": "111",
                "head": {"x": 1, "y": 9},
                "length": 11
              }
            }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let parsed_body = response
        .into_json::<MoveShoutResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "left");
    assert!(parsed_body.shout.contains("right"));
    assert!(parsed_body.shout.contains("left"));
    assert!(parsed_body.shout.contains("up"));
    assert!(!parsed_body.shout.contains("down"));
}
