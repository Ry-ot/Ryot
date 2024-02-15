#![feature(type_alias_impl_trait)]
use bevy::app::{App, AppExit, Plugin};
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use leafwing_input_manager::common_conditions::action_toggle_active;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;
use ryot::bevy_ryot::sprites::animate_sprite_system;
use ryot::prelude::*;

#[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
pub mod lmdb;
#[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
pub use lmdb::*;

mod error_handling;
pub use error_handling::*;

pub mod helpers;
use helpers::*;

pub mod minimap;

mod bevy_compass;
pub use bevy_compass::*;

mod tileset;
pub use tileset::*;

mod ui;
pub use ui::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<InternalContentState>()
            .add_event::<AppExit>()
            .insert_resource(AssetMetaCheck::Never)
            .add_plugins(InputManagerPlugin::<ToggleFeatures>::default())
            .init_resource::<ActionState<ToggleFeatures>>()
            .insert_resource(
                InputMap::<ToggleFeatures>::default()
                    .insert_chord(
                        inputs![CONTROL_COMMAND, Modifier::Alt, KeyCode::F],
                        ToggleFeatures::Inspector,
                    )
                    .insert_chord(
                        inputs![CONTROL_COMMAND, Modifier::Alt, KeyCode::X],
                        ToggleFeatures::Animation,
                    )
                    .build(),
            )
            .add_plugins((
                DefaultPlugins
                    .set(entitled_window("Compass".to_string()))
                    .set(ImagePlugin::default_nearest()),
                ContentPlugin::<CompassContentAssets>::new(),
                WorldInspectorPlugin::default()
                    .run_if(action_toggle_active(false, ToggleFeatures::Inspector)),
            ))
            .add_systems(
                Update,
                animate_sprite_system.run_if(action_toggle_active(true, ToggleFeatures::Animation)),
            )
            .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)));
    }
}

#[derive(Actionlike, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
enum ToggleFeatures {
    Inspector,
    Animation,
}
