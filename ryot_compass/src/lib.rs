use crate::sprites::SpritesPlugin;
use bevy::app::{App, AppExit, Plugin};
use bevy::asset::AssetMetaCheck;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod item;

#[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
pub mod lmdb;
#[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
pub use lmdb::*;

mod generator;
pub use generator::{build_map, get_chunks_per_z};

mod plan;
pub use plan::*;

mod serde;
pub use serde::types::*;

mod error;
pub use error::*;

mod error_handling;
pub use error_handling::*;
use ryot::prelude::*;

pub mod helpers;

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
            .add_plugins((
                DefaultPlugins
                    .set(entitled_window("Compass".to_string()))
                    .set(ImagePlugin::default_nearest()),
                ContentPlugin::<CompassContentAssets>::new(),
                SpritesPlugin::<CompassSpriteAssets>::new(),
                WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            ))
            .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)));
    }
}
