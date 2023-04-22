use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MoveResponse {
    #[serde(rename = "move")]
    pub chosen_move: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MoveShoutResponse {
    #[serde(rename = "move")]
    pub chosen_move: String,
    pub shout: String,
}

#[derive(Deserialize, Serialize)]
pub struct InfoResponse {
    #[serde(rename = "apiversion")]
    pub api_version: String,
    pub author: String,
    pub color: String,
    pub head: String,
    pub tail: String,
}
