use crate::{movement_set::Movement, request::Coord};

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Node {
    pub coord: Coord,
    pub movement: Movement,
}
