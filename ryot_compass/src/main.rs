use bevy::app::AppExit;
use bevy::math::Vec4Swizzles;
use bevy::{
    ecs::system::Resource,
    input::{common_conditions::input_toggle_active, mouse::MouseWheel},
    prelude::*,
};
use std::io::{Read, Seek};

use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use egui::load::SizedTexture;
use egui::TextureId;
use image::Pixel;
use itertools::Itertools;
use prost::Message;
use rand::prelude::{IteratorRandom, SliceRandom};
use rand::Rng;
use time_test::time_test;

mod error_handling;
mod helpers;
use error_handling::{check_for_exit, display_error_window, ErrorState};
use helpers::camera::movement as camera_movement;
use rayon::prelude::*;
use ryot::cip_content::{
    decompress_all_sprite_sheets, get_full_file_buffer, load_content, Appearances, ContentType,
    SheetGrid, SpriteInfo,
};
use ryot_compass::item::{ItemRepository, ItemsFromHeedLmdb};
use ryot_compass::minimap::{Minimap, MinimapPlugin};
use ryot_compass::{
    config, draw_palette_window, draw_sprite, init_env, load_sprites, CipContent, LmdbEnv, Palette,
    PaletteState, Position, TextureAtlasHandlers, Tile, TilesetCategory,
};
use strum::{EnumCount, IntoEnumIterator};

const CIP_CONTENT_FOLDER: &str = "assets/cip_catalog";
const DECOMPRESSED_CONTENT_FOLDER: &str = "assets/sprite-sheets";

fn build_cip_content_path(file: &String) -> String {
    format!("{}/{}", CIP_CONTENT_FOLDER, file)
}

fn scroll_events(mut minimap: ResMut<Minimap>, mut scroll_evr: EventReader<MouseWheel>) {
    for ev in scroll_evr.read() {
        minimap.zoom += ev.y * 0.1;
        minimap.zoom = minimap.zoom.clamp(1.0, 25.0);
    }
}

fn draw_tiles_on_minimap(
    mut minimap: ResMut<Minimap>,
    mut images: ResMut<Assets<Image>>,
    mut tiles: ResMut<Tiles>,
) {
    let positions = tiles
        .0
        .iter()
        .map(|(tile, _)| UVec2::new(tile.position.x.into(), tile.position.y.into()))
        .collect::<Vec<_>>();
    minimap.update_texture(positions, &mut images);
    tiles.0.clear(); // TODO: replace this with a system that only adds new tiles
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource, Debug)]
pub struct CursorPos(Vec2);

impl Default for CursorPos {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(Vec2::new(0.0, 0.0))
    }
}

#[derive(Resource, Debug)]
pub struct Tiles(Vec<(Tile, bool)>);

impl Default for Tiles {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(vec![])
    }
}

#[derive(Resource, Debug)]
pub struct Counter(u32);
impl Default for Counter {
    fn default() -> Self {
        Self(0)
    }
}

fn load_tiles(env: ResMut<LmdbEnv>, mut tiles: ResMut<Tiles>) {
    let tiles = &mut tiles.0;

    if tiles.len() > 0 {
        return;
    }

    time_test!("Loading");

    let initial_pos = Position::new(60000, 60000, 0);
    let final_pos = Position::new(61100, 61100, 0);

    let item_repository = ItemsFromHeedLmdb::new(env.0.clone().unwrap());

    let lmdb_tiles = {
        time_test!("Reading");
        item_repository
            .get_for_area(&initial_pos, &final_pos)
            .unwrap()
    };

    for tile in lmdb_tiles {
        tiles.push((
            Tile {
                position: Position::from_binary_key(&tile.0),
                item: Some(tile.1),
            },
            false,
        ));
    }
}

