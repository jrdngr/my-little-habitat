use super::super::utils::*;
use super::super::grid::Grid;
use super::{ Creature, CreatureType };

pub struct Plant {
    position: Position,
    creature_type: CreatureType,
    color: Color,
}

impl Plant {
    pub fn new(position: Position) -> Self {
        Plant {
            position: position,
            creature_type: CreatureType::Plant,
            color:  [0.0, 0.5, 0.0, 1.0],
        }
    }
}

impl Creature for Plant {
    fn get_position(&self) -> Position {
        self.position
    }

    fn get_type(&self) -> CreatureType {
        self.creature_type
    }

    fn get_color(&self) -> Color {
        self.color
    }

    #[allow(unused_variables)]
    fn act(&mut self, grid: Grid) {
        // Do nothing
    }
}