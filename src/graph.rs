use crate::Coord;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Node {
    pub coord: Coord,
    pub origin_direction: Direction,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::None => "",
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        }
    }
}
