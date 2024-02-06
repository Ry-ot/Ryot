use bevy::{input::Input, math::Vec3, prelude::*, render::camera::Camera};
use ryot::{position::TilePosition, CONTENT_CONFIG};

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
        let normalize_dimension = |dimension: f32, tile_size: u32| {
            (dimension / tile_size as f32).round() * tile_size as f32
        };

        transform.translation += time.delta_seconds() * direction * 5_000.;

        let window = windows.single();

        let scale_balance = if ortho.scale > 1. {
            ortho.scale / 2.
        } else {
            ortho.scale
        };

        // Using default because camera doesn't work properly with smaller grids

        let (bottom_left_tile, top_right_tile) = (TilePosition::MIN, TilePosition::MAX);
        let (min_bounds, max_bounds) = (Vec2::from(bottom_left_tile), Vec2::from(top_right_tile));
        let tile_size = CONTENT_CONFIG.sprite_sheet.tile_size;
        const MARGIN: f32 = 90.;
        transform.translation.x = normalize_dimension(transform.translation.x, tile_size.x);
        transform.translation.x = transform.translation.x.clamp(
            min_bounds.x + window.width() / 2. / scale_balance - MARGIN,
            max_bounds.x - window.width() / 2. / scale_balance + MARGIN,
        );

        transform.translation.y = normalize_dimension(transform.translation.y, tile_size.y);
        transform.translation.y = transform.translation.y.clamp(
            min_bounds.y + window.height() / 2. / scale_balance - MARGIN,
            max_bounds.y - window.height() / 2. / scale_balance + MARGIN,
        );

        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}
