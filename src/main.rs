#![allow(dead_code)]

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod grid;
mod creatures;
mod utils;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use grid::*;
use creatures::*;
use utils::*;

pub struct App {
    grid: Grid<Creature>,
    gl: GlGraphics,
}

impl App {
    fn new(opengl: OpenGL) -> App {
        const GRID_WIDTH: u16 = 50;
        const GRID_HEIGHT: u16 = 50;
        
        const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
        const GREEN: Color = [0.0, 0.5, 0.0, 1.0];

        let empty_creature = Creature {
            name: String::from("empty"),
            color: BLACK,
        };

        let grid: Grid<Creature> = Grid::new(GRID_WIDTH, GRID_HEIGHT, empty_creature);

        App {
            grid: grid,
            gl: GlGraphics::new(opengl),
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: Color = [1.0, 1.0, 1.0, 1.0];

        let cell_width: f64 = args.width as f64 / self.grid.width as f64;
        let cell_height: f64 = args.height as f64/ self.grid.height as f64;
        
        let square = rectangle::square(0.0, 0.0, cell_width);
  
        let cells = self.grid.get_cells();

        self.gl.draw(args.viewport(), |c, gl|{
            clear(WHITE, gl);

            for cell in cells {
                    let x = cell.position.x;
                    let y = cell.position.y;

                    let transform = c.transform.trans(x as f64 * cell_width, y as f64 * cell_height);
                    rectangle(cell.contents.color, square, transform, gl);
            }

        });
    }

    // fn update(&mut self, args: &UpdateArgs) {

    // }
}

fn main() {
    const WINDOW_WIDTH: u32 = 500;
    const WINDOW_HEIGHT: u32 = 500;

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
            "My Little Habitat",
            [WINDOW_WIDTH, WINDOW_HEIGHT]
        ).opengl(opengl)
         .exit_on_esc(true)
         .build()
         .unwrap();
    
    let mut app = App::new(opengl);
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        // if let Some(u) = e.update_args() {
        //     app.update(&u);
        // }
    }

}