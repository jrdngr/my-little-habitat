pub mod empty;
pub mod plant;

use std::cell::RefCell;

use ::lib::gridcell::GridCell;
use ::lib::grid::Grid;

pub const EMPTY: &'static str = "empty";
pub const PLANT: &'static str = "plant";

impl Grid<RefCell<GridCell>> {
    pub fn new(width: u32, height: u32) -> Grid<RefCell<GridCell>> {
        let mut data = Vec::new();
        for _ in 0..(width * height) {
            data.push(RefCell::new(empty::new_empty()));
        }
        Grid::with_data(width, height, data)
    }
}

pub fn get(organism_id: &String) -> GridCell {
    match organism_id.as_ref() {
        EMPTY   => empty::new_empty(),
        PLANT   => plant::new_plant(),
        _       => empty::new_empty(),
    }
}

pub fn act(grid: &Grid<RefCell<GridCell>>, position: (u32, u32)) {
    match grid[position].borrow().id.as_ref() {
        PLANT   => plant::plant_action(grid, position),
        _       => {},
    }
}