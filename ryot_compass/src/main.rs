use std::io::Cursor;

use bevy::prelude::*;
use bevy::winit::WinitWindows;

use bevy_egui::EguiContexts;

mod error_handling;
use ryot::prelude::*;

#[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
use ryot::prelude::lmdb::LmdbEnv;
#[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
use ryot_compass::{init_tiles_db, read_area};

use ryot_compass::{
    AppPlugin, CameraPlugin, CompassContentAssets, DrawingPlugin, PalettePlugin, UiPlugin,
};
use winit::window::Icon;

use crate::error_handling::ErrorPlugin;
use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
    SystemInformationDiagnosticsPlugin,
};
use bevy::window::PrimaryWindow;

fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}

pub fn setup_window(
    mut egui_ctx: EguiContexts,
    windows: NonSend<WinitWindows>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
) {
    egui_extras::install_image_loaders(egui_ctx.ctx_mut());

    let primary_window_entity = primary_window_query.single();
    let primary_window = windows.get_window(primary_window_entity).unwrap();

    let (icon_rgba, icon_width, icon_height) = {
        let Ok(image) = image::open("assets/icons/compass_4.png") else {
            error!("Failed to load icon image");
            return;
        };
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary_window.set_window_icon(Some(icon));
}

fn main() {
    // lmdb_example().unwrap();
    color_eyre::install().unwrap();
    let mut app = App::new();

    app.add_plugins((
        AppPlugin,
        UiPlugin::<CompassContentAssets>::default(),
        CameraPlugin::<CompassContentAssets>::default(),
        PalettePlugin::<CompassContentAssets>::default(),
        DrawingPlugin::<CompassContentAssets>::default(),
        ErrorPlugin,
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
        LogDiagnosticsPlugin::default(),
    ))
    .add_systems(Startup, set_window_icon)
    .add_systems(Startup, setup_window);

    #[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
    app.init_resource::<LmdbEnv>()
        .add_systems(Startup, init_tiles_db.map(drop))
        .add_systems(
            Update,
            read_area.run_if(in_state(InternalContentState::Ready)),
        );

    app.run();
}
