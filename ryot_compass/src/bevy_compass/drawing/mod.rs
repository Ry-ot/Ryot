use crate::{gui_is_not_in_use, MAP_GRAB_INPUTS};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;
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
}

impl DrawingAction {
    pub fn get_default_input_map() -> InputMap<DrawingAction> {
        let mut input_map = InputMap::new([
            (MouseButton::Left, DrawingAction::Draw),
            (MouseButton::Right, DrawingAction::Erase),
        ]);

        input_map.insert_chord(
            [
                InputKind::Modifier(Modifier::Control),
                InputKind::Keyboard(KeyCode::Z),
            ],
            DrawingAction::Undo,
        );
        input_map.insert_chord(
            [
                InputKind::Modifier(Modifier::Control),
                InputKind::Keyboard(KeyCode::R),
            ],
            DrawingAction::Redo,
        );

        input_map.insert_multiple([(KeyCode::Key1, DrawingAction::ChangeBrush)]);

        input_map.insert_chord(
            [KeyCode::ControlLeft, KeyCode::Plus],
            DrawingAction::IncreaseBrush,
        );
        input_map.insert_chord(
            [KeyCode::ControlLeft, KeyCode::Minus],
            DrawingAction::DecreaseBrush,
        );

        // Small hack to remove clash with the pancam plugin
        input_map.insert_chord(MAP_GRAB_INPUTS, DrawingAction::Stop);

        input_map
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
            .init_resource::<Brushes>()
            .add_plugins(drawing::DrawingPlugin)
            .add_plugins(InputManagerPlugin::<DrawingAction>::default())
            .add_systems(
                Update,
                (
                    draw_to_tile::<C>,
                    delete_tile_content,
                    undo_redo_tile_action,
                    update_brush,
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
