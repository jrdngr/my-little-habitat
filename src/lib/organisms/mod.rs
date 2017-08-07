pub mod plant;

use std::cell::RefCell;

use ::lib::grid::gridcell::{ GridCell, LayeredGridCell };
use ::lib::grid::Grid;
use ::lib::grid::grid_manager::GridManager;

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

pub fn act(grid_manager: &mut GridManager, (x, y, layer): (u32, u32, u32)) {
    let mut cell_type = None;
    if let Some(cell) = grid_manager.get(x, y).borrow_mut().get_layer(layer) {
        match cell.id.as_ref() {
            PLANT => cell_type = Some(PLANT),
            _ => {},
        }
    }

    if let Some(id) = cell_type {
        match id {
            PLANT => plant::plant_action(grid_manager, (x, y)),
            _     => {},
        }
    }
}