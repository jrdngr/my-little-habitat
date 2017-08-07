pub mod plant;

use std::cell::RefCell;

use ::lib::grid::gridcell::{ GridCell, LayeredGridCell };
use ::lib::grid::Grid;
use ::lib::grid::grid_manager::GridManager;

#[derive(Copy, Clone)]
pub enum OrganismType {
    Empty,
    Plant,
}

impl Grid<LayeredGridCell> {
    pub fn new(width: u32, height: u32) -> Grid<RefCell<LayeredGridCell>> {
        let mut data = Vec::new();
        for _ in 0..(width * height) {
            data.push(RefCell::new(LayeredGridCell::new()));
        }
        Grid::with_data(width, height, data)
    }
}

pub fn get(organism_type: OrganismType) -> Option<GridCell> {
    match organism_type {
        OrganismType::Plant => Some(plant::new_plant()),
        _                   => None,
    }
}

pub fn act(grid_manager: &mut GridManager, (x, y, layer): (u32, u32, u32)) {
    let mut cell_type = None;
    if let Some(cell) = grid_manager.get(x, y).borrow_mut().get_layer(layer) {
        cell_type = Some(cell.organism_type);
    }

    if let Some(id) = cell_type {
        match id {
            OrganismType::Plant => plant::plant_action(grid_manager, (x, y)),
            _                   => {},
        }
    }
}