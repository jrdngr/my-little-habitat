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
use utils::*;
use creatures::*;

pub struct App {
    grid: Grid,
    gl: GlGraphics,
}

impl App {
    fn new(opengl: OpenGL, grid_width: u16, grid_height: u16) -> Self {

        let grid = Grid::new(grid_width, grid_height);

        App {
            grid: grid,
            gl: GlGraphics::new(opengl),
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: Color = [1.0, 1.0, 1.0, 1.0];

        let creatures_width: f64 = args.width as f64 / self.grid.width as f64;
        let creatures_height: f64 = args.height as f64/ self.grid.height as f64;
        
        let square = rectangle::square(0.0, 0.0, creatures_width);
  
        let creatures = self.grid.get_creatures();

        self.gl.draw(args.viewport(), |c, gl|{

            for creature in creatures {
                    let (x, y) = creature.get_position();

                    let transform = c.transform.trans(x as f64 * creatures_width, y as f64 * creatures_height);
                    rectangle(creature.get_color(), square, transform, gl);
            }

        });
    }

    fn set_creature(&mut self, position: Position, creature_type: CreatureType) {
        self.grid.set_creature(position, creature_type);
    }
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
    
    let mut app = App::new(opengl, GRID_WIDTH, GRID_HEIGHT);
    let mut events = Events::new(EventSettings::new());

    let mut position: Position = (0, 0);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                app.set_creature(position, CreatureType::Plant);
            }
        }

        if let Some(pos) = e.mouse_cursor_args() {
            let (x, y) = (pos[0] as f32, pos[1] as f32);
            let grid_x = (x as u16 / (WINDOW_WIDTH as u16 /GRID_WIDTH)) as u16;
            let grid_y = (y as u16 / (WINDOW_HEIGHT as u16 / GRID_HEIGHT)) as u16;
            position = (grid_x, grid_y);
        };
    }
}