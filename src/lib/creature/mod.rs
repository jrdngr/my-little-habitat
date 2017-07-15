mod empty;
mod wall;
mod plant;
mod ivy;
mod cow;
mod virus;

use std::collections::HashMap;

use super::utils::*;
use super::grid::Action;
use super::grid::neighbors::Neighbors;

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
    Ivy,
    Cow,
    Virus,
}

pub struct Creature {
    pub creature_type: CreatureType,
    pub color: Color,
    pub energy: u64,
    pub properties: HashMap<String, Property>,
    action: fn(&mut Creature, &Neighbors) -> Vec<Action>,
}

impl Creature {
    pub fn act(&mut self, neighbors: &Neighbors) -> Vec<Action> {
        (self.action)(self, neighbors)
    }
}

pub fn get(creature_type: CreatureType) -> Creature {
    match creature_type {
        CreatureType::Empty     => empty::new(),
        CreatureType::Wall      => wall::new(),
        CreatureType::Plant     => plant::new(),
        CreatureType::Ivy       => ivy::new(),
        CreatureType::Cow       => cow::new(),
        CreatureType::Virus     => virus::new(),
    }
}
