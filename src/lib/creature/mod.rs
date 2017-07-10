mod empty;
mod wall;
mod plant;
mod snake;
mod cow;
mod virus;

use std::collections::HashMap;

use super::utils::*;
use super::grid::{Action, Neighbors};

pub enum Property {
    Integer(i64),
    Decimal(f64),
    Text(String),
    Boolean(bool),
}

#[derive(Copy, Clone, PartialEq)]
pub enum CreatureType {
    Empty,
    Wall,
    Plant,
    Snake,
    Cow,
    Virus,
}

pub struct Creature {
    pub creature_type: CreatureType,
    pub color: Color,
    pub properties: HashMap<String, Property>,
    action: fn(&Neighbors) -> Vec<Action>,
}

impl Creature {
    pub fn act(&self, neighbors: &Neighbors) -> Vec<Action> {
        (self.action)(neighbors)
    }
}

pub fn get(creature_type: CreatureType) -> Creature {
    match creature_type {
        CreatureType::Empty     => empty::new(),
        CreatureType::Wall      => wall::new(),
        CreatureType::Plant     => plant::new(),
        CreatureType::Snake     => snake::new(),
        CreatureType::Cow       => cow::new(),
        CreatureType::Virus     => virus::new(),
    }
}
