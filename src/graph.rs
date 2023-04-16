use crate::{movement_set::Movement, request::Coord};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Node {
    pub coord: Coord,
    pub movement: Movement,
}

pub struct LeafNode {
    pub node: Node,
    pub parent: Node,
}
