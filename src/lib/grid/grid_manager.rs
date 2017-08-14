use std::cell::RefCell;
use std::cmp;

use ::lib::type_aliases::Color;
use ::lib::grid::Grid;
use ::lib::grid::gridcell::{ GridCell, LayeredGridCell };
use ::lib::grid::turn_queue::TurnQueue;
use ::lib::organisms;
use ::lib::organisms::OrganismType;

pub type GridType = RefCell<LayeredGridCell>;

pub struct GridManager {
    grid: Grid<GridType>,
    turn_queue: TurnQueue,
}

impl GridManager {
    pub fn new(width: u32, height: u32) -> Self {
        GridManager {
            grid: Grid::new(width, height),
            turn_queue: TurnQueue::new(),
        }
    }

    pub fn add_to_queue(&mut self, x: u32, y: u32, layer: u32) {
        self.turn_queue.push((x, y, layer));
    }

    pub fn get(&self, x: u32, y: u32) -> &GridType {
        &self.grid[(x, y)]
    }

    pub fn set(&mut self, x: u32, y: u32, new_grid_cell: GridCell) {
        self.turn_queue.push((x, y, new_grid_cell.layer));
        self.grid[(x, y)].borrow_mut().set(new_grid_cell);
    }

    pub fn color_enumerator(&self) -> ColorEnumerator {
        ColorEnumerator {
            current_index: 0,
            grid: &self.grid,
        }
    }

    pub fn update(&mut self) {
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

    pub fn get_neighborhood_coordinates(&self, (x, y): (u32, u32), radius: u32) -> Vec<(u32, u32)> {
        let mut result = Vec::new();
        let x_min = x.saturating_sub(radius);
        let x_max = cmp::min(x + radius, self.grid.width() - 1) + 1;
        let y_min = y.saturating_sub(radius);
        let y_max = cmp::min(y + radius, self.grid.height() - 1) + 1;

        for j in y_min..y_max {
            for i in x_min..x_max {
                result.push((i, j));
            }
        }

        result
    }

    pub fn get_neighborhood_with_radius(&self, (x, y): (u32, u32), radius: u32) -> Vec<((u32, u32), &GridType)> {
        let mut result = Vec::new();
        for cell_coords in self.get_neighborhood_coordinates((x, y), radius) {
            result.push((cell_coords, &self.grid[cell_coords]));
        }
        result
    }

    pub fn get_neighborhood_of_type_with_radius(&self, (x, y): (u32, u32), radius: u32, organism_type: OrganismType) -> Vec<((u32, u32), &GridType)> {
        let mut result = Vec::new();
        let organism_layer = organisms::get_layer(organism_type);
        match organism_layer {
            Some(layer) => {
                for cell_coords in self.get_neighborhood_coordinates((x, y), radius) {
                    let cell = &self.grid[cell_coords];
                    let cell_type = cell.borrow().get_layer(layer).unwrap().organism_type;
                    if cell_type == organism_type {
                        result.push((cell_coords, &self.grid[cell_coords]));
                    }
                }
            },
            None => {
               for cell_coords in self.get_neighborhood_coordinates((x, y), radius) {
                    let cell = &self.grid[cell_coords];
                    if cell.borrow().is_empty() {
                        result.push((cell_coords, &self.grid[cell_coords]));
                    }
                }
            },
        }
        result
    }
     pub fn get_neighborhood(&self, (x, y): (u32, u32)) -> Vec<((u32, u32), &GridType)> {
         self.get_neighborhood_with_radius((x, y), 1)
     }

     pub fn get_neighborhood_of_type(&self, (x, y): (u32, u32), organism_type: OrganismType) -> Vec<((u32, u32), &GridType)> {
         self.get_neighborhood_of_type_with_radius((x, y), 1, organism_type)
     }

}

pub struct ColorEnumerator<'a> {
    current_index: usize,
    grid: &'a Grid<GridType>,
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
