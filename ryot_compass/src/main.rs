use std::cmp::Ordering;
use std::io::{Read, Seek};
use std::ops::Range;
use bevy::app::AppExit;
use bevy::math::Vec4Swizzles;
use bevy::{
    input::{
        mouse::MouseWheel,
        common_conditions::input_toggle_active
    },
    ecs::system::Resource,
    prelude::*,
};
use bevy::asset::LoadedFolder;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::prelude::*;

use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use egui::{Align, TextureId, WidgetText};
use egui::load::SizedTexture;
use image::Pixel;
use itertools::Itertools;
use prost::Message;
use rand::prelude::{IteratorRandom, SliceRandom};
use rand::Rng;
use time_test::time_test;

mod helpers;
mod error_handling;
use helpers::camera::movement as camera_movement;
use ryot_compass::{CipContent, draw_sprite, init_env, LmdbEnv, load_sprites, Position, TextureAtlasHandlers, Tile};
use ryot_compass::item::{ItemRepository, ItemsFromHeedLmdb};
use ryot_compass::minimap::{Minimap, MinimapPlugin};
use rayon::prelude::*;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};
use ryot::cip_content::{Appearance, AppearanceFlags, Appearances, ContentType, decompress_all_sprite_sheets, get_full_file_buffer, ItemCategory, load_content, SheetGrid, SpriteInfo};
use error_handling::{ErrorState, display_error_window, check_for_exit};

const MAP_SIDE_LENGTH_X: u32 = 0;
const MAP_SIDE_LENGTH_Y: u32 = 0;
const CIP_CONTENT_FOLDER: &str = "ryot_compass/assets/cip_catalog";
const DECOMPRESSED_CONTENT_FOLDER: &str = "ryot_compass/assets/sprite-sheets";

const TILE_SIZE_SQUARE: TilemapTileSize = TilemapTileSize { x: 50.0, y: 50.0 };
const GRID_SIZE_SQUARE: TilemapGridSize = TilemapGridSize { x: 50.0, y: 50.0 };

#[derive(Deref, Resource)]
pub struct TileHandleSquare(Handle<Image>);

impl From<&AppearanceFlags> for TilesetCategory {
    fn from(flags: &AppearanceFlags) -> Self {
        // Market has categories, so we can use it to determine the category of the item.
        // If the item has a market flag, it's category is prioritized over the other flags.
        if let Some(market) = &flags.market {
            if let Some(category) = market.category {
                return (&ItemCategory::try_from(category).unwrap()).into();
            }
        }

        if flags.bank.is_some() {
            return TilesetCategory::Terrains;
        }

        if flags.clip.is_some() {
            return TilesetCategory::Edges;
        }

        if flags.bottom.is_some() {
            return TilesetCategory::BaseLayer;
        }

        if flags.top.is_some() {
            return TilesetCategory::UpperLayer;
        }

        // Corpses are also containers, so they need to be checked first
        if flags.corpse.is_some() || flags.player_corpse.is_some() {
            return TilesetCategory::Corpses;
        }

        if flags.container.is_some() {
            return TilesetCategory::Containers;
        }

        if flags.hang.is_some() || flags.hook.is_some() || flags.rotate.is_some() {
            return TilesetCategory::Decor;
        }

        if flags.clothes.is_some() {
            return TilesetCategory::Clothes;
        }

        TilesetCategory::Miscellaneous
    }
}

impl From<&ItemCategory> for TilesetCategory {
    fn from(category: &ItemCategory) -> Self {
        match category {
            ItemCategory::Ammunition
            | ItemCategory::Axes
            | ItemCategory::Clubs
            | ItemCategory::DistanceWeapons
            | ItemCategory::Shields
            | ItemCategory::Quiver
            | ItemCategory::Swords
            | ItemCategory::WandsRods => TilesetCategory::Weapons,
            ItemCategory::Armors
            | ItemCategory::Amulets
            | ItemCategory::Boots
            | ItemCategory::HelmetsHats
            | ItemCategory::Legs
            | ItemCategory::Rings => TilesetCategory::Clothes,
            ItemCategory::CreatureProducts => TilesetCategory::CreatureProducts,
            ItemCategory::Containers => TilesetCategory::Containers,
            ItemCategory::Decoration => TilesetCategory::Decor,
            ItemCategory::Food
            | ItemCategory::Potions
            | ItemCategory::Runes => TilesetCategory::Consumables,
            ItemCategory::PremiumScrolls
            | ItemCategory::TibiaCoins
            | ItemCategory::Valuables  => TilesetCategory::Valuables,
            ItemCategory::Others => TilesetCategory::Miscellaneous,
            ItemCategory::Tools => TilesetCategory::Tools,
        }
    }
}

