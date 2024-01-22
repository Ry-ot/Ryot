/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */

// use bevy::app::{AppExit, PluginGroupBuilder};
//
// use bevy::reflect::TypeUuid;
// use bevy::render::mesh::{Indices, PrimitiveTopology};
// use bevy::render::render_resource::{AsBindGroup, ShaderRef};
// use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};
// use bevy::window::PrimaryWindow;
// use bevy::winit::WinitWindows;
// use bevy::{input::common_conditions::input_toggle_active, prelude::*};
// use std::fmt::Debug;
//
// use bevy_egui::{EguiContexts, EguiPlugin};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use egui::load::SizedTexture;
// use egui::TextureId;
// use itertools::Itertools;
//
// use crate::error_handling::{check_for_exit, display_error_window, ErrorState};
// use crate::helpers::camera::movement as camera_movement;
// use ryot::*;
//
// // #[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
// // use ryot_compass::item::ItemsFromHeedLmdb;
//
// // #[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
// // use ryot_compass::lmdb::LmdbEnv;
//
// use crate::{
//     build, draw_palette_window, draw_sprite, load_sprites, normalize_tile_pos_to_sprite_pos,
//     test_reload_config, Appearance, AppearanceAssetPlugin, AppearanceHandle, AsyncEventsExtension,
//     CipContent, ConfigExtension, DecompressedCache, EventSender, Palette, PaletteState, Settings,
//     TextureAtlasHandlers, Tile, TilesetCategory,
// };
// use winit::window::Icon;
//
// use bevy::asset::AssetMetaCheck;
// use rfd::AsyncFileDialog;
// use ryot::appearances::ContentType;
//
// use std::future::Future;
//
// pub struct CompassPlugin;
//
// impl Plugin for CompassPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_event::<AppExit>();
//
//         app.add_plugins((DefaultPlugins
//             .set(WindowPlugin {
//                 primary_window: Some(Window {
//                     title: "Compass".to_string(),
//                     // Bind to canvas included in `index.html`
//                     canvas: Some("#bevy".to_owned()),
//                     // The canvas size is constrained in index.html and build/web/styles.css
//                     fit_canvas_to_parent: true,
//                     // Tells wasm not to override default event handling, like F5 and Ctrl+R
//                     prevent_default_event_handling: false,
//                     ..default()
//                 }),
//                 ..default()
//             })
//             .set(ImagePlugin::default_nearest()),));
//
//         // This is needed to make AssetsServer work on WASM
//         // See https://github.com/bevyengine/bevy/pull/10623
//         // Maybe add wasm target only (?)
//         app.insert_resource(AssetMetaCheck::Never);
//         app.insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)));
//     }
// }
//
// pub struct CompassPlugins;
// impl PluginGroup for CompassPlugins {
//     fn build(self) -> PluginGroupBuilder {
//         let mut group = PluginGroupBuilder::default();
//
//         group = group.add(CompassPlugin).add(AppearanceAssetPlugin);
//
//         group.build()
//     }
// }
//
// fn main() {
//     App::new()
//         .add_plugins(CompassPlugins)
//         // .init_resource::<ErrorState>()
//         // .add_async_event::<ContentWasLoaded>()
//         .add_config::<ContentConfigs>(CONTENT_CONFIG_PATH)
//         // .insert_resource(build())
//         // .init_resource::<LmdbEnv>()
//         .init_resource::<Settings>()
//         .init_resource::<Palette>()
//         .init_resource::<AboutMeOpened>()
//         .init_resource::<TextureAtlasHandlers>()
//         .init_resource::<CipContent>()
//         .init_resource::<CursorPos>()
//         .init_resource::<Tiles>()
//         .init_resource::<PaletteState>()
//         .add_plugins((
//             EguiPlugin,
//             WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
//             // MinimapPlugin,
//         ))
//         .add_systems(Startup, spawn_camera)
//         // .add_systems(Startup, setup_window)
//         // .add_systems(Startup, init_env.before(load_tiles))
//         .add_systems(Startup, spawn_cursor)
//         // .add_systems(Startup, load_tiles)
//         // .add_systems(Startup, decompress_all_sprites)
//         .add_systems(First, (camera_movement, update_cursor_pos).chain())
//         // .add_systems(Update, decompress_all_sprites)
//         .add_systems(Update, draw)
//         // .add_systems(Update, draw_tiles_on_minimap)
//         // .add_systems(Update, scroll_events)
//         .add_systems(Update, ui_example)
//         .add_systems(Update, print_appearances)
//         .add_systems(Update, print_settings)
//         .add_systems(Update, display_error_window)
//         .add_systems(Update, check_for_exit)
//         .add_systems(Update, update_cursor)
//         .add_systems(Update, handle_content_loaded)
//         .add_systems(Update, test_reload_config::<ContentConfigs>)
//         .run();
// }
