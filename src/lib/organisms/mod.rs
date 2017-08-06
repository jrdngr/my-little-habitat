pub mod plant;

use std::cell::RefCell;

use ::lib::gridcell::{ GridCell, LayeredGridCell };
use ::lib::grid::Grid;

pub const EMPTY: &'static str = "empty";
pub const PLANT: &'static str = "plant";

impl Grid<LayeredGridCell> {
    pub fn new(width: u32, height: u32) -> Grid<RefCell<LayeredGridCell>> {
        let mut data = Vec::new();
        for _ in 0..(width * height) {
            data.push(RefCell::new(LayeredGridCell::new()));
        }
        Grid::with_data(width, height, data)
    }
}

pub fn get(organism_id: &String) -> Option<GridCell> {
    match organism_id.as_ref() {
        PLANT   => Some(plant::new_plant()),
        _       => None,
    }
}

pub fn act(grid: &mut Grid<RefCell<LayeredGridCell>>, (x, y, layer): (u32, u32, u32)) {
    match grid[(x, y)].borrow_mut().get_layer(layer) {
        Some(cell)  => match cell.id.as_ref() {
            PLANT   => plant::plant_action(grid, (x, y)),
            _       => {},
        },
        None        => {},
    }
}