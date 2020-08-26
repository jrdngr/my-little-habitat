use bevy::{
    prelude::*,
    render::camera::{OrthographicProjection, CameraProjection},
};

use crate::{
    creatures::{Energy, Position, Plant},
    resources::InputState,
};

pub fn spawn(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    state: Res<InputState>,
    mut camera_query: Query<(&OrthographicProjection, &Transform)>,
) {
    if state.left {
        let Position { x, y } = state.position;

        for (op, t) in &mut camera_query.iter() {
            let transform = op.get_projection_matrix() * t.value;
            let position = Vec4::new(x, y, 0.0, 1.0);
            let world_position = transform.inverse() * position;
    
            let (x, y) = (world_position.x(), world_position.y());
            
            commands.spawn(SpriteComponents {
                material: materials.add(Color::rgb(0.0, 0.5, 0.1).into()),
                translation: Translation(Vec3::new(x, y, 0.0)),
                sprite: Sprite {
                    size: Vec2::new(10.0, 10.0),
                },
                ..Default::default()
            })
                .with(Plant)
                .with(Energy(100))
                .with(Position { x, y });
            
            break;
        }
    }
}
