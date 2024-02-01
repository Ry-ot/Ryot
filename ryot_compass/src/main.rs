use bevy::app::AppExit;
use std::io::Cursor;

use bevy::prelude::*;
use bevy::winit::WinitWindows;

use bevy_egui::{EguiContexts, EguiPlugin};

mod error_handling;
use error_handling::ErrorState;
use ryot::prelude::*;

use ryot_compass::{
    check_egui_usage, AppPlugin, CameraPlugin, CompassContentAssets, CursorPos, GUIState,
    PalettePlugin, PaletteState,
};
use winit::window::Icon;

use rfd::AsyncFileDialog;

use crate::error_handling::ErrorPlugin;
use bevy::window::PrimaryWindow;
use ryot::prelude::sprites::load_sprites;
use ryot::prelude::sprites::*;
use ryot_compass::helpers::read_file;
use std::marker::PhantomData;

#[allow(clippy::too_many_arguments)]
fn draw<C: ConfigAssets + SpriteAssets>(
    mut commands: Commands,
    mut egui_ctx: EguiContexts,
    content_assets: Res<C>,
    cursor_pos: Res<CursorPos>,
    palette_state: Res<PaletteState>,
    mouse_button_input: Res<Input<MouseButton>>,
    error_states: Res<ErrorState>,
    mut build_spr_sheet_texture_cmd: EventWriter<LoadSpriteSheetTextureCommand>,
    configs: Res<Assets<ConfigAsset<ContentConfigs>>>,
) {
    if egui_ctx.ctx_mut().is_pointer_over_area() {
        return;
    }

    if error_states.has_error {
        return;
    }

    if content_assets.sprite_sheet_data_set().is_none() {
        return;
    };

    let Some(sprite_id) = palette_state.selected_tile else {
        return;
    };

    let sprites = load_sprites(
        &[sprite_id],
        &content_assets,
        &mut build_spr_sheet_texture_cmd,
    );

    let Some(sprite) = sprites.first() else {
        return;
    };

    if mouse_button_input.pressed(MouseButton::Left) {
        let tile_grid = configs.get(content_assets.config().id()).or_default().grid;

        let pos = tile_grid.get_tile_pos_from_display_pos(cursor_pos.0);

        draw_sprite(
            Vec3::new(pos.x, pos.y, 1.1),
            sprite,
            &mut commands,
            tile_grid,
        );
        debug!("Tile: {:?} drawn", pos);
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        for x in 0..200 {
            for y in 0..120 {
                let mut sprites = vec![195613];
                if x.ge(&20) && x.le(&30) && y.ge(&20) && y.le(&30) {
                    sprites.push(91267);
                }

                // let sprites = load_sprites_2(
                //     &sprites,
                //     sprite_sheets,
                //     &asset_server,
                //     &mut atlas_handlers,
                //     &mut texture_atlases,
                // );
                //
                // for (i, sprite) in sprites.iter().enumerate() {
                //     draw_sprite(
                //         Vec3::new(x as f32, y as f32, 1. + i as f32),
                //         sprite,
                //         &mut commands,
                //     );
                // }
            }
        }
    }

    // let loaded_monster = load_sprites(&vec![91267], &content.raw_content, &asset_server, &mut atlas_handlers, &mut texture_atlases);
    // if let Some(sprite) = loaded_monster.first() {
    //     for x in 20..30 {
    //         for y in 20..30 {
    //             draw_sprite(Vec3::new(x as f32, y as f32, 0.0), sprite, &mut commands);
    //         }
    //     }
    // }

    // let num_of_sprites = 400_689;
    // let sprites_per_row = (num_of_sprites as f32).sqrt() as u32;
    //
    // commands.spawn_batch((0..num_of_sprites).map(move |i| {
    //     let x = (i % sprites_per_row) as f32 * 50.0;
    //     let y = (i / sprites_per_row) as f32 * 50.0;
    //     SpriteBundle {
    //         texture: tile_handle_square.clone(),
    //         transform: Transform::from_xyz(x, y, 0.0),
    //         ..Default::default()
    //     }
    // }));
    //     counter.0 += 100_000;
    //
    //     return;
    // }
}

fn ui_example<C: SpriteAssets>(
    content_assets: Res<C>,
    mut egui_ctx: EguiContexts,
    mut exit: EventWriter<AppExit>,
    // content_sender: Res<EventSender<ContentWasLoaded>>,
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
        let image = image::open("assets/icons/compass_4.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary_window.set_window_icon(Some(icon));
}

pub struct UIPlugin<C: SpriteAssets>(PhantomData<C>);

impl<C: SpriteAssets> UIPlugin<C> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<C: SpriteAssets> Default for UIPlugin<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: AppearancesAssets + ConfigAssets + SpriteAssets> Plugin for UIPlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_optional_plugin(EguiPlugin)
            .init_resource::<GUIState>()
            .add_systems(First, check_egui_usage)
            .init_resource::<AboutMeOpened>()
            .add_systems(
                Update,
                (draw::<C>, ui_example::<C>)
                    .chain()
                    .run_if(in_state(InternalContentState::Ready)),
            );
    }
}

#[derive(Resource, Default)]
struct AboutMeOpened(bool);

fn main() {
    App::new()
        .add_plugins(AppPlugin)
        .add_plugins(UIPlugin::<CompassContentAssets>::new())
        .add_plugins(CameraPlugin::<CompassContentAssets>::new())
        .add_plugins(ErrorPlugin)
        .add_plugins(PalettePlugin::<CompassContentAssets>::new())
        .add_systems(Startup, set_window_icon)
        .add_systems(Startup, setup_window)
        .run();
}
