use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::EguiContexts;

#[derive(Resource, Default)]
pub struct ErrorState {
    pub has_error: bool,
    pub error_message: String,
    pub close_requested: bool,
}

impl ErrorState {
    pub fn new(error_message: String) -> Self {
        Self {
            has_error: true,
            error_message,
            close_requested: false,
        }
    }
}
pub fn display_error_window(mut egui_ctx: EguiContexts, mut error_state: ResMut<ErrorState>) {
    if error_state.has_error {
        egui::Window::new("Error").show(egui_ctx.ctx_mut(), |ui| {
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
