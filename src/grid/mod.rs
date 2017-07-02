struct Point {
    pub x: u16,
    pub y: u16,
}

struct Cell<T> {
    pub position: Point,
    pub contents: T,
}

impl<T> Cell<T> {
    pub fn set_content(&mut self, new_contents: T) {
        self.contents = new_contents;
    }
}

pub struct Grid<T> {
    pub width: u16,
    pub height: u16,
    data: Vec<Cell<T>>,
}

impl<T> Grid<T> {
    pub fn new(width: u16, height: u16) -> Grid<T> {
        Grid {
            width: width,
            height: height,
            data: Vec::with_capacity((width * height) as usize),
        }
    }
}