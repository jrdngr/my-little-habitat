#![allow(dead_code)]

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod lib;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use lib::grid::*;
use lib::utils::*;
use lib::creatures::*;

pub struct App {
    grid: Grid,
    gl: GlGraphics,
}

impl App {
    fn new(opengl: OpenGL, grid_width: u32, grid_height: u32) -> Self {

        let grid = Grid::new(grid_width, grid_height);

        App {
            grid: grid,
            gl: GlGraphics::new(opengl),
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: Color = [1.0, 1.0, 1.0, 1.0];

        let creature_width: f64 = args.width as f64 / self.grid.width as f64;
        let creature_height: f64 = args.height as f64/ self.grid.height as f64;
        
        let square = rectangle::square(0.0, 0.0, creature_width);
        let creatures = self.grid.get_creature_list();

        self.gl.draw(args.viewport(), |c, gl|{

            for creature in creatures {
                    let (x, y) = creature.get_position();

                    let transform = c.transform.trans(x as f64 * creature_width, y as f64 * creature_height);
                    rectangle(creature.get_color(), square, transform, gl);
            }

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        //args.dt;
    }

    fn set_creature(&mut self, position: Position, creature_type: CreatureType) {
        self.grid.set_creature(position, creature_type);
    }
}

fn main() {
    const WINDOW_WIDTH: u32 = 600;
    const WINDOW_HEIGHT: u32 = 600;

    const GRID_WIDTH: u32 = WINDOW_WIDTH / 2;
    const GRID_HEIGHT: u32 = WINDOW_HEIGHT / 2;

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

    let width_scale = (WINDOW_WIDTH as u32 / GRID_WIDTH) as f32;
    let height_scale = (WINDOW_HEIGHT as u32 / GRID_HEIGHT) as f32;

    let mut position: Position = (0, 0);

    while let Some(e) = events.next(&mut window) {
        
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        
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
            let grid_x = (x  / width_scale) as u32;
            let grid_y = (y  /  height_scale) as u32;
            position = (grid_x, grid_y);
        };
    }
}