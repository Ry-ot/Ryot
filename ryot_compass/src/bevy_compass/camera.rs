use crate::bevy_compass::CompassAssets;
use crate::{CompassContentAssets, PaletteState};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;
use bevy_pancam::*;
use ryot::bevy_ryot::drawing::Layer;
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
        app.insert_resource(Msaa::Off)
            .add_plugins(PanCamPlugin)
            .add_systems(
                OnExit(InternalContentState::LoadingContent),
                (spawn_camera, spawn_cursor).chain(),
            )
            .add_systems(
                Update,
                (
                    update_cursor_pos.map(drop),
                    update_cursor_palette_sprite,
                    update_cursor_visibility.map(drop),
                )
                    .chain()
                    .run_if(in_state(InternalContentState::Ready)),
            );
    }
}

#[derive(Component)]
pub struct Cursor;

fn spawn_cursor(mut commands: Commands) {
    commands.spawn((
        Cursor,
        AppearanceDescriptor::default(),
        Layer::Cursor,
        TilePosition::default(),
    ));
}

fn spawn_camera(content: Res<CompassContentAssets>, mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(PanCam {
        grab_buttons: vec![KeyMouseCombo::KeyMouse(
            vec![KeyCode::AltLeft],
            MouseButton::Left,
        )],
        enabled: true,
        zoom_to_cursor: true,
        min_scale: 0.2,
        max_scale: Some(10.),
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: content.mascot().clone(),
        transform: Transform::from_translation(Vec2::ZERO.extend(1.)).with_scale(Vec3::splat(0.5)),
        ..Default::default()
    });
}

fn update_cursor_palette_sprite(
    palette_state: Res<PaletteState>,
    mut cursor_query: Query<&mut AppearanceDescriptor, With<Cursor>>,
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

fn update_cursor_pos(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_query: Query<(&mut TilePosition, Option<&mut Transform>, &Layer), With<Cursor>>,
) -> color_eyre::Result<()> {
    let (camera, camera_transform) = camera_query.get_single()?;

    let Some(cursor_position) = window_query.get_single()?.cursor_position() else {
        return Ok(());
    };

    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return Ok(());
    };

    let (mut cursor_pos, transform, layer) = cursor_query.get_single_mut()?;
    let new_pos = TilePosition::from(point).with_z(layer.z());

    if *cursor_pos == new_pos {
        return Ok(());
    }

    *cursor_pos = new_pos;

    if let Some(mut transform) = transform {
        transform.translation = new_pos.into();
    }

    Ok(())
}

fn update_cursor_visibility(
    palette_state: Res<PaletteState>,
    mut egui_ctx: EguiContexts,
    mut windows: Query<&mut Window>,
    mut cursor_query: Query<(&TilePosition, &mut Visibility), With<Cursor>>,
) -> color_eyre::Result<()> {
    let (tile_pos, mut visibility) = cursor_query.get_single_mut()?;

    if egui_ctx.ctx_mut().is_pointer_over_area() || palette_state.selected_tile.is_none() {
        *visibility = Visibility::Hidden;

        windows.single_mut().cursor.visible = true;
        windows.single_mut().cursor.icon = CursorIcon::Default;
        egui_ctx.ctx().set_cursor_icon(egui::CursorIcon::Default);

        return Ok(());
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

    Ok(())
}
