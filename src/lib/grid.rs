use std::ops::Index;

/*
 *  Generic 2-dimensional reference vector with reference counted elements 
 *  and implemented as a flat vector.
 */
pub struct Grid<T> {
    width:  u32,
    height: u32,
    data:   Vec<T>,
}

/*
 *  Public methods
 */
impl <T> Grid<T> {
    // Create a new Grid backed by the given vector
    pub fn with_data(width: u32, height: u32, data: Vec<T>) -> Grid<T> {
        if data.len() as u32 != width * height {
            panic!("Grid (width * height) size must equal data.len(). Grid: {}, data.len(): {}", width * height, data.len());
        } 
        Grid { width, height, data }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&T> {
        self.data.get(self.coordinates_to_index(x, y))
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator {
            data: &self.data,
            current: 0,
        }
    }
}

/*  
 *  Private methods
 */
impl <T> Grid<T> {
    fn coordinates_to_index(&self, x: u32, y: u32) -> usize {
        (self.width* y + x) as usize
    }
}

// Convenience methods for Grids that use Copy types
impl<T: Clone> Grid<T> {
    pub fn new_filled(width: u32, height: u32, value: T) -> Grid<T> {
        let mut data = Vec::new();
        for _ in 0..(width * height) {
            data.push(value.clone());
        }
        Grid { width, height, data }
    }
}

impl <T> Index<(u32, u32)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (u32, u32)) -> &T {
        &self.data[self.coordinates_to_index(x, y)]
    }
}

impl <T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
} 

pub struct GridIterator<'a, T: 'a> {
    data: &'a Vec<T>,
    current: usize,
}

impl <'a, T> Iterator for GridIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.data.len() {
            None
        } else {
            self.current += 1;
            Some(&self.data[self.current - 1])
        }
    }
}