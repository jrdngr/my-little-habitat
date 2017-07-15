mod empty;
mod wall;
mod plant;
mod deadplant;
mod vine;
mod cow;
mod parasite;

use std::collections::HashMap;

use super::utils::*;
use super::grid::Action;
use super::grid::neighbors::Neighbors;

#[derive(Clone)]
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
    DeadPlant,
    Vine,
    Cow,
    Parasite,
}

pub struct Creature {
    pub creature_type: CreatureType,
    pub color: Color,
    pub energy: i64,
    pub sleep: i64,
    pub properties: HashMap<String, Property>,
    action: fn(&mut Creature, &Neighbors) -> Vec<Action>,
}

impl Creature {
    pub fn act(&mut self, neighbors: &Neighbors) -> Vec<Action> {
        (self.action)(self, neighbors)
    }
}

impl Clone for Creature {
    fn clone(&self) -> Self {
        Creature {
            creature_type: self.creature_type,
            color: self.color,
            energy: self.energy,
            sleep: self.sleep,
            properties: self.properties.clone(),
            action: self.action,
        }
    }
}

pub fn get(creature_type: CreatureType) -> Creature {
    match creature_type {
        CreatureType::Empty     => empty::new(),
        CreatureType::Wall      => wall::new(),
        CreatureType::Plant     => plant::new(),
        CreatureType::DeadPlant => deadplant::new(),
        CreatureType::Vine      => vine::new(),
        CreatureType::Cow       => cow::new(),
        CreatureType::Parasite  => parasite::new(),
    }
}
