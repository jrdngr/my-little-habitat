use super::utils::Color;
use super::grid::Grid;

pub trait Actor {
    fn act(&mut self, grid: Grid<Creature>);
}

#[derive(Clone, Debug)]
pub struct Creature {
    pub name: String,
    pub color: Color,
}