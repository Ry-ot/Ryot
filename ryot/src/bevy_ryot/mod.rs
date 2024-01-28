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

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum InternalContentState {
    #[default]
    LoadingContent,
    PreparingContent,
    LoadingSprites,
    PreparingSprites,
    Ready,
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
        if !self.is_plugin_added::<T>() {
            self.add_plugins(plugin);
        }

        self
    }
}
