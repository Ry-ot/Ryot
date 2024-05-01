use bevy_app::{App, Plugin};
use bevy_ecs::prelude::{FromWorld, Resource};

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
/// use ryot_utils::prelude::OptionalPlugin;
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

/// Helper trait to initialize a resource only if it hasn't been added already.
/// This is useful for external resources that are used by multiple plugins or dependencies
/// and avoids resetting the resource if it has already been initialized.
///
/// # Example
/// You have a resource that is used by multiple plugins, and you want to avoid resetting it
/// if it has already been initialized. You can use init_resource_once::<MyResource> in your plugins
/// to avoid resetting the resource.
///
/// So instead of having:
/// ```rust
/// use bevy_app::prelude::*;
/// use bevy_ecs::prelude::Resource;
///
/// pub struct MyPlugin;
///
/// #[derive(Resource, Default)]
/// pub struct MyResource;
///
/// impl Plugin for MyPlugin {
///    fn build(&self, app: &mut App) {
///       if !app.world.is_resource_added::<MyResource>() {
///          app.init_resource::<MyResource>();
///      }
///   }
/// }
/// ```
///
/// You can do:
/// ```rust
/// use bevy_app::prelude::*;
/// use bevy_ecs::prelude::Resource;
/// use ryot_utils::prelude::InitResourceOnce;
///
/// pub struct MyPlugin;
///
/// #[derive(Resource, Default)]
/// pub struct MyResource;
///
/// impl Plugin for MyPlugin {
///     fn build(&self, app: &mut App) {
///         app.init_resource_once::<MyResource>();
///     }
/// }
/// ```
pub trait InitResourceOnce {
    fn init_resource_once<R: Resource + FromWorld>(&mut self) -> &mut Self;
}

impl InitResourceOnce for App {
    fn init_resource_once<R: Resource + FromWorld>(&mut self) -> &mut Self {
        if !self.world.is_resource_added::<R>() {
            self.init_resource::<R>();
        }

        self
    }
}
