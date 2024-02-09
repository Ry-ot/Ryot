use crate::bevy_compass::CompassAssets;
use crate::{CompassContentAssets, DrawingAction, PaletteState};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;
use bevy_pancam::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;
use ryot::bevy_ryot::drawing::DrawingBundle;
use ryot::position::{Layer, TilePosition};
use ryot::prelude::drawing::{Brushes, DetailLevel};
use ryot::prelude::*;
use std::fmt;
use std::marker::PhantomData;

pub struct CameraPlugin<C: CompassAssets>(PhantomData<C>);

pub const CURSOR_COLOR: Color = Color::rgba(0.7, 0.7, 0.7, 0.7);

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
                    update_cursor_preview,
                    update_cursor_brush_preview,
                    update_cursor_visibility.map(drop),
                    update_camera_edges,
                )
                    .chain()
                    .run_if(in_state(InternalContentState::Ready)),
            );
    }
}

#[derive(Eq, PartialEq, Component, Default, Clone, Copy, Reflect)]
pub struct Cursor {
    pub drawing_state: DrawingState,
}

#[derive(Eq, PartialEq, Clone, Copy, Reflect)]
pub struct DrawingState {
    pub enabled: bool,
    pub brush_size: i32,
    pub brush_index: usize,
}

impl Default for DrawingState {
    fn default() -> Self {
        Self {
            enabled: true,
            brush_size: 3,
            brush_index: 0,
        }
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
        Layer::Max,
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

pub static MAP_GRAB_INPUTS: [InputKind; 2] = [
    InputKind::Modifier(Modifier::Alt),
    InputKind::Mouse(MouseButton::Left),
];

fn spawn_camera(content: Res<CompassContentAssets>, mut commands: Commands) {
    let mut input_map = InputMap::default();
    input_map.insert_chord(MAP_GRAB_INPUTS, bevy_pancam::Action::Grab);
    input_map.insert(SingleAxis::mouse_wheel_y(), bevy_pancam::Action::Zoom);

    commands.spawn((
        Camera2dBundle::default(),
        Edges::default(),
        DetailLevel::default(),
        PanCam {
            enabled: true,
            zoom_to_cursor: true,
            min_scale: 0.2,
            max_scale: Some(8.75),
            ..default()
        },
        InputManagerBundle::<bevy_pancam::Action> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map,
        },
    ));

    commands.spawn(SpriteBundle {
        texture: content.mascot().clone(),
        transform: Transform::from_translation(Vec2::ZERO.extend(-100.))
            .with_scale(Vec3::splat(0.5)),
        ..Default::default()
    });
}

fn update_cursor_preview(
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
    mut cursor_query: Query<(&mut TilePosition, &Layer), With<Cursor>>,
) -> color_eyre::Result<()> {
    let (camera, camera_transform) = camera_query.get_single()?;

    let Some(cursor_position) = window_query.get_single()?.cursor_position() else {
        return Ok(());
    };

    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return Ok(());
    };

    let (mut cursor_pos, layer) = cursor_query.get_single_mut()?;
    let new_pos = TilePosition::from(point).with_z(layer.z());

    if *cursor_pos == new_pos {
        return Ok(());
    }

    *cursor_pos = new_pos;

    Ok(())
}

