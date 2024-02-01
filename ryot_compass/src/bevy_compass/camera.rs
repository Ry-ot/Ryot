use crate::bevy_compass::MascotAssets;
use crate::helpers::camera::movement;
use crate::PaletteState;
use bevy::asset::{Assets, Handle};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;
use ryot::bevy_ryot::sprites::{load_sprites, LoadSpriteSheetTextureCommand};
use ryot::prelude::*;
use std::marker::PhantomData;

pub struct CameraPlugin<C: ConfigAssets + MascotAssets + SpriteAssets>(PhantomData<C>);

impl<C: ConfigAssets + MascotAssets + SpriteAssets> CameraPlugin<C> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<C: ConfigAssets + MascotAssets + SpriteAssets> Default for CameraPlugin<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: ConfigAssets + MascotAssets + SpriteAssets> Plugin for CameraPlugin<C> {
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
                    update_cursor_palette_sprite::<C>,
                    update_cursor_visibility,
                )
                    .chain()
                    .run_if(in_state(InternalContentState::Ready)),
            );
    }
}

#[derive(Resource, Debug, Default)]
pub struct CursorPos(pub Vec2);

#[derive(Debug, Component, Default)]
pub struct SelectedTile(Option<(usize, Handle<TextureAtlas>)>);

fn spawn_cursor(mut commands: Commands) {
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                anchor: Anchor::BottomRight,
                ..Default::default()
            },
            ..default()
        },
        SelectedTile::default(),
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

fn spawn_camera<C: ConfigAssets + MascotAssets + SpriteAssets>(
    content: Res<C>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tile_grid = CONTENT_CONFIG.grid;

    let mut positions = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();
    let mut idx = 0;

    // Create vertices for the vertical lines (columns)
    let (bottom_left, top_right) = tile_grid.get_bounds_screen();
    let (bottom_left_tile, top_right_tile) = tile_grid.get_bounds_tiles();

    for col in bottom_left_tile.x - 1..=top_right_tile.x {
        let x_offset = (col * tile_grid.tile_size.x as i32) as f32;

        positions.push([x_offset, bottom_left.y, 0.0]);
        positions.push([x_offset, top_right.y, 0.0]);

        // Add colors (white for grid lines)
        colors.extend(vec![Color::WHITE.as_rgba_f32(); 2]);

        // Add indices for the line
        indices.extend_from_slice(&[idx, idx + 1]);
        idx += 2;
    }

    // Create vertices for the horizontal lines (rows)
    for row in bottom_left_tile.y - 1..=top_right_tile.y {
        let y_offset = (row * tile_grid.tile_size.y as i32) as f32;

        positions.push([bottom_left.x, y_offset, 0.0]);
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

    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Spawn a black square on top for the main area
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(255., 255., 255., 0.01),
            custom_size: Some(tile_grid.get_size().as_vec2()),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::ZERO),
        ..Default::default()
    });

    // Spawn the square with the grid
    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_handle.into(),
        transform: Transform::from_translation(Vec2::ZERO.extend(256.)),
        material: materials.add(ColorMaterial::default()),
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: content.mascot().clone(),
        transform: Transform::from_translation(Vec2::ZERO.extend(1.)).with_scale(Vec3::splat(0.5)),
        ..Default::default()
    });
}

fn update_cursor_palette_sprite<C: SpriteAssets>(
    content_assets: Res<C>,
    palette_state: Res<PaletteState>,
    mut cursor_query: Query<(
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
        &mut SelectedTile,
    )>,
    mut build_spr_sheet_texture_cmd: EventWriter<LoadSpriteSheetTextureCommand>,
) {
    let Some(sprite_id) = palette_state.selected_tile else {
        return;
    };

    for (mut sprite, mut atlas_handle, mut selected_tile) in cursor_query.iter_mut() {
        if let Some((index, handle)) = &selected_tile.0 {
            if *index == sprite_id as usize && *handle == *atlas_handle {
                continue;
            }
        }

        let sprites = load_sprites(
            &[sprite_id],
            &content_assets,
            &mut build_spr_sheet_texture_cmd,
        );

        let Some(new_sprite) = sprites.first() else {
            continue;
        };

        *atlas_handle = new_sprite.atlas_texture_handle.clone();
        sprite.index = new_sprite.get_sprite_index();

        selected_tile.0 = Some((
            new_sprite.get_sprite_index(),
            new_sprite.atlas_texture_handle.clone(),
        ));
    }
}

fn update_cursor_visibility(
    cursor_pos: Res<CursorPos>,
    mut egui_ctx: EguiContexts,
    mut windows: Query<&mut Window>,
    mut cursor_query: Query<(&mut Transform, &mut Visibility), With<SelectedTile>>,
) {
    if egui_ctx.ctx_mut().is_pointer_over_area() {
        egui_ctx
            .ctx_mut()
            .set_cursor_icon(egui::CursorIcon::Default);

        windows.single_mut().cursor.visible = true;
        windows.single_mut().cursor.icon = CursorIcon::Default;

        return;
    }

    for (mut transform, mut visibility) in cursor_query.iter_mut() {
        let tile_grid = CONTENT_CONFIG.grid;
        let tile_pos = tile_grid.get_tile_pos_from_display_pos(cursor_pos.0);

        if tile_grid.screen_clamp(cursor_pos.0) != cursor_pos.0 {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }

        let Some(cursor_pos) = tile_grid.get_display_position_from_tile_pos(tile_pos.extend(128.))
        else {
            return;
        };

        transform.translation = cursor_pos;

        if egui_ctx.ctx_mut().is_pointer_over_area() {
            continue;
        }

        match *visibility {
            Visibility::Visible => {
                egui_ctx.ctx_mut().set_cursor_icon(egui::CursorIcon::None);
                windows.single_mut().cursor.icon = CursorIcon::Default;
                windows.single_mut().cursor.visible = false;
            }
            Visibility::Hidden => {
                egui_ctx
                    .ctx_mut()
                    .set_cursor_icon(egui::CursorIcon::NotAllowed);
                windows.single_mut().cursor.icon = CursorIcon::NotAllowed;
                windows.single_mut().cursor.visible = true;
            }
            _ => {}
        }
    }
}