fn draw(
    mut commands: Commands,
    // tiles: ResMut<Tiles>,
    mut content: ResMut<CipContent>,
    // mut textures: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut atlas_handlers: ResMut<TextureAtlasHandlers>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut counter: ResMut<Counter>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // cursor_pos: Res<CursorPos>,
    // mut tile_storage_query: Query<(&mut TileStorage, &Transform, Entity)>,
    mut error_states: Res<ErrorState>,
) {
    if error_states.has_error {
        return;
    }
    // let (tile_storage, transform, entity) = tile_storage_query.single_mut();

    let mut sprite_ids = vec![];

    for c in &content.raw_content {
        match c {
            ContentType::Appearances { file, version: _ } => {
                let buffer = get_full_file_buffer(&build_cip_content_path(&file)).unwrap();
                let appearances = Appearances::decode(&*buffer).unwrap();

                for group in vec![
                    &appearances.object,
                    &appearances.outfit,
                    &appearances.missile,
                    &appearances.effect,
                ] {
                    group.iter().for_each(|appearance| {
                        for frame_group in &appearance.frame_group {
                            if let Some(SpriteInfo { sprite_id, .. }) = &frame_group.sprite_info {
                                for id in sprite_id {
                                    sprite_ids.push(id.clone());
                                }
                            }
                        }
                    });
                }
            }
            _ => (),
        }
    }

    sprite_ids = sprite_ids.iter().cloned().unique().collect();

    // Random loading just to test memory consumption
    if counter.0 < sprite_ids.len() as u32 {
        {
            // time_test!("Loading");
            // sprite_ids.chunks(10_000).for_each(|chunk| {
            //     let loaded_sprites = load_sprites(&chunk.to_vec(), &content.raw_content, &asset_server, &mut atlas_handlers, &mut texture_atlases);
            //     info!("Loaded {} sprites", loaded_sprites.len());
            //     counter.0 += loaded_sprites.len() as u32;
            // });
        };
    }

    if counter.0 < 200_000 {
        for x in 0..200 {
            for y in 0..120 {
                let mut sprites = vec![195613];
                if x.ge(&20) && x.le(&30) && y.ge(&20) && y.le(&30) {
                    sprites.push(91267);
                }

                let sprites = load_sprites(
                    &sprites,
                    &content.raw_content,
                    &asset_server,
                    &mut atlas_handlers,
                    &mut texture_atlases,
                );

                for (i, sprite) in sprites.iter().enumerate() {
                    draw_sprite(
                        Vec3::new(x as f32, y as f32, i as f32),
                        sprite,
                        &mut commands,
                    );
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
        counter.0 += 100_000;

        return;
    }
}

fn decompress_all_sprites(content: Res<CipContent>) {
    // time_test!("Decompressing");
    std::fs::create_dir_all(DECOMPRESSED_CONTENT_FOLDER).unwrap();
    decompress_all_sprite_sheets(
        &content.raw_content,
        CIP_CONTENT_FOLDER,
        DECOMPRESSED_CONTENT_FOLDER,
    );
}

// We need to keep the cursor position updated based on any `CursorMoved` events.
pub fn update_cursor_pos(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for cursor_moved in cursor_moved_events.read() {
        // To get the mouse's world position, we have to transform its window position by
        // any transforms on the camera. This is done by projecting the cursor position into
        // camera space (world space).
        for (cam_t, cam) in camera_q.iter() {
            if let Some(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
                *cursor_pos = CursorPos(pos);
            }
        }
    }
}

// fn cursor_pos_to_tile_pos(
//     cursor_pos: Res<CursorPos>,
//     tile_map: (&TilemapSize, &TilemapGridSize, &TilemapType, &Transform),
// ) -> Option<TilePos> {
//     let (map_size, grid_size, map_type, map_transform) = tile_map;
//     let cursor_pos = Vec4::from((cursor_pos.0, 0.0, 1.0));
//     let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
//
//     TilePos::from_world_pos(&cursor_in_map_pos.xy(), map_size, grid_size, map_type)
// }

fn load_cip_content(
    path: &str,
    mut content: ResMut<CipContent>,
    mut error_state: ResMut<ErrorState>,
) {
    match load_content(path) {
        Ok(raw_content) => content.raw_content = raw_content,
        Err(e) => {
            info!("Failed to load CIP content: {:?}", e);
            error_state.has_error = true;
            error_state.error_message = "Failed to load CIP content".to_string();
        }
    }
}

fn ui_example(
    mut egui_ctx: EguiContexts,
    mut content: ResMut<CipContent>,
    mut exit: EventWriter<AppExit>,
    error_state: ResMut<ErrorState>,
    mut about_me: ResMut<AboutMeOpened>,
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

                let is_content_loaded = content.raw_content.len() > 0;

                egui::menu::menu_button(ui, "File", |ui| {
                    if ui
                        .add_enabled(is_content_loaded, egui::Button::new("🗁 Open"))
                        .clicked()
                    {
                        let path = rfd::FileDialog::new()
                            .add_filter(".mdb, .otbm", &["mdb", "otbm"])
                            .pick_file();

                        info!("Loading map from file: {:?}", path);
                        info!("Current dir: {:?}", std::env::current_dir());
                    }

                    if ui
                        .add_enabled(is_content_loaded, egui::Button::new("💾 Save"))
                        .clicked()
                    {
                        let path = rfd::FileDialog::new()
                            .add_filter(".mdb, .otbm", &["mdb", "otbm"])
                            .save_file();

                        info!("Saving map to file: {:?}", path);
                    }

                    ui.separator();

                    if ui.button("Load Content").clicked() {
                        let path = rfd::FileDialog::new()
                            .add_filter(".json", &["json"])
                            .pick_file();

                        let Some(path) = path else {
                            return;
                        };

                        let Some(path) = path.to_str() else {
                            return;
                        };

                        info!("Loading cip content");
                        load_cip_content(path, content, error_state);
                        info!("Content loaded!");
                    }

                    if ui
                        .add_enabled(is_content_loaded, egui::Button::new("🔃 Refresh Content"))
                        .clicked()
                    {
                        if let Ok(_) = std::fs::remove_dir_all(DECOMPRESSED_CONTENT_FOLDER) {
                            // decompress_all_sprites(content);
                        }
                    }

                    ui.separator();

                    if ui.button("⚙ Settings").clicked() {}

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

pub fn print_appearances(
    content: Res<CipContent>,
    mut palette_state: ResMut<PaletteState>,
    asset_server: Res<AssetServer>,
    mut egui_ctx: EguiContexts,
    mut palettes: ResMut<Palette>,
    mut atlas_handlers: ResMut<TextureAtlasHandlers>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut error_states: Res<ErrorState>,
) {
    if error_states.has_error {
        return;
    }

    if palettes
        .tile_set
        .get(&TilesetCategory::Terrains)
        .unwrap()
        .len()
        > 0
    {
        let mut egui_images: Vec<(u32, SheetGrid, egui::Image)> = vec![];

        let mut sprite_ids: Vec<u32> = if palette_state.category == TilesetCategory::Raw {
            let mut sprite_ids = vec![];

            for category_sprites in palettes.tile_set.values() {
                sprite_ids.extend(category_sprites);
            }

            sprite_ids
        } else {
            palettes
                .tile_set
                .get(&palette_state.category)
                .unwrap()
                .to_vec()
        }
        .into_iter()
        .unique()
        .collect();

        sprite_ids.sort();

        for sprite in load_sprites(
            &sprite_ids[palette_state.begin()..palette_state.end()],
            &content.raw_content,
            &asset_server,
            &mut atlas_handlers,
            &mut texture_atlases,
        ) {
            let Some(atlas) = texture_atlases.get(sprite.atlas_texture_handle) else {
                continue;
            };

            let Some(rect) = atlas.textures.get(sprite.sprite_index) else {
                continue;
            };

            let uv: egui::Rect = egui::Rect::from_min_max(
                egui::pos2(rect.min.x / atlas.size.x, rect.min.y / atlas.size.y),
                egui::pos2(rect.max.x / atlas.size.x, rect.max.y / atlas.size.y),
            );

            let rect_vec2: egui::Vec2 =
                egui::Vec2::new(rect.max.x - rect.min.x, rect.max.y - rect.min.y);
            let tex: TextureId = egui_ctx.add_image(atlas.texture.clone_weak());
            egui_images.push((
                sprite.sprite_id,
                sprite.atlas_grid,
                egui::Image::new(SizedTexture::new(tex, rect_vec2)).uv(uv),
            ));
        }

        draw_palette_window(
            sprite_ids.len(),
            palettes.tile_set.keys().sorted().collect_vec(),
            egui_images,
            egui_ctx,
            palette_state,
        );

        return;
    }

    let mut total = 0;

    content
        .raw_content
        .iter()
        .for_each(|content| match content {
            ContentType::Appearances { file, version: _ } => {
                let buffer = get_full_file_buffer(&build_cip_content_path(file)).unwrap();
                let appearances = Appearances::decode(&*buffer).unwrap();
                appearances.outfit.iter().for_each(|outfit| {
                    if let None = outfit.id {
                        warn!("No-id {:?}", outfit);
                    }
                    total += 1;
                });
                appearances.object.iter().for_each(|object| {
                    total += 1;

                    let Some(frame_group) = object.frame_group.first() else {
                        warn!("No-sprite {:?}", object);
                        return;
                    };

                    let Some(sprite_info) = &frame_group.sprite_info else {
                        warn!("No-sprite {:?}", object);
                        return;
                    };

                    let Some(sprite_id) = sprite_info.sprite_id.first() else {
                        warn!("No-sprite {:?}", object);
                        return;
                    };

                    let category: TilesetCategory = object.into();
                    palettes
                        .tile_set
                        .get_mut(&category)
                        .unwrap()
                        .push(*sprite_id);
                });
            }
            _ => (),
        });

    total = 0;

    for (category, ids) in palettes.tile_set.iter_mut() {
        *ids = ids.iter().unique().cloned().collect();
        info!("{}: {}", category.get_label(), ids.len());
        total += ids.len();
    }
    info!("Total: {}", total);
}

fn main() {
    App::new()
        .add_event::<AppExit>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Mouse Position to Tile Position"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_resource::<ErrorState>()
        .init_resource::<LmdbEnv>()
        .init_resource::<Palette>()
        .init_resource::<AboutMeOpened>()
        .init_resource::<TextureAtlasHandlers>()
        .init_resource::<CipContent>()
        .init_resource::<CursorPos>()
        .init_resource::<Tiles>()
        .init_resource::<Counter>()
        .init_resource::<PaletteState>()
        .add_plugins((
            EguiPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            MinimapPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, init_env.before(load_tiles))
        // .add_systems(Startup, load_tiles)
        // .add_systems(Startup, decompress_all_sprites)
        .add_systems(First, (camera_movement, update_cursor_pos).chain())
        // .add_systems(Update, decompress_all_sprites)
        .add_systems(Update, draw)
        .add_systems(Update, draw_tiles_on_minimap)
        .add_systems(Update, scroll_events)
        .add_systems(Update, ui_example)
        .add_systems(Update, print_appearances)
        .add_systems(Update, display_error_window)
        .add_systems(Update, check_for_exit)
        .run();
}

#[derive(Resource, Default)]
struct AboutMeOpened(bool);
