pub trait GridCell: Default + Clone {
    fn color(&self) -> &str;
}

#[derive(Debug, Clone, Default)]
pub struct SimpleCell {
    color: String,
}

impl GridCell for SimpleCell {
    fn color(&self) -> &str {
        &self.color
    }
}

impl SimpleCell {
    pub fn new(color: &str) -> Self {
        Self { color: String::from(color) }
    }
}
