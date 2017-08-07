use std::collections::HashMap;

use ::lib::type_aliases::Color;

pub const EMPTY_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

#[derive(Clone)]
pub enum Property {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Text(String),
}

pub struct GridCell {
    pub id: String,
    pub color: Color,
    pub layer: u32,
    pub properties: HashMap<String, Property>,
}

impl GridCell {
    pub fn new(id: String, color: Color, layer: u32) -> Self {
        Self { 
            id: id,
            color: color,
            layer: layer,
            properties: HashMap::new(),
        }
    }

    pub fn with_properties(id: String, color: Color, layer: u32, properties: HashMap<String, Property>) -> Self {
        Self { id, color, layer, properties }
    }
}

pub struct LayeredGridCell {
    layers: HashMap<u32, GridCell>,
}

impl LayeredGridCell {
    pub fn new() -> Self {
        LayeredGridCell {
            layers: HashMap::new(),
        }
    }

    pub fn get_color(&self) -> Color {
        match self.layers.keys().max() {
            Some(n) => match self.layers.get(n) {
                Some(cell) => cell.color,
                None => EMPTY_COLOR,
            },
            None => EMPTY_COLOR, 
        }
    }

    pub fn get_layer(&self, layer: u32) -> Option<&GridCell> {
        self.layers.get(&layer)
    }

    pub fn set(&mut self, new_grid_cell: GridCell) {
        self.layers.insert(new_grid_cell.layer, new_grid_cell);
    }
}