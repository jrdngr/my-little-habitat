use std::mem;
use super::utils::*;
use super::creatures::*;

pub struct Grid {
    pub width: u32,
    pub height: u32,
    data: Vec<Creature>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        let grid_size = width * height;
        let mut grid_data = Vec::with_capacity(grid_size as usize);
        
        for i in 0..grid_size {
            let creature = get_creature(CreatureType::Empty);
            grid_data.push(creature);
        }

        Grid {
            width: width,
            height: height,
            data: grid_data,
        }
    }

    pub fn iter(&self) -> GridIterator {
        GridIterator {
            grid: self,
            next_index: 0,
        }
    }

    pub fn set_cell(&mut self, position: Position, creature: Creature) {
        let index = self.position_to_index(position);
        mem::replace(&mut self.data[index], creature);
    }

    pub fn position_to_index(&self, position: Position) -> usize {
        let (x, y) = position;
        (y * self.height + x) as usize
    }

    pub fn index_to_position(&self, index: usize) -> Position {
         (index as u32 % self.width, index as u32 / self.height)
    }
}

pub struct GridIterator<'a> {
    grid: &'a Grid,
    next_index: usize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = &'a mut Creature;
    
    fn next(&mut self) -> Option<&'a mut Creature> {
        let current_index = self.next_index;
        if current_index >= self.grid.data.len() {
            return None;
        } else {
            self.next_index += 1;
            let creature = &self.grid.data[current_index];
            return Some(&mut creature);
        }
    }
}