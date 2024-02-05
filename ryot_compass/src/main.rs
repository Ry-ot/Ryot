use bevy::app::AppExit;
use ryot::position::TilePosition;
use std::io::Cursor;

use bevy::prelude::*;
use bevy::winit::WinitWindows;

use bevy_egui::{EguiContexts, EguiPlugin};

mod error_handling;
use ryot::prelude::*;

use ryot_compass::{
    check_egui_usage, gui_is_not_in_use, AppPlugin, CameraPlugin, CompassContentAssets, CursorPos,
    GUIState, PalettePlugin, PaletteState,
};
use winit::window::Icon;

use rfd::AsyncFileDialog;

use crate::error_handling::ErrorPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::system::EntityCommand;
use bevy::window::PrimaryWindow;
use ryot::drawing::Layer;
use ryot::drawing_commands::{
    AddTileContent, ChangeTileContentVisibility, CommandHistory, MapTiles, UndoableCommand,
    UpdateTileContent,
};
use ryot_compass::helpers::read_file;
use std::marker::PhantomData;

/*
Drawing levels (keeping it around 100k sprites per level):
- Max details: 1 floor, 1 top, 1 bottom, 1 ground and 10 contents - ~64x64
- Medium details: 1 floor: 1 top, 1 bottom, 1 ground and 5 content - ~112x112
- Minimal details: 1 floor: 1 top, 1 bottom, 1 ground and 1 content - ~160x160
- Ground+top: 1 floor, 1 top, 1 ground - 224x224
- Ground only - 320x320
- >320x320 - Not possible (maybe chunk view so that people can navigate through the map quicker in the future)
- Draw rules change per detail level

We load 2-3x the current view but we only set as visible the 1.1 * view.
As we move the camera, we set the new tiles as visible and the old ones as hidden and we deload/load the edges (as hidden)
As we zoom in and out, we change the detail level of the tiles and change visibility accordingly.

So when a click happens the first tihng that it does is a c
*/

#[allow(clippy::too_many_arguments)]
fn update_map_from_mouse_input<C: ContentAssets>(
    mut commands: Commands,
    mut tiles: ResMut<MapTiles>,
    mut command_history: ResMut<CommandHistory>,
    content_assets: Res<C>,
    cursor_pos: Res<CursorPos>,
    palette_state: Res<PaletteState>,
    current_appearance_query: Query<(&mut AppearanceDescriptor, &Visibility)>,
    mouse_button_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if content_assets.sprite_sheet_data_set().is_none() {
        warn!("Trying to draw a sprite without any loaded content");
        return;
    };

    let Some(AppearanceDescriptor { group, id, .. }) = &palette_state.selected_tile else {
        return;
    };

    let Some(prepared_appearance) = content_assets
        .prepared_appearances()
        .get_for_group(group.clone(), *id)
    else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::U) {
        if let Some(command) = command_history.commands.pop() {
            info!("Undoing command: {}", command_history.commands.len());
            match command {
                UndoableCommand::Regular(command) => {
                    command.undo(&mut commands);
                }
                UndoableCommand::Entity(entity, command) => {
                    command.undo(entity, &mut commands);
                }
            }
        }
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        let tile_pos = TilePosition::from(cursor_pos.0);
        // let command =
        //     ChangeTileContentVisibility(TilePosition::from(cursor_pos.0), Visibility::Hidden);
        // commands.add(command.clone());
        // command_history
        //     .commands
        //     .push(UndoableCommand::Regular(Box::new(command)));

        let Some(tile_content) = tiles.get(&tile_pos) else {
            return;
        };

        let mut content: Option<(Layer, &AppearanceDescriptor)> = None;

        for layer in [Layer::Top, Layer::None, Layer::Bottom, Layer::Ground] {
            if let Some(top) = tile_content.get(&layer) {
                if let Ok((current, visibility)) = current_appearance_query.get(*top) {
                    if visibility == Visibility::Hidden {
                        continue;
                    }

                    content = Some((layer, current));
                    break;
                }
            }
        }

        let Some((layer, _)) = content else {
            return;
        };

        let command = ChangeTileContentVisibility(tile_pos, Visibility::Hidden, layer);
        commands.add(command.clone());
        command_history
            .commands
            .push(UndoableCommand::Regular(Box::new(command)));
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        let tile_pos = TilePosition::from(cursor_pos.0);
        let desired_appearance = AppearanceDescriptor::new(group.clone(), *id, default());
        command_history.commands.push(
            match tiles
                .entry(tile_pos)
                .or_default()
                .get(&prepared_appearance.layer)
            {
                Some(entity) => {
                    let (current, _) = current_appearance_query.get(*entity).unwrap();
                    let command =
                        UpdateTileContent(Some(desired_appearance), Some(current.clone()));

                    commands.add(command.clone().with_entity(*entity));

                    UndoableCommand::Entity(*entity, Box::new(command.clone()))
                }
                None => {
                    let entity = commands.spawn_empty().id();
                    let command =
                        AddTileContent(tile_pos, desired_appearance, prepared_appearance.layer);
                    commands.add(command.clone().with_entity(entity));
                    UndoableCommand::Entity(entity, Box::new(command.clone()))
                }
            },
        );
    }
}

