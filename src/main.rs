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
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
        const GREEN: Color = [0.0, 0.5, 0.0, 1.0];

        let width = self.grid.width;
        let height = self.grid.height;

        let cell_width: f64 = args.width as f64 / width as f64;

        let square = rectangle::square(0.0, 0.0, cell_width);
  
        self.gl.draw(args.viewport(), |c, gl|{
            clear(BLACK, gl);

            for i in 0..width {
                for j in 0..height {
                    let transform = c.transform.trans(i as f64 * cell_width, j as f64 * cell_width);
                    rectangle(GREEN, square, transform, gl);
                }
            }
        });
    }

    // fn update(&mut self, args: &UpdateArgs) {

    // }
}

fn main() {
    const WINDOW_WIDTH: u32 = 500;
    const WINDOW_HEIGHT: u32 = 500;

    const GRID_WIDTH: u16 = 50;
    const GRID_HEIGHT: u16 = 50;

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
            "My Little Habitat",
            [WINDOW_WIDTH, WINDOW_HEIGHT]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut app = App {
        grid: Grid::new(GRID_WIDTH, GRID_HEIGHT),
        gl: GlGraphics::new(opengl),
    };

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