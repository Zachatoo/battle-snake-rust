use std::collections::HashSet;

use serde_json::{json, Value};

use crate::{
    fifo_queue::FifoQueue,
    graph::{LeafNode, Node},
    movement_set::{Movement, WeightedMovementSet},
    request::{Battlesnake, Board, Coord, Game},
    response::MoveResponse,
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

pub fn start(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START {}", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER {}", game.id);
}

pub fn get_move(_game: &Game, turn: &u32, board: &Board, you: &Battlesnake) -> Value {
    let mut movement_set = WeightedMovementSet::new();

    avoid_bounds(board.width, board.height, you, &mut movement_set);
    avoid_snake_bodies(&board.snakes, you, &mut movement_set);
    scan_food(&board, you, &mut movement_set);
    avoid_small_spaces(&board, you, &mut movement_set);
    handle_opponent_heads(&board.snakes, you, &mut movement_set);
    avoid_hazards(&board.hazards, you, &mut movement_set);

    info!("Safe moves: {:?}", movement_set.moves);
    let chosen_move = movement_set.pick_movement().as_str().to_string();
    info!("MOVE {}: {}", turn, chosen_move);
    return json!(MoveResponse {
        chosen_move,
        shout: movement_set
            .moves
            .into_iter()
            .map(|x| x.movement.as_str().to_owned())
            .collect::<Vec<String>>()
            .join(","),
    });
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

fn avoid_hazards(hazards: &Vec<Coord>, you: &Battlesnake, set: &mut WeightedMovementSet) {
    if hazards.len() == 0 {
        return;
    }

    info!("Avoiding hazards");
    let my_head = &you.head;
    let adjacent_nodes = get_adjacent_nodes(my_head);
    for adjacent_node in adjacent_nodes {
        for hazard in hazards {
            if hazard.x == adjacent_node.coord.x && hazard.y == adjacent_node.coord.y {
                set.update_score(&adjacent_node.movement, -70);
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
                {
                    if you.length < opponent.length {
                        set.update_score(&adjacent_node.movement, -60);
                    } else if you.length == opponent.length {
                        set.update_score(&adjacent_node.movement, -50);
                    } else {
                        set.update_score(&adjacent_node.movement, 30);
                    }
                }
            }
        }
    }
}

fn scan_food(board: &Board, you: &Battlesnake, set: &mut WeightedMovementSet) {
    if board.food.len() == 0 {
        return;
    }
    info!("Searching for food");

    let my_head = you.head.to_owned();
    let snake_coords = get_all_snake_coords(&board.snakes);

    let mut food_movements = FifoQueue::<Movement>::new();
    let mut frontier = FifoQueue::<LeafNode>::new();
    let mut visited_coords: HashSet<_> = vec![my_head].into_iter().collect();

    let adjacent_nodes = get_adjacent_nodes(&my_head);
    for adjacent_node in adjacent_nodes {
        if set.moves.contains(&adjacent_node.movement) {
            frontier.enqueue(LeafNode {
                node: adjacent_node,
                parent: adjacent_node,
            });
            visited_coords.insert(adjacent_node.coord);
        }
    }

    loop {
        let current = match frontier.dequeue() {
            Some(x) => x,
            None => break,
        };
        let coord = &current.node.coord;

        if board.food.contains(coord) {
            info!("Found food at {} {}", coord.x, coord.y);
            food_movements.enqueue(current.parent.movement);
        }

        let adjacent_nodes = get_adjacent_nodes(coord);
        for adjacent_node in adjacent_nodes {
            if adjacent_node.coord.x >= 0
                && adjacent_node.coord.x < (board.width as i32)
                && adjacent_node.coord.y >= 0
                && adjacent_node.coord.y < (board.height as i32)
                && !snake_coords.contains(&adjacent_node.coord)
                && !visited_coords.contains(&adjacent_node.coord)
            {
                frontier.enqueue(LeafNode {
                    node: adjacent_node,
                    parent: current.parent,
                });
                visited_coords.insert(adjacent_node.coord);
            }
        }
    }

    let mut probability = 20;
    loop {
        let movement = match food_movements.dequeue() {
            Some(x) => x,
            None => break,
        };
        set.update_score(&movement, probability);
        probability -= 5;
    }
}

fn avoid_small_spaces(board: &Board, you: &Battlesnake, set: &mut WeightedMovementSet) {
    info!("Check if snake can fit in space");

    let my_head = you.head.to_owned();
    let my_length = you.length as usize;
    let snake_coords = get_all_snake_coords(&board.snakes);

    let mut frontier = FifoQueue::<LeafNode>::new();

    let adjacent_nodes = get_adjacent_nodes(&my_head);
    for adjacent_node in adjacent_nodes {
        let movement = &adjacent_node.movement;
        if !set.moves.contains(movement) {
            continue;
        }

        frontier.enqueue(LeafNode {
            node: adjacent_node,
            parent: adjacent_node,
        });
        let mut visited_coords: HashSet<_> =
            vec![my_head, adjacent_node.coord].into_iter().collect();

        loop {
            if visited_coords.len() >= my_length {
                frontier.clear();
                break;
            }
            let current = match frontier.dequeue() {
                Some(x) => x,
                None => {
                    info!(
                        "movement: {:?}, len: {}, space: {}",
                        movement,
                        my_length,
                        visited_coords.len()
                    );
                    if my_length > visited_coords.len() {
                        set.update_score(movement, -70);
                    }
                    break;
                }
            };
            let coord = &current.node.coord;

            let adjacent_nodes = get_adjacent_nodes(coord);
            for adjacent_node in adjacent_nodes {
                if adjacent_node.coord.x >= 0
                    && adjacent_node.coord.x < (board.width as i32)
                    && adjacent_node.coord.y >= 0
                    && adjacent_node.coord.y < (board.height as i32)
                    && !snake_coords.contains(&adjacent_node.coord)
                    && !visited_coords.contains(&adjacent_node.coord)
                {
                    frontier.enqueue(LeafNode {
                        node: adjacent_node,
                        parent: current.parent,
                    });
                    visited_coords.insert(adjacent_node.coord);
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

fn get_all_snake_coords(snakes: &Vec<Battlesnake>) -> HashSet<Coord> {
    let mut coords: HashSet<Coord> = HashSet::new();
    for snake in snakes {
        for coord in &snake.body {
            coords.insert(*coord);
        }
    }
    coords
}
