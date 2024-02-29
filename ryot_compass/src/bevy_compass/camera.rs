use crate::bevy_compass::CompassAssets;
use crate::{CompassAction, CompassContentAssets, HudLayers, UiState};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{EguiContext, EguiContexts};
use bevy_pancam::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
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
                        move_to_cursor.run_if(action_just_pressed(CompassAction::Focus)),
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
            )
            .add_systems(
                PostUpdate,
                (update_cursor_sprite).run_if(in_state(InternalContentState::Ready)),
            );
    }
}

#[derive(Eq, PartialEq, Component, Default, Clone, Copy, Reflect)]
pub struct Cursor {
    pub drawing_state: DrawingState,
}

#[derive(Eq, PartialEq, Clone, Copy, Default, Reflect)]
pub struct DrawingState {
    pub brush_index: usize,
    pub tool_mode: ToolMode,
    pub input_type: InputType,
}

#[derive(Eq, PartialEq, Default, Clone, Copy, Debug, Reflect)]
pub enum ToolMode {
    #[default]
    None,
    Draw(AppearanceDescriptor),
    Erase,
}

impl ToolMode {
    pub fn is_none(&self) -> bool {
        matches!(self, ToolMode::None)
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Reflect)]
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

fn update_cursor_preview(mut cursor_query: Query<(&Cursor, &mut AppearanceDescriptor)>) {
    for (cursor, mut desired_appearance) in cursor_query.iter_mut() {
        let appearance = match cursor.drawing_state.tool_mode {
            ToolMode::Draw(appearance) => appearance,
            _ => AppearanceDescriptor::object(799),
        };

        if *desired_appearance != appearance {
            *desired_appearance = appearance;
        }
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

fn move_to_cursor(
    q_cursor: Query<&mut TilePosition, With<Cursor>>,
    mut q_camera: Query<&mut Transform, With<Camera>>,
) {
    let tile_pos = q_cursor.single();

    for mut transform in q_camera.iter_mut() {
        let screen_pos: Vec2 = tile_pos.into();
        transform.translation = screen_pos.extend(transform.translation.z);
    }
}

fn update_cursor_sprite(
    mut cursor: Query<(&Cursor, &mut Sprite), (Without<BrushPreviewTile>, With<Cursor>)>,
    mut preview_query: Query<&mut Sprite, With<BrushPreviewTile>>,
) {
    let Ok((cursor, mut cursor_sprite)) = cursor.get_single_mut() else {
        return;
    };

    cursor_sprite.color = match cursor.drawing_state.tool_mode {
        ToolMode::Draw(_) => Color::WHITE,
        ToolMode::Erase => Color::CRIMSON,
        _ => Color::NONE,
    };

    if cursor_sprite.color != Color::NONE {
        cursor_sprite.color.set_a(0.5);
    }

    for mut sprite in preview_query.iter_mut() {
        *sprite = cursor_sprite.clone();
    }
}

fn update_cursor_visibility(
    mut egui_ctx: Query<&mut EguiContext>,
    mut windows: Query<&mut Window>,
    mut cursor_query: Query<(&Cursor, &TilePosition, &mut Visibility)>,
    gui_state: Res<UiState>,
) -> color_eyre::Result<()> {
    let mut egui_ctx = egui_ctx.single_mut();
    let (cursor, tile_pos, mut visibility) = cursor_query.get_single_mut()?;

    if gui_state.is_being_used || cursor.drawing_state.tool_mode.is_none() {
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
    for (mut tile_pos, mut appearance, mut visibility) in brush_preview_tiles.iter_mut() {
        // If the drawing_state is disable we are probably interacting with the UI, so we hide the preview.
        if cursor.drawing_state.tool_mode.is_none() {
            *visibility = Visibility::Hidden;
            continue;
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
    if cursor.drawing_state.tool_mode.is_none() {
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
