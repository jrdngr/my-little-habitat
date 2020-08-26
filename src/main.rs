pub mod creatures;
pub mod plugins;
pub mod resources;
pub mod systems;

use bevy::{
    prelude::*,
    render::pass::ClearColor,
};

const WINDOW_SIZE: (u32, u32) = (1920, 1080);
const GRID_SIZE: (usize, usize) = (100, 100);

fn  main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "My Little Habitat".to_string(),
            width: WINDOW_SIZE.0,
            height: WINDOW_SIZE.1,
            vsync: true,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(plugins::window_resize::WindowResizePlugin)
        .add_resource(resources::Grid::new(GRID_SIZE.0, GRID_SIZE.1))
        .add_plugin(GamePlugin)
        .run();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        use systems::spawn;

        app.add_startup_system(systems::setup.system())
            .add_resource(spawn::MouseState::default())
            .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_system(spawn.system());
    }
}
