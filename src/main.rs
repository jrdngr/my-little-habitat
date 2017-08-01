#![allow(dead_code)]

extern crate rand;

mod lib;

use lib::grid::Grid;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut data = Vec::new(); 
    for _ in 0..25 {
        data.push(Rc::new(RefCell::new(Box::new(0))));
    }
    let grid = Grid::with_data(5, 5, data);


    **grid[(3, 2)].borrow_mut() = 3;
    **grid[(1, 1)].borrow_mut() = 2;

    match grid.get_mut(2, 4) {
        Some(mut cell)   => **cell = 7,
        None         => {},
    }

    match grid.get_rc(2, 3).clone() {
        Some(cell)   => **cell.borrow_mut() = 5,
        None         => {},
    }

    for x in 0..grid.height() {
        for y in 0..grid.width() {
            print!("{:?}", grid[(x, y)].borrow());
        }
        print!("\n");
    }
}