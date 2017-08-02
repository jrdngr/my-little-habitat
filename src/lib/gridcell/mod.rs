use ::lib::grid::Grid;

pub trait GridCell {
    fn get_color(&self) -> [f32; 4];
    fn act(&self, grid: &Grid<&GridCell>);
}

pub struct Empty {
    disp: char,   
}

impl Empty {
    pub fn new() -> Self {
        Self { disp: '0' }
    }
}

impl GridCell for Empty {
    fn get_color(&self) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn act(&self, grid: &Grid<&GridCell>) {
        // Do nothing;
    }
}