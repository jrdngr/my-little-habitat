use ::lib::utils::*;
use ::lib::creature::*;

pub struct Neighbors {
    pub my_pos: Position,
    pub neighbors: Vec<(CreatureType, Position)>,
}

impl Neighbors {
    pub fn pos(&self) -> Position {
        self.my_pos
    }

    pub fn get_neighbor(&self, index: usize) -> (CreatureType, Position) {
        self.neighbors[index]
    }

    pub fn all(&self) -> &Vec<(CreatureType, Position)> {
        &self.neighbors    
    }

    pub fn of_types(&self, creature_types: &[CreatureType]) -> Vec<(CreatureType, Position)> {
        let mut creature_list = Vec::new();
        for i in 0..self.neighbors.len() {
            for creature_type in creature_types {
                if self.neighbors[i].0 == *creature_type {
                    creature_list.push(self.neighbors[i])
                }
            }
        }
        creature_list
    }

    pub fn of_type(&self, creature_type: CreatureType) -> Vec<(CreatureType, Position)> {
        self.of_types(&[creature_type])
    }
}