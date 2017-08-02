#![allow(dead_code)]

extern crate rand;

mod lib;

use std::rc::Rc;
use std::cell::RefCell;

use lib::grid::Grid;
use lib::gridcell::*;

fn main() {
    let mut data: Vec<Rc<RefCell<Box<GridCell>>>> = Vec::new(); 
    for _ in 0..25 {
        data.push(Rc::new(RefCell::new(Box::new(Zeero::new()))));
    }
    let grid = Grid::with_data(5, 5, data);

    for (i, cell) in grid.iter().enumerate() {
        if i % 5 == 0 { print!("\n") };
        cell.borrow().display();
    }

    {
        let what = grid[(1, 1)].clone();
        what.borrow_mut().set_char('1');
    }

    println!();

    for (i, cell) in grid.iter().enumerate() {
        if i % 5 == 0 { print!("\n") };
        cell.borrow().display();
    }
}