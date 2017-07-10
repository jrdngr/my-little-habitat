use std::mem;
use std::collections::VecDeque;

use super::utils::*;
use super::creature::*;

pub enum Action {
    Set(Position, Creature),
    Clear(Position),
    Queue(Position),
    Idle,
}

pub struct Grid {
    pub width: u32,
    pub height: u32,
    data: Vec<Creature>,
    turn_queue: VecDeque<usize>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        let grid_size = width * height;
        let mut grid_data = Vec::with_capacity(grid_size as usize);
        
        for _ in 0..grid_size {
            let creature = get(CreatureType::Empty);
            grid_data.push(creature);
        }

        Grid {
            width: width,
            height: height,
            data: grid_data,
            turn_queue: VecDeque::new()
        }
    }

    pub fn step(&mut self) {
        let start_length = self.turn_queue.len();
        let mut current = 0;

        while current < start_length {
            current += 1;
            match self.turn_queue.pop_front() {
                Some(index) => {
                    let position = self.index_to_position(index);
                    let neighbors = self.get_neighbors(position);
                    let actions = self.data[index].act(&neighbors);
                    for action in actions {
                        match action {
                            Action::Set(pos, creature) => {
                                self.set_cell(pos, creature);
                            },
                            Action::Clear(pos) => {
                                let index = self.position_to_index(pos);
                                mem::replace(&mut self.data[index], get(CreatureType::Empty));
                            },
                            Action::Queue(pos) => {
                                let index = self.position_to_index(pos);
                                self.turn_queue.push_back(index);
                            },
                            Action::Idle => {},
                        }
                    }
                },
                None => break,
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

    pub fn get_cell(&mut self, position: Position) -> &Creature {
        &self.data[self.position_to_index(position)]
    }

    pub fn set_cell(&mut self, position: Position, creature: Creature) {
        let index = self.position_to_index(position);
        mem::replace(&mut self.data[index], creature);
        self.turn_queue.push_back(index);
    }

    pub fn position_to_index(&self, (x, y): Position) -> usize {
        let index = (y * self.height + x) as usize;
        if index >= self.data.len() {
            panic!("Position ({},{}) gave invalid index: {}", x, y, index);
        }
        index
    }

    pub fn index_to_position(&self, index: usize) -> Position {
         (index as u32 % self.width, index as u32 / self.height)
    }

    pub fn get_neighbors(&mut self, (x, y): Position) -> Neighbors {
        let mut neighbor_list: Vec<(CreatureType, Position)> = Vec::new();

        let top_free = y > 0;
        let bottom_free = y < self.height - 1;
        let left_free = x > 0;
        let right_free = x < self.width - 1;

        if top_free {
            if left_free {
                let pos = (x-1, y-1);
                neighbor_list.push((self.get_cell(pos).creature_type, pos));
            }
            let pos = (x, y-1);
            neighbor_list.push((self.get_cell(pos).creature_type, pos));
            if right_free {
                let pos = (x+1, y-1);
                neighbor_list.push((self.get_cell(pos).creature_type, pos));
            }
        }

        if left_free {
            let pos = (x-1, y);
            neighbor_list.push((self.get_cell(pos).creature_type, pos));
        }
        if right_free {
            let pos = (x+1, y);
            neighbor_list.push((self.get_cell(pos).creature_type, pos));
        }

        if bottom_free {
            if left_free {
                let pos = (x-1, y+1);
                neighbor_list.push((self.get_cell(pos).creature_type, pos));
            }
            let pos = (x, y+1);
            neighbor_list.push((self.get_cell(pos).creature_type, pos));
            if right_free {
                let pos = (x+1, y+1);
                neighbor_list.push((self.get_cell(pos).creature_type, pos));
            }
        }

        Neighbors {
            my_pos: (x, y),
            neighbors: neighbor_list,            
        }
    }
}

pub struct Neighbors {
    my_pos: Position,
    neighbors: Vec<(CreatureType, Position)>,
}

impl Neighbors {
    pub fn pos(&self) -> Position {
        self.my_pos
    }

    pub fn get_neighbor(&self, index: usize) -> (CreatureType, Position) {
        self.neighbors[index]
    }
    
    pub fn of_type(&self, creature_type: CreatureType) -> Vec<(CreatureType, Position)> {
        let mut creature_list = Vec::new();
        for i in 0..self.neighbors.len() {
            if self.neighbors[i].0 == creature_type {
                creature_list.push(self.neighbors[i])
            }
        }
        creature_list
    }
}