use bevy::app::AppExit;
use ryot::position::TilePosition;
use std::fmt::Debug;
use std::io::Cursor;

use bevy::prelude::*;
use bevy::winit::WinitWindows;

use bevy_egui::{EguiContexts, EguiPlugin};

mod error_handling;
use ryot::prelude::*;

use ryot_compass::{
    check_egui_usage, gui_is_not_in_use, AppPlugin, CameraPlugin, CompassContentAssets, CursorPos,
    GUIState, PalettePlugin, PaletteState, SelectedTile,
};
use winit::window::Icon;

use rfd::AsyncFileDialog;

use crate::error_handling::ErrorPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::system::{Command, EntityCommand, WithEntity};
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use color_eyre::owo_colors::OwoColorize;
use ryot::drawing::Layer;
use ryot::prelude::sprites::*;
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
trait ReversibleCommand: Command + Send + Sync + 'static {
    fn undo(&self, commands: &mut Commands);
}

trait ReversibleEntityCommand: EntityCommand + Send + Sync + 'static {
    fn undo(&self, entity: Entity, commands: &mut Commands);
}

enum UndoableCommand {
    Regular(Box<dyn ReversibleCommand>),
    Entity(Entity, Box<dyn ReversibleEntityCommand>),
}

#[derive(Debug, Default, Resource, Deref, DerefMut)]
pub struct MapTiles(HashMap<TilePosition, HashMap<Layer, Entity>>);

#[derive(Debug, Clone)]
pub struct AddTileContent(TilePosition, LoadedSprite, PreparedAppearance);
impl Command for AddTileContent {
    fn apply(self, world: &mut World) {
        let AddTileContent(tile_pos, loaded_sprite, prepared_appearance) = self;

        let layer = prepared_appearance.layer;

        if let Some(bundle) = build_sprite_bundle(
            loaded_sprite.get_sprite_index(),
            tile_pos.with_z(10 + layer.base_z_offset()),
            loaded_sprite.atlas_texture_handle.clone(),
        ) {
            let new_entity = world
                .spawn((bundle, tile_pos, loaded_sprite, prepared_appearance))
                .id();
            {
                let mut map_tiles = world.resource_mut::<MapTiles>();
                let content = map_tiles.entry(self.0.clone()).or_default();
                content.insert(layer, new_entity);
            }
        }
    }
}

impl ReversibleCommand for AddTileContent {
    fn undo(&self, commands: &mut Commands) {
        commands.add(ChangeTileContentVisibility(
            self.0.clone(),
            Visibility::Hidden,
        ));
    }
}

#[derive(Debug, Clone)]
pub struct ChangeTileContentVisibility(TilePosition, Visibility);

impl Command for ChangeTileContentVisibility {
    fn apply(self, world: &mut World) {
        let ChangeTileContentVisibility(tile_pos, tile_visibility) = self;

        // Separate the entities to modify from the MapTiles resource borrowing scope
        let entities_to_modify = {
            let map_tiles = world.resource_mut::<MapTiles>();
            map_tiles.get(&tile_pos).map(|content| {
                content
                    .iter()
                    .map(|(_, &entity)| entity)
                    .collect::<Vec<_>>()
            })
        };

        // Apply changes to entities outside of the MapTiles borrowing scope
        if let Some(entities) = entities_to_modify {
            for entity in entities {
                if let Some(mut visibility) = world.get_mut::<Visibility>(entity) {
                    *visibility = tile_visibility;
                }
            }
        }
    }
}

impl ReversibleCommand for ChangeTileContentVisibility {
    fn undo(&self, commands: &mut Commands) {
        commands.add(ChangeTileContentVisibility(
            self.0.clone(),
            match self.1 {
                Visibility::Hidden => Visibility::Visible,
                Visibility::Visible => Visibility::Hidden,
                _ => self.1,
            },
        ));
    }
}

#[derive(Default, Resource)]
pub struct CommandHistory {
    commands: Vec<UndoableCommand>,
}

#[derive(Debug, Clone)]
pub struct UpdateTileContent(Option<LoadedSprite>, Option<LoadedSprite>);
impl EntityCommand for UpdateTileContent {
    fn apply(self, id: Entity, world: &mut World) {
        let UpdateTileContent(loaded_sprite, _) = &self;

        let Some(loaded_sprite) = loaded_sprite else {
            world.despawn(id);
            return;
        };

        if let Some(mut loaded) = world.get_mut::<LoadedSprite>(id) {
            *loaded = loaded_sprite.clone();
        }

        if let Some(mut atlas_sprite) = world.get_mut::<TextureAtlasSprite>(id) {
            atlas_sprite.index = loaded_sprite.get_sprite_index();
        }

        if let Some(mut atlas_handle) = world.get_mut::<Handle<TextureAtlas>>(id) {
            *atlas_handle = loaded_sprite.atlas_texture_handle.clone();
        }

        if let Some(mut visibility) = world.get_mut::<Visibility>(id) {
            *visibility = Visibility::Visible;
        }
    }
}

impl ReversibleEntityCommand for UpdateTileContent {
    fn undo(&self, entity: Entity, commands: &mut Commands) {
        commands.add(UpdateTileContent(self.1.clone(), self.0.clone()).with_entity(entity));
    }
}

#[derive(Debug, Default, Event)]
pub struct DrawIntoPositionCommand(TilePosition, PreparedAppearance);

fn update_map_from_mouse_input<C: ContentAssets>(
    mut commands: Commands,
    mut tiles: ResMut<MapTiles>,
    mut command_history: ResMut<CommandHistory>,
    content_assets: Res<C>,
    cursor_pos: Res<CursorPos>,
    palette_state: Res<PaletteState>,
    mut loaded_query: Query<&mut LoadedSprite>,
    mouse_button_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if content_assets.sprite_sheet_data_set().is_none() {
        warn!("Trying to draw a sprite without any loaded content");
        return;
    };

    let Some((content_id, loaded_sprite)) = &palette_state.selected_tile else {
        return;
    };

    let Some(prepared_appearance) = content_assets
        .prepared_appearances()
        .get_for_group(AppearanceGroup::Object, *content_id)
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
        let command =
            ChangeTileContentVisibility(TilePosition::from(cursor_pos.0), Visibility::Hidden);
        commands.add(command.clone());
        command_history
            .commands
            .push(UndoableCommand::Regular(Box::new(command)));
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        let tile_pos = TilePosition::from(cursor_pos.0);
        command_history.commands.push(
            match tiles
                .entry(tile_pos.clone())
                .or_default()
                .get(&prepared_appearance.layer)
            {
                Some(entity) => {
                    let current = loaded_query.get(*entity).unwrap();
                    let command =
                        UpdateTileContent(Some(loaded_sprite.clone()), Some(current.clone()));

                    commands.get_entity(*entity).unwrap().add(command.clone());

                    UndoableCommand::Entity(entity.clone(), Box::new(command.clone()))
                }
                None => {
                    let command = AddTileContent(
                        tile_pos.clone(),
                        loaded_sprite.clone(),
                        prepared_appearance.clone(),
                    );
                    commands.add(command.clone());
                    UndoableCommand::Regular(Box::new(command))
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
