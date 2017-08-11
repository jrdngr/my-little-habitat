use ::lib::utils::random_element;
use ::lib::type_aliases::Color;
use ::lib::grid::grid_manager::GridManager;
use ::lib::grid::gridcell::{ GridCell };
use ::lib::organisms::OrganismType;

const LAYER: u32 = 0;
const COLOR: Color = [0.0, 0.75, 0.0, 1.0]; 

pub fn new_plant() -> GridCell {
    GridCell::new(OrganismType::Plant, COLOR, LAYER)
}

pub fn plant_action(grid_manager: &mut GridManager, (x, y): (u32, u32)) {
    {
        let neighborhood = grid_manager.get_neighborhood_of_type((x, y), OrganismType::Empty);
    }
    
    grid_manager.add_to_queue(x, y, LAYER);
}