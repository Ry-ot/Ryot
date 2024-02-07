use crate::bevy_compass::CompassAssets;
use crate::helpers::camera::movement;
use crate::PaletteState;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;
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
        Layer::Cursor,
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

fn spawn_camera<C: CompassAssets>(content: Res<C>, mut commands: Commands) {
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
