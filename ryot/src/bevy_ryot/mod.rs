//! Bevy plugins and utilities for RyOT games.
//!
//! This module is intended to be used as a library dependency for RyOT games.
//! It provides common ways of dealing with OT content, such as loading sprites and appearances,
//! configuring the game, and handling asynchronous events.
mod appearances;
pub use appearances::*;

mod async_events;
pub use async_events::*;

mod configs;
pub use configs::*;

mod content;
pub use content::*;

pub mod sprites;

#[cfg(test)]
mod tests;

use bevy::app::{App, Plugin};
use bevy::prelude::{default, States, Window, WindowPlugin};

/// The states that the content loading process can be in.
/// This is used to track the progress of the content loading process.
/// It's also used to determine if the content is ready to be used.
/// It's internally used by the `ContentPlugin` and should not be manipulated directly.
/// Can be checked by applications to perform actions that depend on the state of the content.
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum InternalContentState {
    #[default]
    LoadingContent,
    PreparingContent,
    LoadingSprites,
    PreparingSprites,
    Ready,
}

/// Quick way to create WASM compatible windows with a title.
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

/// Helper trait to add plugins only if they haven't been added already.
/// This is useful for external plugins that are used by multiple plugins or dependencies
/// and should only be added once.
///
/// # Example
/// You have a UI plugin dependent on Egui but you also use Bevy's inspector plugin that uses Egui.
/// You can use add_optional_plugin(EguiPlugin) in your UI plugin to avoid adding EguiPlugin twice,
/// clashing with the inspector plugin.
///
/// So instead of having
/// ```rust
/// use bevy::prelude::*;
/// use bevy::time::TimePlugin;
///
/// pub struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn build(&self, app: &mut App) {
///         if !app.is_plugin_added::<TimePlugin>() {
///             app.add_plugins(TimePlugin);
///         }
///
///        //...
///     }
/// }
/// ```
/// You can do
/// ```rust
/// use bevy::prelude::*;
/// use bevy::time::TimePlugin;
/// use ryot::prelude::OptionalPlugin;
///
/// pub struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn build(&self, app: &mut App) {
///        app.add_optional_plugin(TimePlugin);
///
///        //...
///     }
/// }
/// ```
pub trait OptionalPlugin {
    fn add_optional_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self;
}

impl OptionalPlugin for App {
    fn add_optional_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        if !self.is_plugin_added::<T>() {
            self.add_plugins(plugin);
        }

        self
    }
}
