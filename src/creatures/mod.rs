use super::utils::Color;
use super::grid::Grid;

pub trait Actor {
    fn act(&mut self, grid: Grid<Creature>);
}

pub struct Creature {
    name: String,
    color: Color,
}