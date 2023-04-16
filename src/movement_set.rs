use std::borrow::Borrow;
use std::cmp::max;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use log::info;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Movement {
    Right,
    Left,
    Up,
    Down,
}

impl Movement {
    pub fn as_str(&self) -> &'static str {
        match self {
            Movement::Up => "up",
            Movement::Down => "down",
            Movement::Left => "left",
            Movement::Right => "right",
        }
    }
}

#[derive(Debug, Eq)]
pub struct WeightedMovement {
    pub movement: Movement,
    pub probability_of_success: usize,
}

impl PartialEq for WeightedMovement {
    fn eq(&self, other: &WeightedMovement) -> bool {
        self.movement == other.movement
    }
}

impl Hash for WeightedMovement {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.movement.hash(state);
    }
}

impl Borrow<Movement> for WeightedMovement {
    fn borrow(&self) -> &Movement {
        &self.movement
    }
}

pub struct WeightedMovementSet {
    pub moves: HashSet<WeightedMovement>,
}

impl WeightedMovementSet {
    pub fn new() -> WeightedMovementSet {
        WeightedMovementSet {
            moves: vec![
                WeightedMovement {
                    movement: Movement::Up,
                    probability_of_success: 100,
                },
                WeightedMovement {
                    movement: Movement::Down,
                    probability_of_success: 100,
                },
                WeightedMovement {
                    movement: Movement::Left,
                    probability_of_success: 100,
                },
                WeightedMovement {
                    movement: Movement::Right,
                    probability_of_success: 100,
                },
            ]
            .into_iter()
            .collect(),
        }
    }

    pub fn remove(&mut self, movement: &Movement) {
        self.moves.remove(movement);
        info!("Set {} as unsafe", movement.as_str());
    }

    pub fn set_probability(&mut self, movement: &Movement, new_probability: usize) {
        match self.moves.get(movement) {
            Some(_) => {
                self.moves.replace(WeightedMovement {
                    movement: movement.to_owned(),
                    probability_of_success: new_probability,
                });
                info!(
                    "Set {} as probability of {}",
                    movement.as_str(),
                    new_probability
                );
            }
            None => {
                info!(
                    "Tried to set {} to have a probability of {}, but {} is not a safe move",
                    movement.as_str(),
                    new_probability,
                    movement.as_str()
                );
            }
        }
    }

    pub fn update_probability(&mut self, movement: &Movement, amount: isize) {
        match self.moves.get(movement) {
            Some(x) => {
                let new_probability = max(0, x.probability_of_success as isize + amount);
                self.set_probability(movement, new_probability as usize);
            }
            None => {
                info!(
                    "Tried to increment/decrement the probability of {} by {}, but {} is not a safe move",
                    movement.as_str(),
                    amount,
                    movement.as_str()
                );
            }
        }
    }

    pub fn pick_movement(&self) -> Movement {
        match self.moves.iter().max_by_key(|x| x.probability_of_success) {
            Some(x) => x.movement,
            None => Movement::Up,
        }
    }
}

#[test]
fn pick_movement_picks_highest_probability() {
    let mut movement_set = WeightedMovementSet::new();
    movement_set.set_probability(&Movement::Down, 101);
    assert!(movement_set.pick_movement() == Movement::Down);
    movement_set.set_probability(&Movement::Up, 102);
    assert!(movement_set.pick_movement() == Movement::Up);
    movement_set.set_probability(&Movement::Right, 103);
    assert!(movement_set.pick_movement() == Movement::Right);
    movement_set.update_probability(&Movement::Left, -1);
    assert!(movement_set.pick_movement() == Movement::Right);
}

#[test]
fn remove_removes_option() {
    let mut movement_set = WeightedMovementSet::new();
    let size = movement_set.moves.len();
    movement_set.remove(&Movement::Down);
    assert!(movement_set.moves.len() == size - 1);
    movement_set.set_probability(&Movement::Down, 100);
    assert!(movement_set.moves.len() == size - 1);
}
