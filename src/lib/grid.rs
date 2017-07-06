use std::mem;
use super::utils::*;
use super::creatures::*;

pub struct Grid {
    pub width: u32,
    pub height: u32,
    data: Vec<Box<Creature>>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
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

    pub fn get_creature_list(&mut self) -> &mut Vec<Box<Creature>> {
        &mut self.data
    }

    pub fn get_creature(&self, position: Position) -> &Box<Creature> {
        &self.data[self.get_index(position)]
    }

    pub fn get_creature_by_index(&self, index: usize) -> &Box<Creature> {
        &self.data[index]
    }

    pub fn set_creature(&mut self, position: Position, creature_type: CreatureType) {
        let index = self.get_index(position);
        mem::replace(&mut self.data[index], get_creature(position, creature_type));
    }

    fn get_index(&self, position: Position) -> usize {
        let (x, y) = position;
        (y * self.height + x) as usize
    }

}