impl From<&Appearance> for TilesetCategory {
    fn from(appearance: &Appearance) -> Self {
        if let Some(flags) = &appearance.flags {
            return flags.into();
        }

        TilesetCategory::Miscellaneous
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumCount)]
pub enum TilesetCategory {
    Terrains,
    Edges,
    BaseLayer,
    UpperLayer,
    Decor,
    Corpses,
    Containers,
    Clothes,
    Consumables,
    Tools,
    Miscellaneous,
    Valuables,
    CreatureProducts,
    Weapons,
    Raw,
}

impl PartialOrd<Self> for TilesetCategory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_label().partial_cmp(&other.get_label())
    }
}

impl Ord for TilesetCategory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_label().cmp(&other.get_label())
    }
}

impl Into<WidgetText> for &TilesetCategory {
    fn into(self) -> WidgetText {
        WidgetText::from(self.get_label())
    }
}

impl TilesetCategory {
    pub fn get_label(&self) -> String {
        match self {
            TilesetCategory::Terrains => String::from("Terrains"),
            TilesetCategory::Edges => String::from("Edges"),
            TilesetCategory::BaseLayer => String::from("Base Layer"),
            TilesetCategory::UpperLayer => String::from("Upper Layer"),
            TilesetCategory::Decor => String::from("Decor"),
            TilesetCategory::Corpses => String::from("Corpses"),
            TilesetCategory::Containers => String::from("Containers"),
            TilesetCategory::Clothes => String::from("Clothes"),
            TilesetCategory::Consumables => String::from("Consumables"),
            TilesetCategory::Tools => String::from("Tools"),
            TilesetCategory::Miscellaneous => String::from("Miscellaneous"),
            TilesetCategory::Valuables => String::from("Valuables"),
            TilesetCategory::CreatureProducts => String::from("Creature Products"),
            TilesetCategory::Weapons => String::from("Weapons"),
            TilesetCategory::Raw => String::from("Raw"),
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub struct Palette {
    pub tile_set: HashMap<TilesetCategory, Vec<u32>>,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            tile_set: [
                (TilesetCategory::Terrains, vec![]),
                (TilesetCategory::Edges, vec![]),
                (TilesetCategory::BaseLayer, vec![]),
                (TilesetCategory::UpperLayer, vec![]),
                (TilesetCategory::Decor, vec![]),
                (TilesetCategory::Corpses, vec![]),
                (TilesetCategory::Containers, vec![]),
                (TilesetCategory::Clothes, vec![]),
                (TilesetCategory::Consumables, vec![]),
                (TilesetCategory::Tools, vec![]),
                (TilesetCategory::Miscellaneous, vec![]),
                (TilesetCategory::Valuables, vec![]),
                (TilesetCategory::CreatureProducts, vec![]),
                (TilesetCategory::Weapons, vec![]),
                (TilesetCategory::Raw, vec![]),
            ].into(),
        }
    }

}

fn build_cip_content_path(file: &String) -> String {
    format!("{}/{}", CIP_CONTENT_FOLDER, file)
}

impl FromWorld for TileHandleSquare {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("bw-tile-square.png"))
    }
}

fn scroll_events(
    mut minimap: ResMut<Minimap>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
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
    let positions = tiles.0.iter().map(|(tile, _)| {
        UVec2::new(tile.position.x.into(), tile.position.y.into())
    }).collect::<Vec<_>>();
    minimap.update_texture(positions, &mut images);
    tiles.0.clear(); // TODO: replace this with a system that only adds new tiles
}

