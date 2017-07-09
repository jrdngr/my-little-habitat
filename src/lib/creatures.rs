extern crate rand;

use std::collections::HashMap;
use rand::Rng;

use super::utils::*;
use super::grid::Grid;
use super::grid::Action;

pub enum Property {
    Integer(i64),
    Decimal(f64),
    Text(String),
    Boolean(bool),
}

pub enum CreatureType {
    Empty,
    Plant,
}

pub struct Creature {
    pub creature_type: CreatureType,
    pub color: Color,
    pub properties: HashMap<String, Property>,
    action: fn(Position, &Grid) -> Action,
}

impl Creature {
    pub fn act(&self, position: Position, grid: &Grid) -> Action {
        (self.action)(position, grid)
    }
}

pub fn get_creature(creature_type: CreatureType) -> Creature {
    match creature_type {
        CreatureType::Empty   => {
            Creature {
                creature_type: CreatureType::Empty,
                color: [0.0, 0.0, 0.0, 1.0],
                properties: HashMap::new(),
                action: empty_action,
            }
        }
        CreatureType::Plant => {
            Creature {
                creature_type: CreatureType::Plant,
                color: [0.0, 0.5, 0.0, 1.0],
                properties: HashMap::new(),
                action: plant_action,
            }
        }
    }
}

fn empty_action(position: Position, grid: &Grid) -> Action {
    Action::Idle
}

fn plant_action((x, y): Position, grid: &Grid) -> Action {
    let x = x as i64;
    let y = y as i64;
    let mut new_dir: (i64, i64) =  match rand::thread_rng().gen_range(0, 4) {
        0   => (x+1, y),
        1   => (x-1, y),
        2   => (x, y+1),
        3   => (x, y-1),
        _   => (0, 0),
    };

    if new_dir.0 < 0 || new_dir.0 >= grid.width as i64 {
        return Action::Idle;
    }
    if new_dir.1 < 0 || new_dir.1 >= grid.height as i64 {
        return Action::Idle;
    }

    Action::Split((x as u32, y as u32), (new_dir.0 as u32, new_dir.1 as u32))
}