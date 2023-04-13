use log::info;
use serde_json::{json, Value};

use crate::{
    graph::Node,
    movement_set::{Movement, WeightedMovementSet},
    request::{Battlesnake, Board, Coord, Game},
};

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

pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

pub fn get_move(_game: &Game, turn: &u32, board: &Board, you: &Battlesnake) -> Value {
    let mut movement_set = WeightedMovementSet::new();

    avoid_bounds(board.width, board.height, you, &mut movement_set);
    avoid_snake_bodies(&board.snakes, you, &mut movement_set);
    handle_opponent_heads(&board.snakes, you, &mut movement_set);

    info!("Safe moves: {:?}", movement_set.moves);
    let chosen_move = movement_set.pick_movement().as_str();
    info!("MOVE {}: {}", turn, chosen_move);
    return json!({ "move": chosen_move });
}

fn avoid_bounds(width: u32, height: u32, you: &Battlesnake, set: &mut WeightedMovementSet) {
    info!("Avoiding going out of bounds");
    let my_head = &you.head;
    let adjacent_nodes = get_adjacent_nodes(my_head);
    for node in &adjacent_nodes {
        if node.coord.x < 0
            || node.coord.x >= (width as i32)
            || node.coord.y < 0
            || node.coord.y >= (height as i32)
        {
            set.remove(&node.movement);
        }
    }
}

fn avoid_snake_bodies(snakes: &Vec<Battlesnake>, you: &Battlesnake, set: &mut WeightedMovementSet) {
    info!("Avoiding snake bodies");
    let my_head = &you.head;
    let adjacent_nodes = get_adjacent_nodes(my_head);
    for adjacent_node in &adjacent_nodes {
        for snake in snakes {
            for snake_coord in &snake.body[0..snake.body.len() - 1] {
                if adjacent_node.coord.x == snake_coord.x && adjacent_node.coord.y == snake_coord.y
                {
                    set.remove(&adjacent_node.movement);
                }
            }
            // Movement into tail space is not safe if snake has just eaten
            if snake_is_stacked(snake)
                && adjacent_node.coord.x == snake.body.last().unwrap().x
                && adjacent_node.coord.y == snake.body.last().unwrap().y
            {
                set.remove(&adjacent_node.movement);
            }
        }
    }
}

fn handle_opponent_heads(
    snakes: &Vec<Battlesnake>,
    you: &Battlesnake,
    set: &mut WeightedMovementSet,
) {
    info!("Avoiding opponent snake heads if short");
    let my_head = &you.head;
    let opponents: Vec<_> = snakes.iter().filter(|x| x.id != you.id).collect();
    let adjacent_nodes = get_adjacent_nodes(my_head);
    for adjacent_node in &adjacent_nodes {
        for opponent in &opponents {
            let adjacent_opponent_nodes = get_adjacent_nodes(&opponent.head);
            for adjacent_opponent_node in &adjacent_opponent_nodes {
                if adjacent_node.coord.x == adjacent_opponent_node.coord.x
                    && adjacent_node.coord.y == adjacent_opponent_node.coord.y
                    && you.length <= opponent.length
                {
                    set.change_probability(&adjacent_node.movement, 50);
                }
            }
        }
    }
}

fn snake_is_stacked(snake: &Battlesnake) -> bool {
    for i in 0..snake.body.len() - 1 {
        for j in i + 1..snake.body.len() {
            if snake.body[i] == snake.body[j] {
                return true;
            }
        }
    }
    false
}

fn get_adjacent_nodes(coord: &Coord) -> Vec<Node> {
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
