use bevy::math::Vec4Swizzles;
use bevy::{
    input::{
        mouse::MouseWheel,
        common_conditions::input_toggle_active
    },
    ecs::system::Resource,
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;

use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use time_test::time_test;

mod helpers;
use helpers::camera::movement as camera_movement;
use ryot_compass::{init_env, LmdbEnv, Position, Tile};
use ryot_compass::item::{ItemRepository, ItemsFromHeedLmdb};
use ryot_compass::minimap::{Minimap, MinimapPlugin};

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
    mut counter: ResMut<Counter>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    tile_handle_square: Res<TileHandleSquare>,
    cursor_pos: Res<CursorPos>,
    tile_map: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &Transform)>,
    // tile_added_writer: EventWriter<TilesAdded>,
    // mut tile_storage_query: Query<(&mut TileStorage, &Transform, Entity)>,
) {
    // let (tile_storage, transform, entity) = tile_storage_query.single_mut();

    info!("{:?}", cursor_pos_to_tile_pos(cursor_pos, tile_map.single()));

    if counter.0 < 1_100_000 {
        info!("{}", counter.0);
        let tile_handle_square = tile_handle_square.clone();
        let texture_atlas = TextureAtlas::from_grid(
            tile_handle_square,
            Vec2::new(50.0, 50.0),
            1_000,
            1_100,
            None,
            None,
        );

        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        for x in 0..1_100 {
            for y in 0..1_100 {
                if x % 10 != 0 {
                    continue;
                }
                if y % 10 != 0 {
                    continue;
                }
                let x = x * 50;
                let y = y * 50;
                commands.spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(0), // Using the same sprite from the atlas for each instance
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_xyz(x as f32, y as f32, 0.0).with_scale(Vec3::new(10.0, 10.0, 1.0)),
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

fn main() {
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
        .init_resource::<CursorPos>()
        .init_resource::<Tiles>()
        .init_resource::<Counter>()
        .init_resource::<TileHandleSquare>()
        .add_plugins(TilemapPlugin)
        .add_plugins((
            EguiPlugin,
            // WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
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
        .add_systems(First, (camera_movement, update_cursor_pos).chain())
        .add_systems(Update, add_tile)
        .add_systems(Update, draw)
        .add_systems(Update, draw_tiles_on_minimap)
        .add_systems(Update, scroll_events)
        .run();
}