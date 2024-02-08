use crate::bevy_compass::CompassAssets;
use crate::{CompassContentAssets, DrawingAction, PaletteState};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;
use bevy_pancam::*;
use leafwing_input_manager::prelude::*;
use ryot::bevy_ryot::drawing::Layer;
use ryot::position::TilePosition;
use ryot::prelude::drawing::DetailLevel;
use ryot::prelude::*;
use std::fmt;
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
                    update_camera_edges,
                )
                    .chain()
                    .run_if(in_state(InternalContentState::Ready)),
            );
    }
}

#[derive(Eq, PartialEq, Component, Reflect, Default, Clone, Copy, Debug)]
pub struct Cursor {
    pub drawing_state: DrawingState,
}

#[derive(Eq, PartialEq, Reflect, Clone, Copy, Debug)]
pub struct DrawingState {
    pub enabled: bool,
}

impl Default for DrawingState {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Eq, PartialEq, Component, Reflect, Default, Clone, Copy, Debug)]
pub struct Edges {
    pub min: TilePosition,
    pub max: TilePosition,
}

impl fmt::Display for Edges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Edges({}, {})", self.min, self.max)
    }
}

impl Edges {
    pub fn from_transform_and_projection(
        transform: &Transform,
        projection: &OrthographicProjection,
    ) -> Self {
        let visible_width = projection.area.max.x - projection.area.min.x;
        let visible_height = projection.area.max.y - projection.area.min.y;

        // Adjust by the camera scale if necessary
        let visible_width = visible_width * transform.scale.x;
        let visible_height = visible_height * transform.scale.y;

        // Calculate boundaries based on the camera's position
        let camera_position = transform.translation;
        let left_bound = camera_position.x - visible_width / 2.0;
        let right_bound = camera_position.x + visible_width / 2.0;
        let bottom_bound = camera_position.y - visible_height / 2.0;
        let top_bound = camera_position.y + visible_height / 2.0;

        Self {
            min: TilePosition::from(Vec2::new(left_bound, bottom_bound)),
            max: TilePosition::from(Vec2::new(right_bound, top_bound)),
        }
    }

    pub fn size(&self) -> IVec2 {
        IVec2::new(self.max.x - self.min.x, self.max.y - self.min.y)
    }

    pub fn area(&self) -> u32 {
        (self.size().x * self.size().y).unsigned_abs()
    }
}

fn spawn_cursor(mut commands: Commands) {
    commands.spawn((
        Cursor::default(),
        Layer::Cursor,
        TilePosition::default(),
        AppearanceDescriptor::default(),
        InputManagerBundle::<DrawingAction> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: DrawingAction::get_default_input_map(),
        },
    ));
}

fn spawn_camera(content: Res<CompassContentAssets>, mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        Edges::default(),
        DetailLevel::default(),
        PanCam {
            grab_buttons: vec![KeyMouseCombo::KeyMouse(
                vec![KeyCode::AltLeft],
                MouseButton::Left,
            )],
            enabled: true,
            zoom_to_cursor: true,
            min_scale: 0.2,
            max_scale: Some(8.75),
            ..default()
        },
    ));

    commands.spawn(SpriteBundle {
        texture: content.mascot().clone(),
        transform: Transform::from_translation(Vec2::ZERO.extend(1.)).with_scale(Vec3::splat(0.5)),
        ..Default::default()
    });
}

fn update_cursor_palette_sprite(
    palette_state: Res<PaletteState>,
    mut cursor_query: Query<(&mut Cursor, &mut AppearanceDescriptor)>,
) {
    for (mut cursor, mut desired_appearance) in cursor_query.iter_mut() {
        let Some(selected) = &palette_state.selected_tile else {
            cursor.drawing_state.enabled = false;
            return;
        };

        cursor.drawing_state.enabled = true;

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

fn update_camera_edges(
    mut camera_query: Query<
        (
            &mut Edges,
            &mut DetailLevel,
            &Transform,
            &OrthographicProjection,
        ),
        With<Camera>,
    >,
) {
    for (mut edges, mut detail_level, transform, projection) in camera_query.iter_mut() {
        let new_edges = Edges::from_transform_and_projection(transform, projection);

        if new_edges == *edges {
            continue;
        }

        *edges = new_edges;
        let new_detail_level = DetailLevel::from_area(edges.area());

        if *detail_level != new_detail_level {
            *detail_level = new_detail_level;
        }
    }
}
