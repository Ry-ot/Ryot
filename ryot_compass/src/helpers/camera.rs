use bevy::{input::Input, math::Vec3, prelude::*, render::camera::Camera};

// A simple camera system for moving and zooming the camera.
#[allow(dead_code)]
pub fn movement(
    time: Res<Time>,
    windows: Query<&mut Window>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.just_pressed(KeyCode::Z) {
            ortho.scale += 0.25;
        }

        if keyboard_input.just_pressed(KeyCode::X) {
            ortho.scale -= 0.25;
        }

        ortho.scale = ortho.scale.clamp(0.25, 5.0);

        let z = transform.translation.z;
        let normalize_dimension =
            |dimension: f32, tile_size: f32| (dimension / tile_size).round() * tile_size;

        transform.translation += time.delta_seconds() * direction * 5_000.;

        let window = windows.single();

        let scale_balance = if ortho.scale > 1. {
            ortho.scale / 2.
        } else {
            ortho.scale
        };

        transform.translation.x = normalize_dimension(transform.translation.x, 32.);
        transform.translation.x = transform.translation.x.clamp(
            ortho.scale * (window.width() / 2. - 50. / scale_balance),
            u16::MAX as f32,
        );

        transform.translation.y = normalize_dimension(transform.translation.y, 32.);
        transform.translation.y = transform.translation.y.clamp(
            -(u16::MAX as f32),
            -ortho.scale * (window.height() / 2. - 90. / scale_balance),
        );

        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}
