use ::lib::utils;
use ::lib::type_aliases::Color;
use ::lib::grid::grid_manager::GridManager;
use ::lib::grid::gridcell::{ GridCell };
use ::lib::organisms::OrganismType;

const LAYER: u32 = 0;
const COLOR: Color = [0.0, 0.0, 1.0, 1.0];

pub fn new_cow() -> GridCell {
    GridCell::new(OrganismType::Cow, COLOR, LAYER)
}

pub fn cow_action(grid_manager: &mut GridManager, (x, y): (u32, u32)) {
    if utils::random_percentage(75.0) {
         grid_manager.add_to_queue(x, y, LAYER);
    } else {
        let mut new_cell_coordinates = (0, 0);
        let mut set_new_cell = false;
        {
            let plant_neighborhood = grid_manager.get_neighborhood_of_type((x, y), OrganismType::Plant);
            if plant_neighborhood.len() > 0 {
                new_cell_coordinates = utils::random_element(plant_neighborhood.as_slice()).1.borrow().get_position();
                set_new_cell = true;
            }
        }
        
        if set_new_cell {
            grid_manager.set(new_cell_coordinates.0, new_cell_coordinates.1, new_cow());
            grid_manager.clear(x, y, LAYER);
            grid_manager.add_to_queue(x, y, LAYER);
        }
    }
}