fn update_cursor_visibility(
    palette_state: Res<PaletteState>,
    mut egui_ctx: EguiContexts,
    mut windows: Query<&mut Window>,
    mut cursor_query: Query<
        (&TilePosition, &mut Visibility, &mut TextureAtlasSprite),
        With<Cursor>,
    >,
) -> color_eyre::Result<()> {
    let (tile_pos, mut visibility, mut sprite) = cursor_query.get_single_mut()?;

    sprite.color = CURSOR_COLOR;

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

#[derive(Component)]
struct BrushPreviewTile;

type CursorHasChangedFilter = (
    Without<BrushPreviewTile>,
    Or<(
        Changed<Cursor>,
        Changed<Visibility>,
        Changed<TilePosition>,
        Changed<AppearanceDescriptor>,
    )>,
);

type CursorBrushPreviewFilter = (With<BrushPreviewTile>, Without<Cursor>);

/// This function listens for changes in the cursor's main tile and updates its brush preview
/// accordingly. The brush preview is a set of tiles that show the area that the brush will cover,
/// depending on the current brush size and shape.
///
/// When the cursor grows we draw new preview tiles, and when it shrinks we hide the extra ones.
/// We never remove the tiles once added, as the cost of keeping them hidden is good enough and
/// deleting/spawning components can be expensive.
///
/// We always update the preview tiles position, appearance, visibility and sprite color, the
/// first three according to the main cursor definition and the color is always CURSOR_COLOR.
fn update_cursor_brush_preview(
    brushes: Res<Brushes<DrawingBundle>>,
    cursor: Query<
        (&Cursor, &TilePosition, &AppearanceDescriptor, &Visibility),
        (Without<BrushPreviewTile>, CursorHasChangedFilter),
    >,
    mut commands: Commands,
    mut brush_preview_tiles: Query<
        (
            &mut TilePosition,
            &mut AppearanceDescriptor,
            &mut Visibility,
            Option<&mut TextureAtlasSprite>,
        ),
        CursorBrushPreviewFilter,
    >,
) {
    let Ok((cursor, cursor_pos, cursor_appearance, cursor_visibility)) = cursor.get_single() else {
        return;
    };

    // Here we get the positions of the tiles that will be part of the brush preview
    let mut positions: Vec<TilePosition> = brushes(
        cursor.drawing_state.brush_index,
        cursor.drawing_state.brush_size,
        DrawingBundle::from_tile_position(*cursor_pos),
    )
    .into_iter()
    .map(|bundle| bundle.tile_pos)
    .collect();

    // We first iterate over all the existing brush preview tiles and update them according to
    // the cursor state and the brush preview positions. We do that to avoid spawning new tiles
    // unnecessarily.
    for (mut tile_pos, mut appearance, mut visibility, sprite) in brush_preview_tiles.iter_mut() {
        // If the drawing_state is disable we are probably interacting with the UI, so we hide the preview.
        if !cursor.drawing_state.enabled {
            *visibility = Visibility::Hidden;
            continue;
        }

        // If the sprite is already loaded, TextureAtlasSprite exists, so we change its color to
        // differentiate the preview tiles from the rest, making them grayish and transparent.
        if let Some(mut sprite) = sprite {
            sprite.color = CURSOR_COLOR
        }

        // If we finished iterating over the positions, we hide the remaining preview tiles.
        let Some(new_pos) = positions.pop() else {
            *visibility = Visibility::Hidden;
            continue;
        };

        // If the new position is not valid, we hide the tile and continue to the next one.
        // This is only relevant in the borders of the map, where the preview tiles would be
        // partially or completely outside the map. We also don't want to add a preview tile
        // for the cursor center, that is already there.
        if !new_pos.is_valid() || new_pos == *cursor_pos {
            *visibility = Visibility::Hidden;
            continue;
        }

        // Here we update the existing preview tile with the new position, appearance and visibility.
        *visibility = *cursor_visibility;
        *tile_pos = new_pos;
        *appearance = *cursor_appearance;
    }

    // We never proceed if the drawing state is disabled, since we don't want to spawn new tiles
    // and we never preview when interacting with the UI.
    if !cursor.drawing_state.enabled {
        return;
    }

    // If there are no more positions to preview, we don't need to spawn new tiles.
    if positions.is_empty() {
        return;
    }

    // If after covering all the existing preview tiles, we still have positions to preview
    // we spawn new tiles for the remaining positions.
    for new_pos in positions {
        if !new_pos.is_valid() || new_pos == *cursor_pos {
            continue;
        }

        commands.spawn((
            BrushPreviewTile,
            new_pos,
            *cursor_appearance,
            *cursor_visibility,
            Layer::Cursor,
        ));
    }
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
