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
                  "food": [
                    {"x": 5, "y": 5}, 
                    {"x": 9, "y": 0}, 
                    {"x": 2, "y": 6}
                  ],
                  "hazards": [
                    {"x": 3, "y": 2}
                  ],
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
}
