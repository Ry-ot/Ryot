use crate::bevy_compass::CompassAssets;
use crate::{
    CompassAction, CompassContentAssets, HudLayers, {PaletteState, UiState},
};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{EguiContext, EguiContexts};
use bevy_pancam::*;
use leafwing_input_manager::prelude::*;
use ryot::bevy_ryot::drawing::DrawingBundle;
use ryot::position::{Sector, TilePosition};
use ryot::prelude::drawing::{BrushItem, BrushParams, Brushes};
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
            .init_resource::<ToggleActions<PanCamAction>>()
            .add_plugins(PanCamPlugin)
            .add_systems(
                OnExit(InternalContentState::LoadingContent),
                (spawn_camera, spawn_cursor).chain(),
            )
            .insert_resource(CompassAction::get_default_input_map())
            .add_systems(
                Update,
                (
                    (
                        update_cursor_pos.map(drop),
                        update_cursor_preview,
                        update_cursor_brush_preview,
                        update_cursor_visibility.map(drop),
                        update_camera_visible_sector,
                    )
                        .chain(),
                    update_pan_cam_actions.run_if(resource_changed::<UiState>),
                )
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
    pub brush_index: usize,
    pub tool_mode: ToolMode,
    pub input_type: InputType,
}

impl Default for DrawingState {
    fn default() -> Self {
        Self {
            enabled: true,
            brush_index: 0,
            tool_mode: ToolMode::default(),
            input_type: InputType::default(),
        }
    }
}

#[derive(Eq, PartialEq, Default, Clone, Copy, Reflect)]
pub enum ToolMode {
    #[default]
    Draw,
    Erase,
}

#[derive(Eq, PartialEq, Clone, Copy, Reflect)]
pub enum InputType {
    SingleClick(i32),
    DoubleClick(Option<TilePosition>),
}

impl Default for InputType {
    fn default() -> Self {
        InputType::SingleClick(0)
    }
}

impl<E: BrushItem> From<InputType> for BrushParams<E> {
    fn from(input_type: InputType) -> Self {
        match input_type {
            InputType::SingleClick(size) => BrushParams::Size(size),
            InputType::DoubleClick(Some(pos)) => BrushParams::Position(pos),
            InputType::DoubleClick(None) => BrushParams::Size(0),
        }
    }
}

fn spawn_cursor(mut commands: Commands) {
    commands.spawn((
        Cursor::default(),
        Layer::from(HudLayers::Cursor),
        TilePosition::default(),
        AppearanceDescriptor::default(),
    ));
}

pub static MAP_GRAB_INPUTS: MouseButton = MouseButton::Right;

