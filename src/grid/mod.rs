use std::mem;
use super::utils::*;
use super::creatures::*;

pub struct Grid {
    pub width: u16,
    pub height: u16,
    data: Vec<Box<Creature>>,
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        let grid_size = width * height;
        let mut grid_data = Vec::with_capacity(grid_size as usize);
        
        for i in 0..grid_size {
            let position = (i % width, i / height);

            let creature = get_creature(position, CreatureType::Empty);

            grid_data.push(creature);
        }

        Grid {
            width: width,
            height: height,
            data: grid_data,
        }
    }

    pub fn get_creatures(&self) -> &Vec<Box<Creature>> {
        &self.data
    }

    pub fn set_creature(&mut self, position: Position, creature_type: CreatureType) {
        let (x, y) = position;
        let index = y * self.height + x;

        mem::replace(&mut self.data[index as usize], get_creature(position, creature_type));
    }
}