use ::lib::grid::grid_manager::GridManager;
use ::lib::grid::gridcell::{ GridCell };
use ::lib::organisms::OrganismType;

pub fn new_plant() -> GridCell {
    GridCell::new(OrganismType::Plant, [0.0, 0.75, 0.0, 1.0], 0)
}

pub fn plant_action(grid_manager: &mut GridManager, (x, y): (u32, u32)) {
    println!("woo!");
    grid_manager.add_to_queue(x, y, 0);
}