fn ui_example<C: ContentAssets>(
    content_assets: Res<C>,
    mut egui_ctx: EguiContexts,
    mut exit: EventWriter<AppExit>,
    mut about_me: ResMut<AboutMeOpened>,
    mut _windows: NonSend<WinitWindows>,
) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.scope(|ui| {
                let mut style = (*ui.ctx().style()).clone();

                // Modify the style for your specific widget
                style.visuals.widgets.inactive.bg_fill = egui::Color32::GRAY;
                style.visuals.widgets.active.bg_fill = egui::Color32::GRAY;
                style.visuals.widgets.hovered.bg_fill = egui::Color32::GRAY;

                // Temporarily apply the style
                ui.set_style(style);

                let is_content_loaded = content_assets.sprite_sheet_data_set().is_some();

                // Load the image using `image-rs`
                // let image_data = include_bytes!("path/to/your/image.png").to_vec();
                // let image = image::RgbaImage::from_raw(1024, 1024, image_data);
                //
                // // Create an `egui::TextureHandle`
                // let texture_handle = egui::TextureHandle::from_rgba_unmultiplied(
                //     ctx,
                //     egui::ColorImage::from_rgba_unmultiplied(size, &image_data)
                // );

                // let img = egui::include_image!("../assets/icons/compass_2.png");
                //
                // ui.image(img);

                egui::menu::menu_button(ui, "File", |ui| {
                    // #[cfg(not(target_arch = "wasm32"))]
                    if ui
                        .add_enabled(is_content_loaded, egui::Button::new("🗁 Open"))
                        .clicked()
                    {
                        read_file(
                            AsyncFileDialog::new().add_filter(".mdb, .otbm", &["mdb", "otbm"]),
                            |(file_name, content)| {
                                debug!("Loading map from file: {:?}", file_name);
                                debug!("Content: {:?}", content);
                                debug!("Current dir: {:?}", std::env::current_dir());
                            },
                        );

                        // let path = rfd::FileDialog::new()
                        //     .add_filter(".mdb, .otbm", &["mdb", "otbm"])
                        //     .pick_file();
                        //
                        // debug!("Loading map from file: {:?}", path);
                        // debug!("Current dir: {:?}", std::env::current_dir());
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    if ui
                        .add_enabled(is_content_loaded, egui::Button::new("💾 Save"))
                        .clicked()
                    {
                        let path = rfd::FileDialog::new()
                            .add_filter(".mdb, .otbm", &["mdb", "otbm"])
                            .save_file();

                        debug!("Saving map to file: {:?}", path);
                    }

                    ui.separator();

                    // #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Load Content").clicked() {
                        // let sender = content_sender.0.clone();

                        read_file(
                            AsyncFileDialog::new().add_filter(".json", &["json"]),
                            move |(file_name, _loaded)| {
                                debug!("Loading content from file: {:?}", file_name);
                                // let Some(content_was_loaded) =
                                //     ContentWasLoaded::from_bytes(file_name.clone(), loaded.clone())
                                // else {
                                //     warn!("Failed to load content from file: {:?}", file_name);
                                //     return;
                                // };

                                // sender
                                //     .send(content_was_loaded)
                                //     .expect("Failed to send content loaded event");
                            },
                        );
                    }

                    ui.add_enabled(is_content_loaded, egui::Button::new("🔃 Refresh Content"))
                        .clicked();

                    ui.separator();

                    ui.button("⚙ Settings").clicked();

                    ui.separator();

                    if ui.button("Exit").clicked() {
                        exit.send(AppExit);
                    }
                });

                egui::menu::menu_button(ui, "View", |ui| {
                    if ui.button("Windows").clicked() {
                        // Open action
                    }
                });

                egui::menu::menu_button(ui, "Help", |ui| {
                    if ui.button("About").clicked() {
                        about_me.0 = true;
                    }
                });

                // ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                //     if ui.button("⚙").clicked() {
                //     }
                // })
            });
        });
    });

    egui::Window::new("About Ryot Compass")
        .auto_sized()
        .collapsible(false)
        .movable(false)
        .default_pos(egui::pos2(100.0, 100.0)) // Adjust position as needed
        .open(&mut about_me.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("About Me information...");
        });
}

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

#[derive(Resource, Default)]
struct AboutMeOpened(bool);

pub struct UIPlugin<C: ContentAssets>(PhantomData<C>);

impl<C: ContentAssets> UIPlugin<C> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<C: ContentAssets> Default for UIPlugin<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: ContentAssets> Plugin for UIPlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_optional_plugin(EguiPlugin)
            .init_resource::<GUIState>()
            .add_systems(First, check_egui_usage)
            .init_resource::<AboutMeOpened>()
            .add_systems(
                Update,
                (update_map_from_mouse_input::<C>, ui_example::<C>)
                    .chain()
                    .run_if(in_state(InternalContentState::Ready))
                    .run_if(gui_is_not_in_use()),
            );
    }
}

fn main() {
    App::new()
        .init_resource::<CommandHistory>()
        .init_resource::<MapTiles>()
        .add_plugins((
            AppPlugin,
            UIPlugin::<CompassContentAssets>::new(),
            CameraPlugin::<CompassContentAssets>::new(),
            ErrorPlugin,
            PalettePlugin::<CompassContentAssets>::new(),
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, set_window_icon)
        .add_systems(Startup, setup_window)
        .run();
}
