pub mod gridcell;
pub mod turn_queue;
pub mod grid_manager;

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
 *  Neighborhood methods
 */
impl <T> Grid<T> {
    pub fn get_neighborhood<'a>(&'a self, (x, y): (u32, u32)) -> Vec<&'a T> {
        let mut neighborhood: Vec<&T> = Vec::new();

        let top_free = y > 0;
        let bottom_free = y < self.height - 1;
        let left_free = x > 0;
        let right_free = x < self.width - 1;

        if top_free {
            if left_free {
                let pos = (x-1, y-1);
                neighborhood.push(&self[pos]);
            }
            let pos = (x, y-1);
            neighborhood.push(&self[pos]);
            if right_free {
                let pos = (x+1, y-1);
                neighborhood.push(&self[pos]);
            }
        }

        if left_free {
            let pos = (x-1, y);
            neighborhood.push(&self[pos]);
        }
        if right_free {
            let pos = (x+1, y);
            neighborhood.push(&self[pos]);
        }

        if bottom_free {
            if left_free {
                let pos = (x-1, y+1);
                neighborhood.push(&self[pos]);
            }
            let pos = (x, y+1);
            neighborhood.push(&self[pos]);
            if right_free {
                let pos = (x+1, y+1);
                neighborhood.push(&self[pos]);
            }
        }

        neighborhood
    }
}

/*  
 *  Indexing methods
 */
impl <T> Grid<T> {
    pub fn coordinates_to_index(&self, x: u32, y: u32) -> usize {
        (self.width* y + x) as usize
    }

    pub fn index_to_coordinates(&self, index: usize) -> (u32, u32) {
        (index as u32 % self.width, index as u32 / self.height)
    }
}

/*
 *  Convenience methods for Grids that use Copy types
 */
impl<T: Clone> Grid<T> {
    pub fn new_filled(width: u32, height: u32, value: T) -> Grid<T> {
        let mut data = Vec::new();
        for _ in 0..(width * height) {
            data.push(value.clone());
        }
        Grid { width, height, data }
    }
}

/*
 *  Index
 */
impl <T> Index<(u32, u32)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (u32, u32)) -> &T {
        &self.data[self.coordinates_to_index(x, y)]
    }
}

/*
 *  Iteration
 */
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