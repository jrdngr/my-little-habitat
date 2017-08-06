use std::collections::HashMap;

use ::lib::type_aliases::Color;

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
    pub properties: HashMap<String, Property>,
}

impl GridCell {
    pub fn new(id: String, color: Color) -> Self {
        Self { 
            id: id,
            color: color,
            properties: HashMap::new(),
        }
    }

    pub fn with_properties(id: String, color: Color, properties: HashMap<String, Property>) -> Self {
        Self { id, color, properties }
    }

    pub fn set(&mut self, new_grid_cell: GridCell) {
        self.id = new_grid_cell.id;
        self.color = new_grid_cell.color;
        self.properties = new_grid_cell.properties;
    }
}