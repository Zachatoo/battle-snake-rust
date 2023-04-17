use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MoveResponse {
    #[serde(rename = "move")]
    pub chosen_move: String,
    pub shout: String,
}
