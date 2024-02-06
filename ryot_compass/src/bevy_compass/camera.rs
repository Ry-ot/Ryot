use crate::bevy_compass::CompassAssets;
use crate::helpers::camera::movement;
use crate::PaletteState;
use bevy::asset::{Assets, Handle};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;
use ryot::position::TilePosition;
use ryot::prelude::*;
use std::marker::PhantomData;

pub struct CameraPlugin<C: CompassAssets>(PhantomData<C>);

impl<C: CompassAssets> CameraPlugin<C> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<C: CompassAssets> Default for CameraPlugin<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: CompassAssets> Plugin for CameraPlugin<C> {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPos>()
            .add_systems(
                OnExit(InternalContentState::PreparingSprites),
                (spawn_camera::<C>, spawn_cursor).chain(),
            )
            .add_systems(
                Update,
                (
                    movement,
                    update_cursor_pos.map(drop),
                    update_cursor_palette_sprite,
                    update_cursor_visibility,
                )
                    .chain()
                    .run_if(in_state(InternalContentState::Ready)),
            );
    }
}

#[derive(Resource, Debug, Default)]
pub struct CursorPos(pub Vec2);

#[derive(Component)]
pub struct CursorPointer;

fn spawn_cursor(mut commands: Commands) {
    commands.spawn((
        CursorPointer,
        AppearanceDescriptor::default(),
        TilePosition::default(),
    ));
}

fn update_cursor_pos(
    mut cursor_pos: ResMut<CursorPos>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) -> color_eyre::Result<()> {
    let (camera, camera_transform) = camera_query.get_single()?;
    let Some(cursor_position) = window_query.get_single()?.cursor_position() else {
        return Ok(());
    };
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return Ok(());
    };
    *cursor_pos = CursorPos(point);
    Ok(())
}

fn spawn_camera<C: CompassAssets>(
    content: Res<C>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut positions = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();
    let mut idx = 0;

    // Create vertices for the vertical lines (columns)
    let (bottom_left_tile, top_right_tile) = (TilePosition::MIN, TilePosition::MAX);
    let (bottom_left, top_right) = (Vec2::from(bottom_left_tile), Vec2::from(top_right_tile));
    let tile_size = CONTENT_CONFIG.sprite_sheet.tile_size.as_vec2();

    for col in bottom_left_tile.x - 1..=top_right_tile.x {
        let x_offset = (col * tile_size.x as i32) as f32;

        positions.push([x_offset, bottom_left.y, 0.0]);
        positions.push([x_offset, top_right.y + tile_size.y, 0.0]);

        // Add colors (white for grid lines)
        colors.extend(vec![Color::WHITE.as_rgba_f32(); 2]);

        // Add indices for the line
        indices.extend_from_slice(&[idx, idx + 1]);
        idx += 2;
    }

    // Create vertices for the horizontal lines (rows)
    for row in bottom_left_tile.y - 1..=top_right_tile.y {
        let y_offset = (row * tile_size.y as i32) as f32;

        positions.push([bottom_left.x - tile_size.x, y_offset, 0.0]);
        positions.push([top_right.x, y_offset, 0.0]);

        // Add colors (white for grid lines)
        colors.extend(vec![Color::WHITE.as_rgba_f32(); 2]);

        // Add indices for the line
        indices.extend_from_slice(&[idx, idx + 1]);
        idx += 2;
    }

    // Create the mesh
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.set_indices(Some(Indices::U32(indices)));

    let mesh_handle: Handle<Mesh> = meshes.add(mesh);

    // Spawn the square with the grid
    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_handle.into(),
        transform: Transform::from_translation(Vec2::ZERO.extend(998.)),
        material: materials.add(ColorMaterial::default()),
        ..default()
    });

    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: content.mascot().clone(),
        transform: Transform::from_translation(Vec2::ZERO.extend(1.)).with_scale(Vec3::splat(0.5)),
        ..Default::default()
    });
}

fn update_cursor_palette_sprite(
    palette_state: Res<PaletteState>,
    mut cursor_query: Query<&mut AppearanceDescriptor, With<CursorPointer>>,
) {
    let Some(selected) = &palette_state.selected_tile else {
        return;
    };

    for mut desired_appearance in cursor_query.iter_mut() {
        if *desired_appearance == *selected {
            return;
        }
        *desired_appearance = *selected;
    }
}

fn update_cursor_visibility(
    cursor_pos: Res<CursorPos>,
    palette_state: Res<PaletteState>,
    mut egui_ctx: EguiContexts,
    mut windows: Query<&mut Window>,
    mut cursor_query: Query<
        (&mut TilePosition, &mut Transform, &mut Visibility),
        With<CursorPointer>,
    >,
) {
    for (mut tile_pos, mut transform, mut visibility) in cursor_query.iter_mut() {
        if egui_ctx.ctx_mut().is_pointer_over_area() || palette_state.selected_tile.is_none() {
            *visibility = Visibility::Hidden;

            windows.single_mut().cursor.visible = true;
            windows.single_mut().cursor.icon = CursorIcon::Default;
            egui_ctx.ctx().set_cursor_icon(egui::CursorIcon::Default);

            continue;
        }

        let new_tile_pos = TilePosition::from(cursor_pos.0);

        if *tile_pos != new_tile_pos {
            *tile_pos = TilePosition::from(cursor_pos.0);
            transform.translation = Vec2::from(*tile_pos).extend(999.);
        }

        if tile_pos.is_valid() {
            *visibility = Visibility::Visible;
            egui_ctx
                .ctx_mut()
                .set_cursor_icon(egui::CursorIcon::Crosshair);
            windows.single_mut().cursor.icon = CursorIcon::Crosshair;
        } else {
            *visibility = Visibility::Hidden;
            egui_ctx
                .ctx_mut()
                .set_cursor_icon(egui::CursorIcon::NotAllowed);
            windows.single_mut().cursor.icon = CursorIcon::NotAllowed;
            windows.single_mut().cursor.visible = true;
        }
    }
}
