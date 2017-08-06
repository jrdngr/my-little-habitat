use std::cell::RefCell;

use ::lib::grid::Grid;
use ::lib::grid::gridcell::{ LayeredGridCell, GridCell };
use ::lib::organisms::PLANT;

pub fn new_plant() -> GridCell {
    GridCell::new(String::from(PLANT), [0.0, 0.75, 0.0, 1.0], 0)
}

pub fn plant_action(grid: &Grid<RefCell<LayeredGridCell>>, position: (u32, u32)) {
    let neighbors = grid.get_neighborhood(position);
}