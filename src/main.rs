#![allow(dead_code)]

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

mod lib;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use lib::grid::*;
use lib::utils::*;
use lib::creature::*;

/*
 * Constants
 */
const CANVAS_WIDTH: u32 = 600;
const CANVAS_HEIGHT: u32 = CANVAS_WIDTH;
const GUI_WIDTH: u32 = 300;

const GRID_WIDTH: u32 = 200;
const GRID_HEIGHT: u32 = GRID_WIDTH;
/*
 *
 */

pub struct Environment {
    gl: GlGraphics,
}

impl Environment {
    fn new(opengl: OpenGL) -> Self {
        Environment {
            gl: GlGraphics::new(opengl),
        }
    }

    fn render(&mut self, args: &RenderArgs, grid: &mut Grid) {
        use graphics::*;

        let creature_width: f64 = (args.width as f64 - GUI_WIDTH as f64) / grid.width as f64;
        let creature_height: f64 = args.height as f64/ grid.height as f64;
        
        let square = rectangle::square(0.0, 0.0, creature_width);
        self.gl.draw(args.viewport(), |c, gl|{
            for (i, color) in grid.get_color_grid().into_iter().enumerate() {
                let (x, y) = grid.index_to_position(i);
                let transform = c.transform.trans(x as f64 * creature_width, y as f64 * creature_height);
                rectangle(color, square, transform, gl);
            }
        });
    }

    fn update(&mut self, grid: &mut Grid) {
        grid.step();
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);

    let mut window: Window = WindowSettings::new(
            "My Little Habitat",
            [CANVAS_WIDTH + GUI_WIDTH, CANVAS_HEIGHT]
        ).opengl(opengl)
         .exit_on_esc(true)
         .build()
         .unwrap();
    
    let mut env = Environment::new(opengl);
    let mut events = Events::new(EventSettings::new());

    events.set_max_fps(60);
    events.set_ups(60);

    let width_scale = (CANVAS_WIDTH as u32 / GRID_WIDTH) as f64;
    let height_scale = (CANVAS_HEIGHT as u32 / GRID_HEIGHT) as f64;

    let mut position: Position = (0, 0);
    let mut mouse_down: bool = false;

    let mut current_selection: CreatureType = CreatureType::Plant;

    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Render(render_args) => {
                env.render(&render_args, &mut grid);
            },
            Input::Update(_) => {
                env.update(&mut grid);
            },
            Input::Press(button) => {
                if button == Button::Mouse(MouseButton::Left) {
                    mouse_down = true;
                }
                match button {
                    Button::Mouse(MouseButton::Left) => mouse_down = true,
                    Button::Keyboard(Key::D1) => current_selection = CreatureType::Empty,
                    Button::Keyboard(Key::D2) => current_selection = CreatureType::Wall,
                    Button::Keyboard(Key::D3) => current_selection = CreatureType::Plant,
                    Button::Keyboard(Key::D4) => current_selection = CreatureType::Snake,
                    Button::Keyboard(Key::D5) => current_selection = CreatureType::Cow,
                    Button::Keyboard(Key::D6) => current_selection = CreatureType::Virus,
                    _ => {}
                }
            },
            Input::Release(button) => {
                if button == Button::Mouse(MouseButton::Left) {
                    mouse_down = false;
                }
            },
            Input::Move(motion) => {
                if let Motion::MouseCursor(x, y) = motion {
                    if x < CANVAS_WIDTH as f64 {
                        let grid_x = (x  / width_scale) as u32;
                        let grid_y = (y  /  height_scale) as u32;
                        position = (grid_x, grid_y);
                    } 
                }
            },
            _ => {}
        }
        if mouse_down {
            grid.set_cell(position, get(current_selection));
        }
    }
}