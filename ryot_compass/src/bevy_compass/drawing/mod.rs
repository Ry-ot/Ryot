use crate::{gui_is_not_in_use, helpers::CONTROL_COMMAND, MAP_GRAB_INPUTS};
use bevy::prelude::*;
use leafwing_input_manager::{common_conditions::action_just_pressed, prelude::*};
use ryot::prelude::{drawing::*, *};
use std::marker::PhantomData;

mod draw;
use draw::draw_to_tile;

mod delete;
use delete::delete_tile_content;

mod undo_redo;
use undo_redo::*;

mod brush;
pub use brush::*;

#[derive(Actionlike, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrawingAction {
    Stop,
    Draw,
    Erase,
    Undo,
    Redo,
    ChangeBrush,
    IncreaseBrush,
    DecreaseBrush,
    ClearSelection,
}

impl DrawingAction {
    pub fn get_default_input_map() -> InputMap<DrawingAction> {
        InputMap::default()
            .insert_multiple([
                (MouseButton::Left, DrawingAction::Draw),
                (MouseButton::Right, DrawingAction::Erase),
            ])
            .insert_modified(CONTROL_COMMAND, KeyCode::Z, DrawingAction::Undo)
            .insert_modified(CONTROL_COMMAND, KeyCode::R, DrawingAction::Redo)
            .insert(KeyCode::Key1, DrawingAction::ChangeBrush)
            .insert(KeyCode::Escape, DrawingAction::ClearSelection)
            .insert_modified(CONTROL_COMMAND, KeyCode::Plus, DrawingAction::IncreaseBrush)
            .insert_modified(
                CONTROL_COMMAND,
                KeyCode::Equals,
                DrawingAction::IncreaseBrush,
            )
            .insert_modified(
                CONTROL_COMMAND,
                KeyCode::Minus,
                DrawingAction::DecreaseBrush,
            )
            // Small hack to remove clash with the pancam plugin
            .insert_chord(MAP_GRAB_INPUTS, DrawingAction::Stop)
            .build()
    }
}

/// The drawing plugin is responsible for handling the core drawing logic and related commands.
/// It is also responsible for keeping track of a command history, used to perform undo/redo actions.
/// The plugin also registers the MapTiles resource, that keeps a map between position and layer in the
/// map and the entity that represents it.
pub struct DrawingPlugin<C: ContentAssets>(PhantomData<C>);

impl<C: ContentAssets> DrawingPlugin<C> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<C: ContentAssets> Default for DrawingPlugin<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: ContentAssets> Plugin for DrawingPlugin<C> {
    fn build(&self, app: &mut App) {
        app.init_resource::<UndoRedoConfig>()
            .init_resource::<CommandHistory>()
            .init_resource::<MapTiles>()
            .init_resource::<Brushes<DrawingBundle>>()
            .add_plugins(drawing::DrawingPlugin)
            .add_plugins(InputManagerPlugin::<DrawingAction>::default())
            .init_resource::<ActionState<DrawingAction>>()
            .add_systems(
                Update,
                (
                    draw_to_tile::<C>,
                    delete_tile_content,
                    undo_redo_tile_action,
                    update_brush.run_if(action_just_pressed(DrawingAction::ChangeBrush)),
                )
                    .run_if(in_state(InternalContentState::Ready))
                    .run_if(gui_is_not_in_use()),
            )
            .add_systems(
                OnExit(InternalContentState::PreparingSprites),
                spawn_grid(Color::WHITE),
            );
    }
}
