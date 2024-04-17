use bevy::hierarchy::Children;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_stroked_text::StrokedText;
use color_eyre::eyre::eyre;
use glam::Vec2;
use ryot_grid::prelude::*;

pub fn update_cursor_pos<C: Component>(
    In(camera_info): In<Option<(Entity, Vec2)>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_query: Query<&mut TilePosition, With<C>>,
) -> color_eyre::Result<()> {
    let (entity, base_position) = camera_info.ok_or(eyre!(""))?;
    let (camera, camera_transform) = camera_query.get(entity)?;

    let mut cursor_pos = cursor_query.get_single_mut()?;

    let cursor_position = window_query
        .get_single()?
        .cursor_position()
        .ok_or(eyre!(""))?;

    let point = camera
        .viewport_to_world_2d(camera_transform, cursor_position - base_position)
        .ok_or(eyre!(""))?;

    let new_pos = TilePosition::from(point).with_z(0);

    if *cursor_pos != new_pos {
        *cursor_pos = new_pos;
    }

    Ok(())
}

pub fn move_to_cursor<C: Component>(
    q_cursor: Query<&mut TilePosition, With<C>>,
    mut q_camera: Query<&mut Transform, With<Camera>>,
) {
    let tile_pos = q_cursor.single();

    for mut transform in q_camera.iter_mut() {
        let screen_pos: Vec2 = tile_pos.into();
        transform.translation = screen_pos.extend(transform.translation.z);
    }
}

pub fn draw_cursor_system<C: Component>(
    mut cursor_query: Query<(Option<&mut Transform>, &TilePosition, &Children), With<C>>,
    mut child_text_query: Query<&mut StrokedText>,
) -> color_eyre::Result<()> {
    let (cursor_transform, tile_pos, children) = cursor_query.get_single_mut()?;

    if let Some(mut cursor_transform) = cursor_transform {
        cursor_transform.translation = Vec2::from(tile_pos).extend(cursor_transform.translation.z);
    }

    for &child in children.iter() {
        if let Ok(mut cursor_text) = child_text_query.get_mut(child) {
            cursor_text.text = format!("{tile_pos}");
        }
    }

    Ok(())
}

pub fn cursor_sliding_camera<C: Component>(
    sector_query: Query<&Sector, With<Camera>>,
    cursor_query: Query<&TilePosition, With<C>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) -> color_eyre::Result<()> {
    let main_camera_sector = sector_query.get_single()?;
    let cursor_pos = cursor_query.get_single()?;

    for mut camera_transform in camera_query.iter_mut() {
        let speed = 3.;
        let margin = 2;

        let get_move = |position: i32, min_edge: i32, max_edge: i32| -> f32 {
            if position - min_edge <= margin {
                -speed
            } else if max_edge - position <= margin {
                speed
            } else {
                0.
            }
        };

        camera_transform.translation.x += get_move(
            cursor_pos.x,
            main_camera_sector.min.0.x,
            main_camera_sector.max.0.x,
        );

        camera_transform.translation.y += get_move(
            cursor_pos.y,
            main_camera_sector.min.0.y,
            main_camera_sector.max.0.y,
        );
    }

    Ok(())
}