// Generates the initial tilemap, which is a square grid.
fn spawn_tilemap(mut commands: Commands, tile_handle_square: Res<TileHandleSquare>) {
    commands.spawn(Camera2dBundle::default());

    let map_size = TilemapSize {
        x: MAP_SIDE_LENGTH_X,
        y: MAP_SIDE_LENGTH_Y,
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    fill_tilemap(
        TileTextureIndex(0),
        TilemapSize {
            x: 0,
            y: 0,
        },
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TILE_SIZE_SQUARE;
    let grid_size = GRID_SIZE_SQUARE;
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_handle_square.clone()),
        tile_size,
        map_type,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
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

#[derive(Resource, Debug)]
pub struct PaletteState {
    pub category: TilesetCategory,
    pub grid_size: u32,
    pub visible_rows: Range<usize>,
}

impl Default for PaletteState {
    fn default() -> Self {
        Self {
            category: TilesetCategory::Terrains,
            grid_size: 64,
            visible_rows: Range {start: 0, end: 10},
        }
    }
}

#[derive(Event, Debug)]
struct TilesAdded(TileTextureIndex, TilePos, TilemapSize, TilemapId);

fn load_tiles(
    env: ResMut<LmdbEnv>,
    mut tiles: ResMut<Tiles>,
) {
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
        item_repository.get_for_area(&initial_pos, &final_pos).unwrap()
    };

    for tile in lmdb_tiles {
        tiles.push((Tile{ position: Position::from_binary_key(&tile.0), item: Some(tile.1) }, false));
    }
}

fn draw(
    mut commands: Commands,
    // mut env: ResMut<LmdbEnv>,
    // tiles: ResMut<Tiles>,
    mut content: ResMut<CipContent>,
    mut textures: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut atlas_handlers: ResMut<TextureAtlasHandlers>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut counter: ResMut<Counter>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // tile_handle_square: Res<TileHandleSquare>,
    // cursor_pos: Res<CursorPos>,
    // tile_map: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &Transform)>,
    // tile_added_writer: EventWriter<TilesAdded>,
    // mut tile_storage_query: Query<(&mut TileStorage, &Transform, Entity)>,
    mut error_states: Res<ErrorState>,
) {
    if error_states.has_error {
        return;
    }
    // let (tile_storage, transform, entity) = tile_storage_query.single_mut();

    // info!("{:?}", cursor_pos_to_tile_pos(cursor_pos, tile_map.single()));
    let mut sprite_ids = vec![];

    for c in &content.raw_content {
        match c {
            ContentType::Appearances { file, version: _ } => {
                let buffer = get_full_file_buffer(&build_cip_content_path(&file)).unwrap();
                let appearances =  Appearances::decode(&*buffer).unwrap();

                for group in vec![&appearances.object, &appearances.outfit, &appearances.missile, &appearances.effect] {
                    group.iter().for_each(|appearance| {
                        for frame_group in &appearance.frame_group {
                            if let Some(SpriteInfo{sprite_id, ..}) = &frame_group.sprite_info {
                                for id in sprite_id {
                                    sprite_ids.push(id.clone());
                                }
                            }
                        }
                    });
                }
            },
            _ => (),
        }
    }

    sprite_ids = sprite_ids.iter().cloned().unique().collect();

    info!("{}", counter.0);
    info!("Textures: {}", textures.len());

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

                let sprites = load_sprites(&sprites, &content.raw_content, &asset_server, &mut atlas_handlers, &mut texture_atlases);

                for (i, sprite) in sprites.iter().enumerate() {
                    draw_sprite(Vec3::new(x as f32, y as f32, i as f32), sprite, &mut commands);
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

    return;

    // if buttons.just_pressed(MouseButton::Right) {
    //     for x in 0..=1100 {
    //         for y in 0..=1100 {
    //             let remove_pos = TilePos{x, y};
    //             if let Some(tile_entity) = tile_storage.get(&remove_pos) {
    //                 commands.entity(tile_entity).despawn_recursive();
    //                 tile_storage.remove(&remove_pos);
    //             }
    //         }
    //     }
    //     // tiles.clear();
    //     return;
    // }

    // let mut to_load = 500;
    // for tile in tiles {
    //     let (tile, bool) = tile;
    //
    //     if *bool {
    //         continue;
    //     }
    //
    //     if to_load == 0 {
    //         break;
    //     }
    //
    //     let pos = tile.position;
    //     let tile_pos = TilePos::new((pos.x - 60000u16) as u32, (pos.y - 60000u16) as u32);
    //
    //     tile_added_writer.send(TilesAdded(
    //         TileTextureIndex(0),
    //         tile_pos,
    //         TilemapSize { x: 1, y: 1 },
    //         TilemapId(entity),
    //     ));
    //
    //     *bool = true;
    //     to_load -= 1;
    // }
}

fn decompress_all_sprites(
    content: Res<CipContent>,
) {
    time_test!("Decompressing");
    std::fs::create_dir_all(DECOMPRESSED_CONTENT_FOLDER).unwrap();
    decompress_all_sprite_sheets(
        &content.raw_content,
        CIP_CONTENT_FOLDER,
        DECOMPRESSED_CONTENT_FOLDER,
    );
}

fn add_tile(
    mut commands: Commands,
    mut tile_added_reader: EventReader<TilesAdded>,
    mut tile_storage_query: Query<&mut TileStorage>,
) {
    let mut max_events = 500;
    // info!("Adding tiles");
    for TilesAdded(tile_texture_index, tile_pos, tilemap_size, tilemap_id) in tile_added_reader.read() {
        if max_events == 0 {
            break;
        }

        fill_tilemap_rect(
            *tile_texture_index,
            *tile_pos,
            *tilemap_size,
            *tilemap_id,
            &mut commands,
            &mut tile_storage_query.get_mut(tilemap_id.0).expect("Tilemap not found"),
        );

        max_events -= 1;
    }
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

fn cursor_pos_to_tile_pos(
    cursor_pos: Res<CursorPos>,
    tile_map: (&TilemapSize, &TilemapGridSize, &TilemapType, &Transform),
) -> Option<TilePos> {
    let (map_size, grid_size, map_type, map_transform) = tile_map;
    let cursor_pos = Vec4::from((cursor_pos.0, 0.0, 1.0));
    let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;

    TilePos::from_world_pos(&cursor_in_map_pos.xy(), map_size, grid_size, map_type)
}

#[derive(SystemSet, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct SpawnTilemapSet;

fn load_cip_content(
    mut content: ResMut<CipContent>,
    mut error_state: ResMut<ErrorState>,
) {
    match load_content(&build_cip_content_path(&String::from("catalog-content.json"))) {
        Ok(raw_content) => content.raw_content = raw_content,
        Err(_) => {
            error_state.has_error = true;
            error_state.error_message = "Failed to load CIP content".to_string();
        },
    }
}

fn ui_example(mut egui_ctx: EguiContexts) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(32.0); // Adjust the height as needed
        ui.horizontal(|ui| {
            if ui.button("🏠").clicked() {
                let path = rfd::FileDialog::new().pick_folder();
                println!("Selected file: {:?}", path);
            }
        });
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

    if palettes.tile_set.get(&TilesetCategory::Terrains).unwrap().len() > 0 {
        let mut selected = palette_state.category;

        let mut egui_images: Vec<(u32, SheetGrid, egui::Image)> = vec![];

        let mut sprite_ids = if selected == TilesetCategory::Raw {
            let mut sprite_ids = vec![];

            for category_sprites in palettes.tile_set.values() {
                sprite_ids.extend(category_sprites);
            }

            sprite_ids
        } else {
            palettes.tile_set.get(&selected).unwrap().to_vec()
        };

        let chunk_size = ((320 / palette_state.grid_size) as usize).clamp(4, 9);
        let total_rows = sprite_ids.len() / chunk_size;

        sprite_ids.sort();
        let begin = palette_state.visible_rows.start * chunk_size;
        let end = palette_state.visible_rows.end * chunk_size;

        for sprite in load_sprites(&sprite_ids[begin..=end], &content.raw_content, &asset_server, &mut atlas_handlers, &mut texture_atlases) {
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

            let rect_vec2: egui::Vec2 = egui::Vec2::new(rect.max.x - rect.min.x, rect.max.y - rect.min.y);
            let tex: TextureId = egui_ctx.add_image(atlas.texture.clone_weak());
            egui_images.push((sprite.sprite_id, sprite.atlas_grid, egui::Image::new(SizedTexture::new(tex, rect_vec2)).uv(uv)));
        }

        egui::Window::new("Palette")
            .min_width(350.)
            .max_width(350.)
            .show(egui_ctx.ctx_mut(), |ui| {

                // Add bottom panel for zoom controls
                egui::TopBottomPanel::bottom("bottom_panel").show_inside(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.add_space(5.0); // Add some space from the top border
                        ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                            if ui.button("+").clicked() {
                                palette_state.grid_size += 16;
                            }

                            if ui.button("-").clicked() {
                                palette_state.grid_size -= 16;
                            }

                            palette_state.grid_size = palette_state.grid_size.clamp(32, 80);
                        });
                    });
                });

                egui::ComboBox::from_id_source("palette")
                    .selected_text(selected.get_label().clone())
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        for key in palettes.tile_set.keys().sorted() {
                            if ui.selectable_value(&mut selected.get_label(), key.get_label().clone(), key).clicked() {
                                palette_state.category = key.clone();

                                info!("Selected: {:?}", palette_state.category);
                            }
                        }
                    });

                let row_padding = 3.;
                let row_height = palette_state.grid_size as f32 + row_padding;

                egui::ScrollArea::vertical()
                    .max_height(ui.available_height())
                    .show_rows(ui, row_height, total_rows, |ui, row_range| {
                        ui.set_width(ui.available_width());
                        palette_state.visible_rows = row_range.clone();
                        info!("row_range: {:?}", row_range);
                        egui_images.chunks(chunk_size).for_each(|chunk| {
                            ui.horizontal(|ui| {
                                chunk.iter().enumerate().for_each(|(i, (index, grid, image))| {
                                    let size = palette_state.grid_size as f32;

                                    ui.vertical(|ui| {
                                        ui.add(image.clone().fit_to_exact_size(egui::Vec2::new(size, size)));
                                        ui.add_space(3.);
                                    });

                                    let ratio = grid.tile_size.height as f32 / grid.tile_size.width as f32;

                                    if ratio > 1.0 && i < chunk_size - 1 {
                                        ui.add_space(size / 2.);
                                    }
                                });
                            });
                        });
                        // for row in row_range {
                        //     ui.horizontal(|ui| {
                        //         egui_images[row * chunk_size..(row + 1) * chunk_size].iter().for_each(|(index, grid, image)| {
                        //             let size = palette_state.grid_size as f32;
                        //
                        //             ui.vertical(|ui| {
                        //                 ui.add(image.clone().fit_to_exact_size(egui::Vec2::new(size, size)));
                        //                 ui.add_space(row_padding);
                        //             });
                        //
                        //             let ratio = grid.tile_size.height as f32 / grid.tile_size.width as f32;
                        //
                        //             if ratio > 1.0 && row < chunk_size - 1 {
                        //                 ui.add_space(size / 2.);
                        //             }
                        //         });
                        //     });
                        // }
                });
            });
        return;
    }


    let mut total = 0;

    content.raw_content.iter().for_each(|content| {
        match content {
            ContentType::Appearances { file, version: _ } => {
                let buffer = get_full_file_buffer(&build_cip_content_path(file)).unwrap();
                let appearances =  Appearances::decode(&*buffer).unwrap();
                info!("outfits: {}", appearances.outfit.len());
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
                    palettes.tile_set.get_mut(&category).unwrap().push(*sprite_id);
                    // if let Some(AppearanceFlags{bank, market,  ..}) = &object.flags {
                    //     if bank.is_some() {
                    //         let _ = bank.clone().unwrap();
                    //         // info!("Object: {:?}, {:?}, {:?}", object.id, object.frame_group, bank);
                    //     }
                    //
                    //     if market.is_some() {
                    //         let _ = market.clone().unwrap();
                    //         // info!("Object: {:?}, {:?}, {:?}", object.id, object.frame_group, market);
                    //     }
                    // }
                });
                // info!("Appearances: {:?}", file)
            },
            _ => (),
        }
    });

    for (category, ids) in &palettes.tile_set {
        info!("{}: {}", category.get_label(), ids.len());
    }
    info!("Total: {}", total);
}

