use ::lib::grid::Grid;

pub trait GridCell {
    fn display(&self);
    fn act(&self, grid: &Grid<&GridCell>);
    fn set_char(&mut self, new_char: char);
    fn get_char(&self) -> char;
}

pub struct Zeero {
    disp: char,   
}

impl Zeero {
    pub fn new() -> Zeero {
        Zeero { disp: '0' }
    }
}

impl GridCell for Zeero {
    fn display(&self) {
        print!("{}", self.disp);
    }

    fn act(&self, grid: &Grid<&GridCell>) {
        // Do nothing;
    }

    fn set_char(&mut self, new_char: char) {
        self.disp = new_char;
    }

    fn get_char(&self) -> char {
        self.disp
    }
}