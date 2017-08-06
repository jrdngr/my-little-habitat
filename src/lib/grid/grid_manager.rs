use std::cell::RefCell;

use ::lib::grid::gridcell::LayeredGridCell;
use ::lib::grid::turn_queue::TurnQueue;
use ::lib::organisms;

pub struct GridManager {
    grid: Grid<RefCell<LayeredGridCell>>,
    turn_queue: TurnQueue,
}

impl GridManager {
    pub fn new(width: u32, height: u32) -> Self{
        GridManager {
            grid: Grid::new(),
            turn_queue: TurnQueue::new(),
        }
    }
}