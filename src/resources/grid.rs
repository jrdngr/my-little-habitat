use std::collections::HashMap;
use bevy::ecs::Entity;

pub type GridPosition = (usize, usize);

pub struct Grid {
    width: usize,
    height: usize,
    cells: HashMap<GridPosition, Entity>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            width, 
            height,
            cells: HashMap::with_capacity(width * height),
        }
    }

    pub fn get(&self, position: GridPosition) -> Option<Entity> {
        self.cells.get(&position).copied()
    }

    pub fn set(&mut self, position: GridPosition, entity: Entity) -> Option<Entity> {
        self.cells.insert(position, entity)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
