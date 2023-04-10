// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::{HashMap, VecDeque};

use crate::{
    graph::{Direction, Node},
    Battlesnake, Board, Coord, Game,
};

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "Zachatoo",
        "color": "#00AA33",
        "head": "gamer",
        "tail": "round-bum",
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &u32, board: &Board, you: &Battlesnake) -> Value {
    let mut is_move_safe: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    // Prevent your Battlesnake from moving backwards
    info!("Prevent your Battlesnake from moving backwards");
    let my_head = &you.body[0]; // Coordinates of your head
    let my_neck = &you.body[1]; // Coordinates of your "neck"

    if my_neck.x < my_head.x {
        // Neck is left of head, don't move left
        is_move_safe.insert("left", false);
    } else if my_neck.x > my_head.x {
        // Neck is right of head, don't move right
        is_move_safe.insert("right", false);
    } else if my_neck.y < my_head.y {
        // Neck is below head, don't move down
        is_move_safe.insert("down", false);
    } else if my_neck.y > my_head.y {
        // Neck is above head, don't move up
        is_move_safe.insert("up", false);
    }

    // Prevent your Battlesnake from moving out of bounds
    info!("Prevent your Battlesnake from moving out of bounds");
    let board_width = &board.width;
    let board_height = &board.height;

    if my_head.x == board_width - 1 {
        // Head is on far right side of board, don't move right
        is_move_safe.insert("right", false);
    }
    if my_head.x == 0 {
        // Head is on far left side of board, don't move left
        is_move_safe.insert("left", false);
    }
    if my_head.y == board_height - 1 {
        // Head is on top of board, don't move up
        is_move_safe.insert("up", false);
    }
    if my_head.y == 0 {
        // Head is on bottom of board, don't move down
        is_move_safe.insert("down", false);
    }

    // Prevent your Battlesnake from colliding with itself
    info!("Prevent your Battlesnake fron colliding with itself");
    set_unsafe_moves_given_head_and_unsafe_coords(&mut is_move_safe, my_head, &you.body);

    // Prevent your Battlesnake from colliding with other Battlesnakes
    let opponents = board
        .snakes
        .clone()
        .into_iter()
        .filter(|snake| snake.id != you.id)
        .collect::<Vec<_>>();

    for opponent in opponents {
        let coords_near_opponent_head =
            get_adjacent_coords(&opponent.head, board.width, board.height);
        info!("Prevent your Battlesnake from colliding with other Battlesnakes head on next turn");
        set_unsafe_moves_given_head_and_unsafe_coords(
            &mut is_move_safe,
            my_head,
            &coords_near_opponent_head,
        );

        info!("Prevent your Battlesnake from colliding with other Battlesnakes body");
        set_unsafe_moves_given_head_and_unsafe_coords(&mut is_move_safe, my_head, &opponent.body);
    }

    // Move towards food instead of random, to regain health and survive longer
    let my_head_node = Node {
        coord: my_head.to_owned(),
        origin_direction: Direction::None,
        path: Vec::new(),
    };
    let mut closest_food_node: Option<Node> = None;

    let mut frontier = VecDeque::<Node>::new();
    frontier.push_back(my_head_node.to_owned());

    let mut visited_nodes: HashMap<_, _> =
        vec![(my_head_node.to_owned(), false)].into_iter().collect();

    while !frontier.is_empty() {
        let current = &frontier.pop_front().unwrap();
        let coord = &current.coord;

        if board.food.contains(coord) {
            info!("Found food at {} {}", coord.x, coord.y);
            closest_food_node = Some(current.to_owned());
            break;
        }

        for adjacent_node in get_adjacent_nodes(current, board.width, board.height) {
            if !visited_nodes.contains_key(&adjacent_node) {
                frontier.push_back(adjacent_node.to_owned());
                visited_nodes.insert(adjacent_node.to_owned(), true);
            }
        }
    }

    let first_food_direction = match closest_food_node {
        Some(node) => node.path[0].origin_direction,
        _ => Direction::None,
    };
    info!("First direction {}", first_food_direction.as_str());

    // Are there any safe moves left?
    let safe_moves = is_move_safe
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    if safe_moves
        .iter()
        .any(|v| v == &first_food_direction.as_str())
    {
        info!("Moving towards food");
        info!("MOVE {}: {}", turn, first_food_direction.as_str());
        return json!({ "move": first_food_direction.as_str() });
    }

    // Choose a random move from the safe ones
    info!("Choosing a random move");
    let chosen = match safe_moves.choose(&mut rand::thread_rng()) {
        Some(res) => res,
        None => {
            info!("No safe moves found");
            "up"
        }
    };

    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}

