use crate::response::MoveResponse;

use super::rocket;
use rocket::{
    http::{ContentType, Status},
    local::blocking::Client,
};

#[test]
fn movement_avoid_moving_out_of_bounds() {
    let client = Client::untracked(rocket()).expect("Failed to create client instance");
    let response = client
        .post("/move?x-api-key=valid_api_key")
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
        .into_json::<MoveResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "up");
    assert_eq!(parsed_body.shout, "up");
}

#[test]
fn movement_tail_is_safe() {
    let client = Client::untracked(rocket()).expect("Failed to create client instance");
    let response = client
        .post("/move?x-api-key=valid_api_key")
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
        .into_json::<MoveResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "up");
    assert_eq!(parsed_body.shout, "up");
}

#[test]
fn movement_avoid_snake_bodies() {
    let client = Client::untracked(rocket()).expect("Failed to create client instance");
    let response = client
        .post("/move?x-api-key=valid_api_key")
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
        .into_json::<MoveResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "left");
    assert_eq!(parsed_body.shout, "left");
}

#[test]
fn movement_prefer_safe_move_to_semisafe_move() {
    let client = Client::untracked(rocket()).expect("Failed to create client instance");
    let response = client
        .post("/move?x-api-key=valid_api_key")
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
        .into_json::<MoveResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "up");
    assert!(parsed_body.shout.contains("up"));
    assert!(parsed_body.shout.contains("left"));
}

#[test]
fn movement_prefer_food() {
    let client = Client::untracked(rocket()).expect("Failed to create client instance");
    let response = client
        .post("/move?x-api-key=valid_api_key")
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
        .into_json::<MoveResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "up");
    assert!(parsed_body.shout.contains("up"));
    assert!(parsed_body.shout.contains("down"));
    assert!(parsed_body.shout.contains("left"));
}

#[test]
fn movement_prefer_food_avoid_snake() {
    let client = Client::untracked(rocket()).expect("Failed to create client instance");
    let response = client
        .post("/move?x-api-key=valid_api_key")
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
        .into_json::<MoveResponse>()
        .expect("failed to parse response");
    assert_eq!(parsed_body.chosen_move, "down");
    assert!(parsed_body.shout.contains("down"));
    assert!(parsed_body.shout.contains("left"));
}

#[test]
fn movement_prefer_food_avoid_self() {
    let client = Client::untracked(rocket()).expect("Failed to create client instance");
    let response = client
        .post("/move?x-api-key=valid_api_key")
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
        .into_json::<MoveResponse>()
        .expect("failed to parse response");
    assert_ne!(parsed_body.chosen_move, "left");
    assert!(parsed_body.shout.contains("right"));
    assert!(parsed_body.shout.contains("up"));
    assert!(parsed_body.shout.contains("down"));
}
