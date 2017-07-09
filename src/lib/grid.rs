use std::mem;
use std::iter;
use super::utils::*;
use super::creatures::*;

#[derive(Copy, Clone)]
pub enum Action {
    Move(Position, Position),
    Split(Position, Position),
    Eat(Position, Position),
    Kill(Position),
    Idle,
}

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

    pub fn step(&mut self) {
        for i in 0..self.data.len() {
            let position = self.index_to_position(i);
            let action = self.data[i].act(position, self);
            match action {
                Action::Move(creature_pos, other_pos)   => {
                    let other_index = self.position_to_index(other_pos);
                    mem::replace(&mut self.data[i], get_creature(CreatureType::Empty));
                    mem::replace(&mut self.data[other_index], get_creature(CreatureType::Plant));
                },
                Action::Split(creature_pos, other_pos)  => {
                    let other_index = self.position_to_index(other_pos);
                    if other_index >= self.data.len() {
                        return;
                    }
                    mem::replace(&mut self.data[other_index], get_creature(CreatureType::Plant));
                },
                Action::Eat(creature_pos, other_pos)    => {
                    let other_index = self.position_to_index(other_pos);
                    mem::replace(&mut self.data[i], get_creature(CreatureType::Empty));
                    mem::replace(&mut self.data[other_index], get_creature(CreatureType::Plant));
                },
                Action::Kill(creature_pos)              => {
                    mem::replace(&mut self.data[i], get_creature(CreatureType::Empty));
                },
                Action::Idle                            => {},
            }

        }
    }

    pub fn get_color_grid(&self) -> Vec<Color> {
        let grid_size = self.width as usize * self.height as usize;
        let mut color_grid = Vec::with_capacity(grid_size);
        for i in 0..grid_size {
            color_grid.push(self.data[i].color);
        }
        color_grid
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