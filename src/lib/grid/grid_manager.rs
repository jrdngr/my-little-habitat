use std::cell::RefCell;

use ::lib::type_aliases::Color;
use ::lib::grid::Grid;
use ::lib::grid::gridcell::{ GridCell, LayeredGridCell };
use ::lib::grid::turn_queue::TurnQueue;
use ::lib::organisms;

pub struct GridManager {
    grid: Grid<RefCell<LayeredGridCell>>,
    turn_queue: TurnQueue,
}

impl GridManager {
    pub fn new(width: u32, height: u32) -> Self{
        GridManager {
            grid: Grid::new(width, height),
            turn_queue: TurnQueue::new(),
        }
    }

    pub fn add_to_queue(&mut self, x: u32, y: u32, layer: u32) {
        self.turn_queue.push((x, y, layer));
    }

    pub fn get(&self, x: u32, y: u32) -> &RefCell<LayeredGridCell> {
        &self.grid[(x, y)]
    }

    pub fn set(&mut self, x: u32, y: u32, new_grid_cell: GridCell) {
        self.grid[(x, y)].borrow_mut().set(new_grid_cell);
    }

    pub fn color_enumerator(&self) -> ColorEnumerator {
        ColorEnumerator {
            current_index: 0,
            grid: &self.grid,
        }
    }

    pub fn step(&mut self) {
        for _ in 0..self.turn_queue.len() {
            let next = self.turn_queue.pop();
            match next {
                Some(position) => {
                    organisms::act(self, position);
                },
                None => return,
            }
        }
    }
}

pub struct ColorEnumerator<'a> {
    current_index: usize,
    grid: &'a Grid<RefCell<LayeredGridCell>>,
}

impl<'a> Iterator for ColorEnumerator<'a> {
    type Item = ((u32, u32), Color);
    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.grid.index_to_coordinates(self.current_index);
        let cell = self.grid.get(x, y);
        self.current_index += 1;
        match cell {
            Some(c) => {
                return Some(((x, y), c.borrow().get_color()));
            }
            None => None
        }
    }
}