fn main() {
    App::new()
        .add_event::<AppExit>()
        .add_event::<TilesAdded>()
        .add_state::<AppState>()
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
        .init_resource::<SpriteSheetFolder>()
        .init_resource::<TextureAtlasHandlers>()
        .init_resource::<CipContent>()
        .init_resource::<CursorPos>()
        .init_resource::<Tiles>()
        .init_resource::<Counter>()
        .init_resource::<PaletteState>()
        .init_resource::<TileHandleSquare>()
        .add_plugins(TilemapPlugin)
        .add_plugins((
            EguiPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            MinimapPlugin,
        ))
        .add_systems(
            Startup,
            (spawn_tilemap, apply_deferred)
                .chain()
                .in_set(SpawnTilemapSet),
        )
        .add_systems(Startup, init_env.before(load_tiles))
        // .add_systems(Startup, load_tiles)
        .add_systems(Startup, load_cip_content.before(decompress_all_sprites))
        .add_systems(Startup, decompress_all_sprites)
        .add_systems(First, (camera_movement, update_cursor_pos).chain())
        .add_systems(Update, add_tile)
        // .add_systems(Update, draw)
        .add_systems(Update, draw_tiles_on_minimap)
        .add_systems(Update, scroll_events)
        .add_systems(Update, print_appearances)
        .add_systems(Update, ui_example)
        .add_systems(Update, display_error_window)
        .add_systems(Update, check_for_exit)
        .run();
}

#[derive(Resource, Default)]
struct SpriteSheetFolder(Handle<LoadedFolder>);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}