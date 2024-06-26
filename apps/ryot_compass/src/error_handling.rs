use crate::RyotContentState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::EguiContext;

pub struct ErrorPlugin;

impl Plugin for ErrorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ErrorState>().add_systems(
            Update,
            (display_error_window, check_for_exit).run_if(in_state(RyotContentState::Ready)),
        );
    }
}

#[derive(Resource, Default)]
pub struct ErrorState {
    pub has_error: bool,
    pub error_message: String,
    pub close_requested: bool,
}

#[allow(dead_code)]
impl ErrorState {
    pub fn new(error_message: String) -> Self {
        Self {
            has_error: true,
            error_message,
            close_requested: false,
        }
    }
}
pub fn display_error_window(
    mut egui_ctx: Query<&mut EguiContext>,
    mut error_state: ResMut<ErrorState>,
) {
    let mut egui_ctx = egui_ctx.single_mut();
    if error_state.has_error {
        egui::Window::new("Error").show(egui_ctx.get_mut(), |ui| {
            ui.label(&error_state.error_message);
            if ui.button("OK").clicked() {
                error_state.close_requested = true;
            }
        });
    }
}

pub fn check_for_exit(error_state: Res<ErrorState>, mut app_exit_events: EventWriter<AppExit>) {
    if error_state.close_requested {
        app_exit_events.send(AppExit);
    }
}
