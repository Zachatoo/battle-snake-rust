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
use std::collections::HashMap;

use crate::{Battlesnake, Board, Coord, Game};

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
        let coords_near_opponent_head = get_adjacent_coords(&opponent.head);
        info!("Prevent your Battlesnake from colliding with other Battlesnakes head on next turn");
        set_unsafe_moves_given_head_and_unsafe_coords(
            &mut is_move_safe,
            my_head,
            &coords_near_opponent_head,
        );

        info!("Prevent your Battlesnake from colliding with other Battlesnakes body");
        set_unsafe_moves_given_head_and_unsafe_coords(&mut is_move_safe, my_head, &opponent.body);
    }

    // Are there any safe moves left?
    let safe_moves = is_move_safe
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    // Choose a random move from the safe ones
    let chosen = match safe_moves.choose(&mut rand::thread_rng()) {
        Some(res) => res,
        None => {
            info!("No safe moves found");
            "up"
        }
    };

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    // let food = &board.food;

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
                "Right is usafe. my_head x {} y {}, coord x {} y {}",
                my_head.x, my_head.y, coord.x, coord.y
            );
            is_move_safe.insert("right", false);
        } else if coord_is_left_of_head(my_head, coord) {
            info!(
                "Left is usafe. my_head x {} y {}, coord x {} y {}",
                my_head.x, my_head.y, coord.x, coord.y
            );
            is_move_safe.insert("left", false);
        } else if coord_is_above_head(my_head, coord) {
            info!(
                "Up is usafe. my_head x {} y {}, coord x {} y {}",
                my_head.x, my_head.y, coord.x, coord.y
            );
            is_move_safe.insert("up", false);
        } else if coord_is_below_head(my_head, coord) {
            info!(
                "Down is usafe. my_head x {} y {}, coord x {} y {}",
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

pub fn get_adjacent_coords(coord: &Coord) -> Vec<Coord> {
    let safe_left_x = match coord.x {
        0 => coord.x,
        _ => coord.x - 1,
    };
    let safe_down_y = match coord.y {
        0 => coord.y,
        _ => coord.y - 1,
    };
    return vec![
        Coord {
            x: safe_left_x,
            y: coord.y,
        },
        Coord {
            x: coord.x + 1,
            y: coord.y,
        },
        Coord {
            x: coord.x,
            y: safe_down_y,
        },
        Coord {
            x: coord.x,
            y: coord.y + 1,
        },
    ];
}
