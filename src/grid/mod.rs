struct Point {
    pub x: u16,
    pub y: u16,
}

struct Cell<T: Clone> {
    pub position: Point,
    pub contents: T,
}

impl<T: Clone> Cell<T> {
    pub fn set_content(&mut self, new_contents: T) {
        self.contents = new_contents;
    }
}

pub struct Grid<T: Clone> {
    pub width: u16,
    pub height: u16,
    data: Vec<Cell<T>>,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: u16, height: u16, fill_with: T) -> Grid<T> {
        let vec_size: u16 = width * height;
        let mut grid_data: Vec<Cell<T>> = Vec::new();

        for i in 0..vec_size {
            let point = Point {
                x: i % width,
                y: i / height,
            };

            let cell = Cell {
                position: point,
                contents: fill_with.clone(),
            };

            grid_data.push(cell);
        }

        Grid {
            width: width,
            height: height,
            data: grid_data,
        }
    }

    pub fn get_cells(&self) -> &Vec<Cell<T>> {
        &self.data
    }
}