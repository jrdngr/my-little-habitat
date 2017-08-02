use std::ops::Index;
use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

/*
 *  Generic 2-dimensional reference vector with reference counted elements 
 *  and implemented as a flat vector.
 */
pub struct Grid<T> {
    width:  u32,
    height: u32,
    data:   Vec<Rc<RefCell<T>>>,
}

/*
 *  Public methods
 */
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

    // Unchecked getter for the given cell
    pub fn get(&self, x:u32, y:u32) -> &Rc<RefCell<T>> {
        &self[(x, y)]
    }

    // Gets the Rc contained in the given grid cell so that you can clone it and stuff
    pub fn get_op(&self, x: u32, y: u32) -> Option<&Rc<RefCell<T>>> {
        self.data.get(self.coordinates_to_index(x, y))
    }

    // Unchecked getter for an immutable reference.  Saves some typing.
    pub fn get_ref(&self, x: u32, y: u32) -> Ref<T> {
        (*self[(x, y)]).borrow()
    }
    
    // Unchecked getter for a mutable reference.  Saves some typing.
    pub fn get_ref_mut(&self, x: u32, y: u32) -> RefMut<T> {
        (*self[(x, y)]).borrow_mut()
    }

    // Convenient getter for an immutable reference if you don't trust your coordinates.
    pub fn get_ref_op(&self, x: u32, y: u32) -> Option<Ref<T>> {
        let value = self.data.get(self.coordinates_to_index(x, y));
        match value {
            Some(val) => Some(val.borrow()),
            None      => None,
        }
    }

    // Convenient getter for a mutable reference if you don't trust your coordinates.
    pub fn get_ref_mut_op(&self, x: u32, y: u32) -> Option<RefMut<T>> {
        let value = self.data.get(self.coordinates_to_index(x, y));
        match value {
            Some(val) => Some(val.borrow_mut()),
            None      => None,
        }
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
            data.push(Rc::new(RefCell::new(value.clone())));
        }
        Grid { width, height, data }
    }
}

impl <T> Index<(u32, u32)> for Grid<T> {
    type Output = Rc<RefCell<T>>;

    fn index(&self, (x, y): (u32, u32)) -> &Rc<RefCell<T>> {
        &self.data[self.coordinates_to_index(x, y)]
    }
}

impl <T> IntoIterator for Grid<T> {
    type Item = Rc<RefCell<T>>;
    type IntoIter = ::std::vec::IntoIter<Rc<RefCell<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
} 

pub struct GridIterator<'a, T: 'a> {
    data: &'a Vec<Rc<RefCell<T>>>,
    current: usize,
}

impl <'a, T> Iterator for GridIterator<'a, T> {
    type Item = Rc<RefCell<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.data.len() {
            None
        } else {
            self.current += 1;
            Some(self.data[self.current - 1].clone())
        }
    }
}






#[cfg(test)]
mod tests {
    use ::lib::grid::Grid;
    use std::rc::Rc;
    use std::cell::RefCell;

    fn get_grid() -> Grid<Box<i32>> {
        let mut data = Vec::new(); 
        for _ in 0..25 {
            data.push(Rc::new(RefCell::new(Box::new(0))));
        }
        Grid::with_data(5, 5, data)
    }
    
    #[test]
    fn getters() {
        let grid = get_grid();

        **grid[(3, 2)].borrow_mut() = 3;
        **grid[(1, 1)].borrow_mut() = 2;

        match grid.get_ref_mut_op(2, 4) {
            Some(mut cell)   => **cell = 7,
            None         => {},
        }

        match grid.get_op(2, 3).clone() {
            Some(cell)   => **cell.borrow_mut() = 5,
            None         => {},
        }

        assert_eq!(**grid[(3, 2)].borrow(), 3);
        assert_eq!(**grid[(1, 1)].borrow(), 2);
        assert_eq!(**grid[(2, 4)].borrow(), 7);
        assert_eq!(**grid[(2, 3)].borrow(), 5);
    }

    #[test]
    #[should_panic]
    fn grid_validity() {
        let mut data = Vec::new(); 
        for _ in 0..24 {
            data.push(Rc::new(RefCell::new(Box::new(0))));
        }
        Grid::with_data(5, 5, data);
    }
}