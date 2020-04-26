pub mod cell;

pub use self::cell::{GridCell, SimpleCell};

pub struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T: GridCell> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self::from_cell(width, height, Default::default())
    }

    pub fn from_cell(width: usize, height: usize, cell: T) -> Self {
        let cells = vec![cell; width * height];
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    pub fn cell(&self, x: usize, y: usize) -> Option<&T> {
        let index = self.cell_index(x, y);
        self.cells.get(index)
    }

    pub fn set_cell(&mut self, x: usize, y: usize, new_cell: T) {
        let index = self.cell_index(x, y);
        if let Some(cell) = self.cells.get_mut(index) {
            *cell = new_cell;
        }
    }
    
    pub fn cell_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
    
    pub fn cell_coordinates(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.height;

        (x, y)
    }
}
