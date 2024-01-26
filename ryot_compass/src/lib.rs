use crate::sprites::SpritesPlugin;
use bevy::app::{App, AppExit, Plugin};
use bevy::asset::AssetMetaCheck;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppStates {
    #[default]
    LoadingContent,
    PreparingContent,
    LoadingSprites,
    PreparingSprites,
    Ready,
}

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

pub mod helpers;

pub mod minimap;

mod ryot_bevy;
pub use ryot_bevy::*;

mod tileset;
pub use tileset::*;

mod ui;
pub use ui::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppStates>()
            .add_event::<AppExit>()
            .insert_resource(AssetMetaCheck::Never)
            .add_plugins((
                DefaultPlugins
                    .set(entitled_window("Compass".to_string()))
                    .set(ImagePlugin::default_nearest()),
                ContentPlugin,
                SpritesPlugin,
                WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            ))
            .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)));
    }
}

pub fn entitled_window(title: String) -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title,
            // Bind to canvas included in `index.html`
            canvas: Some("#bevy".to_owned()),
            // The canvas size is constrained in index.html and build/web/styles.css
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5 and Ctrl+R
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }
}

pub trait OptionalPlugin {
    fn add_optional_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self;
}

impl OptionalPlugin for App {
    fn add_optional_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        if !self.is_plugin_added::<EguiPlugin>() {
            self.add_plugins(plugin);
        }

        self
    }
}
