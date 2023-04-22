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

pub fn get_adjacent_nodes(coord: &Coord) -> Vec<Node> {
    vec![
        Node {
            coord: Coord {
                x: coord.x,
                y: coord.y + 1,
            },
            movement: Movement::Up,
        },
        Node {
            coord: Coord {
                x: coord.x,
                y: coord.y - 1,
            },
            movement: Movement::Down,
        },
        Node {
            coord: Coord {
                x: coord.x - 1,
                y: coord.y,
            },
            movement: Movement::Left,
        },
        Node {
            coord: Coord {
                x: coord.x + 1,
                y: coord.y,
            },
            movement: Movement::Right,
        },
    ]
}