pub fn set_unsafe_moves_given_head_and_unsafe_coords(
    is_move_safe: &mut HashMap<&str, bool>,
    my_head: &Coord,
    unsafe_coords: &Vec<Coord>,
) {
    for coord in unsafe_coords {
        if coord_is_right_of_head(my_head, coord) {
            info!(
                "Right is unsafe. my_head x {} y {}, coord x {} y {}",
                my_head.x, my_head.y, coord.x, coord.y
            );
            is_move_safe.insert("right", false);
        } else if coord_is_left_of_head(my_head, coord) {
            info!(
                "Left is unsafe. my_head x {} y {}, coord x {} y {}",
                my_head.x, my_head.y, coord.x, coord.y
            );
            is_move_safe.insert("left", false);
        } else if coord_is_above_head(my_head, coord) {
            info!(
                "Up is unsafe. my_head x {} y {}, coord x {} y {}",
                my_head.x, my_head.y, coord.x, coord.y
            );
            is_move_safe.insert("up", false);
        } else if coord_is_below_head(my_head, coord) {
            info!(
                "Down is unsafe. my_head x {} y {}, coord x {} y {}",
                my_head.x, my_head.y, coord.x, coord.y
            );
            is_move_safe.insert("down", false);
        }
    }
}

pub fn coord_is_right_of_head(my_head: &Coord, coord: &Coord) -> bool {
    return my_head.x + 1 == coord.x && my_head.y == coord.y;
}

pub fn coord_is_left_of_head(my_head: &Coord, coord: &Coord) -> bool {
    let safe_head_left_x = match my_head.x {
        0 => my_head.x,
        _ => my_head.x - 1,
    };
    return safe_head_left_x == coord.x && my_head.y == coord.y;
}

pub fn coord_is_above_head(my_head: &Coord, coord: &Coord) -> bool {
    return my_head.y + 1 == coord.y && my_head.x == coord.x;
}

pub fn coord_is_below_head(my_head: &Coord, coord: &Coord) -> bool {
    let safe_head_down_y = match my_head.y {
        0 => my_head.y,
        _ => my_head.y - 1,
    };
    return safe_head_down_y == coord.y && my_head.x == coord.x;
}

pub fn get_adjacent_coords(coord: &Coord, width: u32, height: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = vec![];
    if coord.x > 0 {
        coords.push(Coord {
            x: coord.x - 1,
            y: coord.y,
        });
    }
    if coord.y > 0 {
        coords.push(Coord {
            x: coord.x,
            y: coord.y - 1,
        });
    }
    if coord.x < width - 1 {
        coords.push(Coord {
            x: coord.x + 1,
            y: coord.y,
        });
    }
    if coord.y < height - 1 {
        coords.push(Coord {
            x: coord.x,
            y: coord.y + 1,
        });
    }
    return coords;
}

pub fn get_adjacent_nodes(node: &Node, width: u32, height: u32) -> Vec<Node> {
    let mut nodes: Vec<Node> = vec![];
    let coord = &node.coord;
    if coord.x > 0 {
        let coord = Coord {
            x: coord.x - 1,
            y: coord.y,
        };
        let mut right_node = Node {
            coord,
            origin_direction: Direction::Right,
            path: node.path.to_owned(),
        };
        right_node.path.push(right_node.to_owned());
        nodes.push(right_node);
    }
    if coord.y > 0 {
        let coord = Coord {
            x: coord.x,
            y: coord.y - 1,
        };
        let mut up_node = Node {
            coord,
            origin_direction: Direction::Up,
            path: node.path.to_owned(),
        };
        up_node.path.push(up_node.to_owned());
        nodes.push(up_node);
    }
    if coord.x < width - 1 {
        let coord = Coord {
            x: coord.x + 1,
            y: coord.y,
        };
        let mut left_node = Node {
            coord,
            origin_direction: Direction::Left,
            path: node.path.to_owned(),
        };
        left_node.path.push(left_node.to_owned());
        nodes.push(left_node);
    }
    if coord.y < height - 1 {
        let coord = Coord {
            x: coord.x,
            y: coord.y + 1,
        };
        let mut down_node = Node {
            coord,
            origin_direction: Direction::Down,
            path: node.path.to_owned(),
        };
        down_node.path.push(down_node.to_owned());
        nodes.push(down_node);
    }
    return nodes;
}
