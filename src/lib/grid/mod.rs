pub mod turn_queue;
pub mod neighbors;

use std::mem;

use super::utils::*;
use super::creature::*;
use turn_queue::TurnQueue;
use neighbors::Neighbors;

pub enum Action {
    Set(Position, Creature),
    Clear(Position),
    Queue(Position),
    QueueNeighbors,
    Idle,
}

pub struct Grid {
    pub width: u32,
    pub height: u32,
    data: Vec<Creature>,
    turn_queue: TurnQueue,
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
            turn_queue: TurnQueue::new(),
        }
    }

    pub fn reset(&mut self) {
        let grid_size = self.width * self.height;
        let mut grid_data = Vec::with_capacity(grid_size as usize);
        for _ in 0..grid_size {
            let creature = get(CreatureType::Empty);
            grid_data.push(creature);
        }
        self.data = grid_data;
    }

    pub fn step(&mut self) {
        let start_length = self.turn_queue.len();
        let mut current = 0;

        while current < start_length {
            current += 1;
            match self.turn_queue.pop() {
                Some(index) => {
                    let position = self.index_to_position(index);
                    let neighbors = self.get_neighbors(position);
                    let actions = self.data[index].act(&neighbors);
                    for action in actions {
                        match action {
                            Action::Set(pos, creature) => {
                                self.set_cell(pos, creature);
                                self.queue_neighbors(&neighbors);
                            },
                            Action::Clear(pos) => {
                                let index = self.position_to_index(pos);
                                mem::replace(&mut self.data[index], get(CreatureType::Empty));
                            },
                            Action::Queue(pos) => {
                                let index = self.position_to_index(pos);
                                self.turn_queue.push(index);
                            },
                            Action::QueueNeighbors => {
                                self.queue_neighbors(&neighbors);
                            }
                            Action::Idle => {},
                        }
                    }
                },
                None => break,
            }
        }
    }

    pub fn color_enumerator(&self) -> ColorEnumerator {
        ColorEnumerator {
            current_index: 0,
            data: &self.data,
        }
    }


    pub fn get_cell(&mut self, position: Position) -> &Creature {
        &self.data[self.position_to_index(position)]
    }

    pub fn set_cell(&mut self, position: Position, creature: Creature) {
        let index = self.position_to_index(position);
        mem::replace(&mut self.data[index], creature);
        self.turn_queue.push(index);
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

    fn queue_neighbors(&mut self, neighbors: &Neighbors) {
        for &(_, pos) in neighbors.all() {
            let index = self.position_to_index(pos);
            self.turn_queue.push(index);
        }
    }
}

pub struct ColorEnumerator<'a> {
    current_index: usize,
    data: &'a Vec<Creature>,
}

impl<'a> Iterator for ColorEnumerator<'a> {
    type Item = (usize, Color);
    fn next(&mut self) -> Option<(usize, Color)> {
        let creature = self.data.get(self.current_index);
        self.current_index += 1;
        match creature {
            Some(cr) => {
                return Some((self.current_index - 1, cr.color));
            }
            None => None
        }
    }
}

