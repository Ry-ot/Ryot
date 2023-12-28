use std::io::{Read, Seek};
use bevy::math::Vec4Swizzles;
use bevy::{
    input::{
        mouse::MouseWheel,
        common_conditions::input_toggle_active
    },
    ecs::system::Resource,
    prelude::*,
};
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy_ecs_tilemap::prelude::*;

use bevy_egui::{EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use image::{Pixel, RgbaImage};
use image::imageops::crop;
use prost::Message;
use rand::Rng;
use time_test::time_test;

mod helpers;
use helpers::camera::movement as camera_movement;
use ryot_compass::{init_env, LmdbEnv, Position, Tile};
use ryot_compass::item::{ItemRepository, ItemsFromHeedLmdb};
use ryot_compass::minimap::{Minimap, MinimapPlugin};
use rayon::prelude::*;
use ryot::cip_content::{AppearanceFlags, Appearances, ContentType, get_full_file_buffer, get_sheet_by_sprite_id, load_content, load_sprite_sheet_for_content, SPRITE_SHEET_SIZE, SpriteLayout};

const MAP_SIDE_LENGTH_X: u32 = 0;
const MAP_SIDE_LENGTH_Y: u32 = 0;

const TILE_SIZE_SQUARE: TilemapTileSize = TilemapTileSize { x: 50.0, y: 50.0 };
const GRID_SIZE_SQUARE: TilemapGridSize = TilemapGridSize { x: 50.0, y: 50.0 };

#[derive(Deref, Resource)]
pub struct TileHandleSquare(Handle<Image>);

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
    content: Res<CipContent>,
    mut textures: ResMut<Assets<Image>>,
    mut counter: ResMut<Counter>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // tile_handle_square: Res<TileHandleSquare>,
    cursor_pos: Res<CursorPos>,
    tile_map: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &Transform)>,
    // tile_added_writer: EventWriter<TilesAdded>,
    // mut tile_storage_query: Query<(&mut TileStorage, &Transform, Entity)>,
) {
    // let (tile_storage, transform, entity) = tile_storage_query.single_mut();

    info!("{:?}", cursor_pos_to_tile_pos(cursor_pos, tile_map.single()));

    if counter.0 < 1_100_000 {
        info!("{}", counter.0);

        let monster = get_sprite_image(91267, &content).unwrap();
        let tile = get_sprite_image(195613, &content).unwrap();
        let monster_img = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: monster.width(),
                    height: monster.height(),
                    depth_or_array_layers: 1,
                },
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT,
                mip_level_count: 1,
                sample_count: 1,
                view_formats: &[],
            },
            data: monster.clone().into_raw(),
            ..Default::default()
        };
        let tile_img = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: tile.width(),
                    height: tile.height(),
                    depth_or_array_layers: 1,
                },
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT,
                mip_level_count: 1,
                sample_count: 1,
                view_formats: &[],
            },
            data: tile.clone().into_raw(),
            ..Default::default()
        };

        let monster_handle = textures.add(monster_img);
        let tile_handle = textures.add(tile_img);

        for x in 0..200 {
            for y in 0..120 {
                commands.spawn(SpriteBundle {
                    texture: tile_handle.clone(),
                    transform: Transform::from_xyz((x * tile.width()) as f32, (y * tile.height()) as f32, 0.0),
                    ..Default::default()
                });
            }
        }

        for x in 20..30 {
            for y in 20..30 {
                commands.spawn(SpriteBundle {
                    texture: monster_handle.clone(),
                    transform: Transform::from_xyz((x * monster.width()) as f32, (y * monster.height()) as f32, 0.0),
                    ..Default::default()
                });
            }
        }

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
        counter.0 += 1_000_000;

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

#[derive(Resource, Debug)]
pub struct CipContent(Vec<ContentType>);

impl Default for CipContent {
    fn default() -> Self {
        Self(vec![])
    }
}

pub fn load_cip_content(
    mut content: ResMut<CipContent>
) {
    info!("Loading CIP Content");
    content.0 = load_content("ryot_compass/assets/assets/catalog-content.json").expect("Failed to load CIP content");
    info!("CIP Content loaded: {}", content.0.len());
}

pub fn print_appearances(
    content: Res<CipContent>,
) {
    content.0.iter().for_each(|content| {
        match content {
            ContentType::Appearances { file, version: _ } => {
                let buffer = get_full_file_buffer(&format!("ryot_compass/assets/assets/{}", file)).unwrap();
                let appearances =  Appearances::decode(&*buffer).unwrap();
                appearances.object.iter().for_each(|object| {
                    if let Some(AppearanceFlags{bank, market,  ..}) = &object.flags {
                        if bank.is_some() {
                            let bank = bank.clone().unwrap();
                            info!("Object: {:?}, {:?}, {:?}", object.id, object.frame_group, bank);
                        }

                        if market.is_some() {
                            let market = market.clone().unwrap();
                            info!("Object: {:?}, {:?}, {:?}", object.id, object.frame_group, market);
                        }
                    }
                });
                // info!("Appearances: {:?}", file)
            },
            _ => (),
        }
    });
}

pub fn get_sprite_image(
    id: u32,
    content: &Res<CipContent>,
) -> Option<RgbaImage> {
    if let Some(ContentType::Sprite {first_sprite_id, layout, file, ..}) = get_sheet_by_sprite_id(&content.0, id) {
        let sprite_offset = id - first_sprite_id;

        let width = match layout {
            SpriteLayout::OneByOne | SpriteLayout::OneByTwo => 32,
            SpriteLayout::TwoByOne | SpriteLayout::TwoByTwo => 64,
        };

        let colums = SPRITE_SHEET_SIZE / width;

        let row = ((sprite_offset as f32) / (colums as f32)).floor() as u32;
        let column = sprite_offset % colums;

        let mut sheet = load_sprite_sheet_for_content(&file, "ryot_compass/assets/assets/").expect("Failed to load sprite sheet");

        let sprite_height = match layout {
            SpriteLayout::OneByOne | SpriteLayout::TwoByOne => 32,
            SpriteLayout::OneByTwo | SpriteLayout::TwoByTwo => 64,
        };
        let x = column * width;
        let y = row * sprite_height;

        // Create a view into the image at the calculated coordinates
        return Some(crop(&mut sheet, x, y, width, sprite_height).to_image());
    }

    None
}

fn main() {
    // println!("Sprites: {:?}", get_all_sprite_sheets(&content).len(), "ryot_compass/assets/assets/");

    App::new()
        .add_event::<TilesAdded>()
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
        .init_resource::<LmdbEnv>()
        .init_resource::<CipContent>()
        .init_resource::<CursorPos>()
        .init_resource::<Tiles>()
        .init_resource::<Counter>()
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
        .add_systems(Startup, load_tiles)
        .add_systems(Startup, load_cip_content)
        .add_systems(First, (camera_movement, update_cursor_pos).chain())
        .add_systems(Update, add_tile)
        .add_systems(Update, draw)
        .add_systems(Update, draw_tiles_on_minimap)
        .add_systems(Update, scroll_events)
        .add_systems(Update, print_appearances)
        .run();
}