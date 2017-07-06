pub mod empty;
pub mod plant;

use super::utils::*;
use super::grid::Grid;
use self::empty::Empty;
use self::plant::Plant;

#[derive(Copy, Clone, PartialEq)]
pub enum CreatureType {
    Empty,
    Plant,
    Cow,
    Parasite,
}


pub fn get_creature(position: Position, creature_type: CreatureType) -> Box<Creature> {

    match creature_type {
        CreatureType::Empty       => Box::new(Empty::new(position)),
        CreatureType::Plant       => Box::new(Plant::new(position)),
        CreatureType::Cow         => Box::new(Empty::new(position)),
        CreatureType::Parasite    => Box::new(Empty::new(position)),
    }
}

pub trait Creature {
    fn get_position(&self) -> Position;
    fn get_type(&self) -> CreatureType;
    fn get_color(&self) -> Color;
    fn act(&self, grid: &mut Grid);
}