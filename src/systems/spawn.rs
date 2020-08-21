use bevy::{
    input::mouse::MouseButtonInput,
    prelude::*,
    window::CursorMoved,
};

#[derive(Default)]
pub struct MouseState {
    pub position: (f32, f32),
    pub left: bool,
    pub right: bool,
}

#[derive(Default)]
pub struct MouseStateReader {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    // mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

/// This system prints out all mouse events as they come in
pub fn spawn(
    mut state_reader: ResMut<MouseStateReader>,
    mut state: ResMut<MouseState>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    // mouse_motion_events: Res<Events<MouseMotion>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    for event in state_reader.mouse_button_event_reader.iter(&mouse_button_input_events) {
        use bevy::input::keyboard::ElementState;

        match event.button {
            MouseButton::Left => state.left = event.state == ElementState::Pressed,
            MouseButton::Right => state.right = event.state == ElementState::Pressed,
            _ => (),
        }
    }

    for event in state_reader.cursor_moved_event_reader.iter(&cursor_moved_events) {
        state.position = (event.position.x(), event.position.y());
    }
}
