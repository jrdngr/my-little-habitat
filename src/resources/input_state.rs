use bevy::{
    input::mouse::MouseButtonInput,
    prelude::*,
    window::CursorMoved,
};

use crate::{
    creatures::{Position},
};

#[derive(Default)]
pub struct InputState {
    pub position: Position,
    pub left: bool,
    pub right: bool,
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

impl InputState {
    pub fn update(&mut self,
        mouse_button_input_events: &Res<Events<MouseButtonInput>>, 
        cursor_moved_events: &Res<Events<CursorMoved>>,
        window: &Res<WindowDescriptor>,
    ) {
        for event in self.mouse_button_event_reader.iter(&mouse_button_input_events) {
            use bevy::input::keyboard::ElementState;
    
            match event.button {
                MouseButton::Left => self.left = event.state == ElementState::Pressed,
                MouseButton::Right => self.right = event.state == ElementState::Pressed,
                _ => (),
            }
        }
    
        for event in self.cursor_moved_event_reader.iter(&cursor_moved_events) {
            let width = window.width as f32;
            let height = window.height as f32;

            let x = event.position.x() / width;
            let y = event.position.y() / height;

            let x = (2.0 * x) - 1.0;
            let y = (2.0 * y) - 1.0;

            self.position = Position { x, y };
        }
    }
}
