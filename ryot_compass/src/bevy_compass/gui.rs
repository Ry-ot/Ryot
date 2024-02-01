use bevy::prelude::*;
use bevy_egui::EguiContexts;

/// The GUIState resource is used to keep track of whether the GUI is being used.
/// This is useful for systems that should only run when the GUI is/is not being used.
/// For example, drawing systems should only run when the GUI is not being used.
#[derive(Resource, Default)]
pub struct GUIState {
    pub is_being_used: bool,
}

/// This condition checks if the GUI is being used and can be used with run_if.
/// ```rust
/// use bevy::prelude::*;
/// use bevy_egui::EguiContexts;
/// use ryot_compass::gui_is_in_use;
///
/// fn gui_is_active_system() {
///     info!("GUI is active");
/// }
///
/// fn main() {
///   App::new().add_systems(Update, gui_is_active_system.run_if(gui_is_in_use()));
/// }
/// ```
pub fn gui_is_in_use() -> impl FnMut(Res<GUIState>) -> bool + Clone {
    move |gui_state| gui_state.is_being_used
}

/// This condition checks if the GUI is not being used and can be used with run_if.
/// ```rust
/// use bevy::prelude::*;
/// use bevy_egui::EguiContexts;
/// use ryot_compass::gui_is_not_in_use;
///
/// fn gui_is_not_active_system() {
///     info!("GUI is not active");
/// }
///
/// fn main() {
///   App::new().add_systems(Update, gui_is_not_active_system.run_if(gui_is_not_in_use()));
/// }
/// ```
pub fn gui_is_not_in_use() -> impl FnMut(Res<GUIState>) -> bool + Clone {
    move |gui_state| !gui_state.is_being_used
}

/// This system updates the GUIState resource to indicate whether EGUI is being used or not.
pub fn check_egui_usage(egui: EguiContexts, mut gui_state: ResMut<GUIState>) {
    gui_state.is_being_used = egui.ctx().is_pointer_over_area()
}
