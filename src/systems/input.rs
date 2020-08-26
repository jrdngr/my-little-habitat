use bevy::{
    input::mouse::MouseButtonInput,
    prelude::*,
    window::CursorMoved,
};

use crate::{
    resources::InputState,
};

pub fn update_input_state(
    mut input_state: ResMut<InputState>,
    window: Res<WindowDescriptor>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    input_state.update(&mouse_button_input_events, &cursor_moved_events, &window);
}
