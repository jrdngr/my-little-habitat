use std::ops::Index;
use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

/*
 *  Generic 2-dimensional vector implemented as a flat vector
 */
pub struct Grid<T> {
    width:  u32,
    height: u32,
    data:   Vec<Rc<RefCell<T>>>,
}

impl <T> Grid<T> {
    // Create a new Grid backed by the given vector
    pub fn with_data(width: u32, height: u32, data: Vec<Rc<RefCell<T>>>) -> Grid<T> {
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

    // Unchecked getter for an immutable reference.  Saves some typing.
    pub fn get(&self, x: u32, y: u32) -> Ref<T> {
        self[(x, y)].borrow()
    }
    
    // Unchecked getter for a mutable reference.  Saves some typing.
    pub fn get_mut(&self, x: u32, y: u32) -> RefMut<T> {
        self[(x, y)].borrow_mut()
    }

    // Convenient getter for an immutable reference if you don't trust your coordinates.
    pub fn get_op(&self, x: u32, y: u32) -> Option<Ref<T>> {
        let value = self.data.get(self.coordinates_to_index(x, y));
        match value {
            Some(val) => Some(val.borrow()),
            None      => None,
        }
    }

    // Convenient getter for a mutable reference if you don't trust your coordinates.
    pub fn get_mut_op(&self, x: u32, y: u32) -> Option<RefMut<T>> {
        let value = self.data.get(self.coordinates_to_index(x, y));
        match value {
            Some(val) => Some(val.borrow_mut()),
            None      => None,
        }
    }

    // Gets the Rc contained in the given grid cell so that you can clone it and stuff
    pub fn get_rc(&self, x: u32, y: u32) -> Option<&Rc<RefCell<T>>> {
        self.data.get(self.coordinates_to_index(x, y))
    }
}

// Private methods
impl <T> Grid<T> {
    fn coordinates_to_index(&self, x: u32, y: u32) -> usize {
        (self.width* y + x) as usize
    }
}

// Convenience methods for Grids that use Copy types
impl<T: Copy> Grid<T> {
    pub fn new_filled(width: u32, height: u32, value: T) -> Grid<T> {
        let mut data = Vec::new();
        for _ in 0..(width * height) {
            data.push(Rc::new(RefCell::new(value.clone())));
        }
        Grid { width, height, data }
    }
} 

impl <T> Index<(u32, u32)> for Grid<T> {
    type Output = RefCell<T>;

    fn index(&self, (x, y): (u32, u32)) -> &RefCell<T> {
        &self.data[self.coordinates_to_index(x, y)]
    }
}