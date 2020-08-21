mod creatures;
mod systems;

use bevy::{
    prelude::*,
    render::pass::ClearColor,
};

const WINDOW_SIZE: (u32, u32) = (800, 600);

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: WINDOW_SIZE.0,
            height: WINDOW_SIZE.1,
            vsync: true,
            ..Default::default()
        })
        .add_default_plugins()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(systems::setup.system())
        .add_system(systems::spawn.system())
        .run();
}

pub struct Energy(u32);
