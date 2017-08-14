use ::lib::utils;
use ::lib::type_aliases::Color;
use ::lib::grid::grid_manager::GridManager;
use ::lib::grid::gridcell::{ GridCell };
use ::lib::organisms::OrganismType;

const LAYER: u32 = 0;
const COLOR: Color = [0.0, 0.60, 0.0, 1.0];

pub fn new_plant() -> GridCell {
    GridCell::new(OrganismType::Plant, COLOR, LAYER)
}

pub fn plant_action(grid_manager: &mut GridManager, (x, y): (u32, u32)) {
    if utils::random_percentage(95) {
         grid_manager.add_to_queue(x, y, LAYER);
    } else {
        let mut new_cell_coordinates = (0, 0);
        let mut set_new_cell = false;
        {
            let neighborhood = grid_manager.get_neighborhood_of_type((x, y), OrganismType::Empty);
            if neighborhood.len() > 0 {
                new_cell_coordinates = utils::random_element(neighborhood.as_slice()).borrow().get_position();
                set_new_cell = true;
            }
        }
        
        if set_new_cell {
            grid_manager.set(new_cell_coordinates.0, new_cell_coordinates.1, new_plant());
            grid_manager.add_to_queue(x, y, LAYER);
        }
    }
}