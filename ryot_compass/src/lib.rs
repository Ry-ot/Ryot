#![feature(type_alias_impl_trait)]
use bevy::app::{App, AppExit, Plugin};
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use leafwing_input_manager::common_conditions::{action_just_pressed, action_toggle_active};
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;
use ryot::bevy_ryot::sprites::toggle_sprite_animation;
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
                        ToggleFeatures::Inspector,
                        inputs![CONTROL_COMMAND, Modifier::Alt, KeyCode::F],
                    )
                    .insert_chord(
                        ToggleFeatures::Animation,
                        inputs![CONTROL_COMMAND, Modifier::Alt, KeyCode::X],
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
                toggle_sprite_animation.run_if(action_just_pressed(ToggleFeatures::Animation)),
            )
            .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)));
    }
}

#[derive(Actionlike, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
enum ToggleFeatures {
    Inspector,
    Animation,
}
