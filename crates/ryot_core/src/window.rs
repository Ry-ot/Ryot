use bevy_utils::default;
use bevy_window::{Window, WindowPlugin};

/// Quick way to create WASM compatible windows with a title.
pub fn entitled_window(title: String) -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            name: Some(title.clone()),
            title,
            // Bind to canvas included in `index.html`
            canvas: Some("#bevy".to_owned()),
            #[cfg(target_arch = "wasm32")]
            mode: bevy::window::WindowMode::SizedFullscreen,
            ..default()
        }),
        ..default()
    }
}
