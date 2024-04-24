use bevy_app::{App, Plugin};

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
/// use bevy_app::prelude::*;
/// use bevy_app::MainSchedulePlugin;
///
/// pub struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn build(&self, app: &mut App) {
///         if !app.is_plugin_added::<MainSchedulePlugin>() {
///             app.add_plugins(MainSchedulePlugin);
///         }
///     }
/// }
/// ```
/// You can do
/// ```rust
/// use bevy_app::prelude::*;
/// use bevy_app::MainSchedulePlugin;
/// use ryot_core::prelude::OptionalPlugin;
///
/// pub struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn build(&self, app: &mut App) {
///        app.add_optional_plugin(MainSchedulePlugin);
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
