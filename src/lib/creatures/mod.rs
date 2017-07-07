use std::collections::HashMap;

use super::utils::*;
use super::grid::Grid;

pub enum Property {
    Integer(i64),
    Decimal(f64),
    Text(String),
    Boolean(bool),
}

pub enum Action {
    Move(Position, Position),
    Split(Position, Position),
    Eat(Position, Position),
}

pub enum CreatureType {
    Empty,
    Plant,
}

pub struct Creature {
    pub creature_type: CreatureType,
    pub color: Color,
    pub properties: HashMap<String, Property>,
    action: fn(Position, &mut Grid),
}

impl Creature {
    pub fn act(&mut self, position: Position, grid: &mut Grid) {
        (self.action)(position, grid);
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

fn empty_action(position: Position, grid: &mut Grid) {
}

fn plant_action((x, y): Position, grid: &mut Grid) {
    grid.set_cell((x+1, y+1), get_creature(CreatureType::Plant));
}