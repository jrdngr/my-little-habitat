use ::lib::gridcell::GridCell;
use ::lib::organisms::EMPTY;

pub fn new_empty() -> GridCell {
    GridCell::new(String::from(EMPTY), [0.0, 0.0, 0.0, 1.0])
}