fn spawn_camera(
    content: Res<CompassContentAssets>,
    mut commands: Commands,
    atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) {
    let layout = content
        .get_atlas_layout(SpriteLayout::OneByOne)
        .expect("Must have atlas layout");

    let atlas_layout = atlas_layouts.get(layout).expect("No atlas layout");
    let zoom_factor = atlas_layout.size.x / 384.;

    #[cfg(not(target_arch = "wasm32"))]
    let zoom_input = SingleAxis::mouse_wheel_y().with_sensitivity(32.);
    #[cfg(target_arch = "wasm32")]
    let zoom_input = SingleAxis::mouse_wheel_y();

    commands.spawn((
        Camera2dBundle::default(),
        Sector::default(),
        PanCamBundle {
            pan_cam: PanCam {
                enabled: true,
                zoom_to_cursor: true,
                min_scale: 0.2 * zoom_factor,
                max_scale: Some(8.75 * zoom_factor),
                ..default()
            },
            inputs: InputManagerBundle::<PanCamAction> {
                action_state: ActionState::default(),
                input_map: InputMap::default()
                    .insert(PanCamAction::Grab, MAP_GRAB_INPUTS)
                    .insert(PanCamAction::Zoom, zoom_input)
                    .build(),
            },
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
    contexts: EguiContexts,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_query: Query<&mut TilePosition, With<Cursor>>,
) -> color_eyre::Result<()> {
    let (camera, camera_transform) = camera_query.get_single()?;

    let Some(cursor_position) = window_query.get_single()?.cursor_position() else {
        return Ok(());
    };

    let mut base_positon = Vec2::ZERO;
    if let Some(vlewport) = &camera.viewport {
        base_positon = vlewport.physical_position.as_vec2() / contexts.ctx().pixels_per_point();
    }
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position - base_positon)
    else {
        return Ok(());
    };

    let mut cursor_pos = cursor_query.get_single_mut()?;
    let new_pos = TilePosition::from(point).with_z(0);

    if *cursor_pos == new_pos {
        return Ok(());
    }

    *cursor_pos = new_pos;

    Ok(())
}

fn update_cursor_visibility(
    palette_state: Res<PaletteState>,
    mut egui_ctx: Query<&mut EguiContext>,
    mut windows: Query<&mut Window>,
    mut cursor_query: Query<(&TilePosition, &mut Visibility, &mut Sprite, &Cursor)>,
    gui_state: Res<UiState>,
) -> color_eyre::Result<()> {
    let mut egui_ctx = egui_ctx.single_mut();
    let (tile_pos, mut visibility, mut sprite, cursor) = cursor_query.get_single_mut()?;

    sprite.color = get_cursor_color(cursor);

    if gui_state.is_being_used || palette_state.selected_tile.is_none() {
        *visibility = Visibility::Hidden;

        windows.single_mut().cursor.visible = true;
        windows.single_mut().cursor.icon = CursorIcon::Default;
        egui_ctx.get().set_cursor_icon(egui::CursorIcon::Default);

        return Ok(());
    }

    if tile_pos.is_valid() {
        *visibility = Visibility::Visible;
        egui_ctx
            .get_mut()
            .set_cursor_icon(egui::CursorIcon::Crosshair);
        windows.single_mut().cursor.icon = CursorIcon::Crosshair;
    } else {
        *visibility = Visibility::Hidden;
        egui_ctx
            .get_mut()
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
            Option<&mut Sprite>,
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
        cursor.drawing_state.input_type.into(),
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

        // If the sprite is already loaded, Sprite exists, so we change its color to
        // differentiate the preview tiles from the rest, making them grayish and transparent.
        if let Some(mut sprite) = sprite {
            sprite.color = get_cursor_color(cursor)
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
            Layer::from(HudLayers::BrushPreview),
        ));
    }
}

fn update_camera_visible_sector(
    mut camera_query: Query<(&mut Sector, &Transform, &OrthographicProjection), With<Camera>>,
) {
    for (mut sector, transform, projection) in camera_query.iter_mut() {
        let new_sector = Sector::from_transform_and_projection(transform, projection);

        if new_sector == *sector {
            continue;
        }

        *sector = new_sector;
    }
}

fn update_pan_cam_actions(mut toggle: ResMut<ToggleActions<PanCamAction>>, ui_state: Res<UiState>) {
    toggle.enabled = !ui_state.is_being_used;
}

pub fn get_cursor_color(cursor: &Cursor) -> Color {
    match cursor.drawing_state.tool_mode {
        ToolMode::Draw => Color::rgba(0.7, 0.7, 0.7, 0.7),
        ToolMode::Erase => Color::rgba(1., 0.0, 0.0, 0.5),
    }
}

/// System responsible for toggling the grid visibility. This system is called when the user presses the
/// [`ToggleGrid`](crate::CompassAction::ToggleGrid) action.
pub fn toggle_grid(mut q_grid: Query<&mut Visibility, With<GridView>>) {
    for mut visibility in q_grid.iter_mut() {
        *visibility = match *visibility {
            Visibility::Inherited => Visibility::Hidden,
            Visibility::Visible => Visibility::Hidden,
            Visibility::Hidden => Visibility::Inherited,
        };
    }
}
