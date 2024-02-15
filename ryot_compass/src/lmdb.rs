use crate::build_map;
use crate::item::ItemRepository;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::{
    Camera, Changed, Commands, Deref, DerefMut, IVec2, Local, Query, Res, ResMut, Resource, With,
};
use heed::Env;
use log::error;
use ryot::bevy_ryot::drawing::UpdateTileContent;
use ryot::bevy_ryot::map::MapTiles;
use ryot::lmdb;
use ryot::position::{Sector, TilePosition};
use ryot::prelude::drawing::DrawingBundle;
use ryot::prelude::{compress, decompress, AppearanceDescriptor, Zstd};
use time_test::time_test;

#[derive(Resource, Deref, DerefMut)]
pub struct LmdbEnv(pub Env);

impl Default for LmdbEnv {
    fn default() -> Self {
        Self(lmdb::create_env(lmdb::get_storage_path()).expect("Failed to create LMDB env"))
    }
}

pub fn read_area(
    tiles: Res<MapTiles>,
    env: ResMut<LmdbEnv>,
    mut commands: Commands,
    mut last_area: Local<Sector>,
    sector_query: Query<&Sector, (With<Camera>, Changed<Sector>)>,
) {
    let Ok(sector) = sector_query.get_single() else {
        return;
    };

    let size = sector.size() / IVec2::new(2, 2);
    let min = TilePosition::new(sector.min.x - size.x, sector.min.y - size.y, 0);
    let max = TilePosition::new(sector.max.x + size.x, sector.max.y + size.y, 0);

    let new_area = Sector::new(min, max);

    for area in last_area.diff(&new_area) {
        load_area(area, env.clone(), &mut commands, &tiles);
    }

    *last_area = new_area;
}

pub fn load_area(sector: Sector, env: Env, commands: &mut Commands, tiles: &Res<MapTiles>) {
    time_test!("Reading");
    let item_repository = crate::item::ItemsFromHeedLmdb::new(env);

    match item_repository.get_for_area(&sector) {
        Ok(area) => {
            for tile in area {
                for (layer, item) in tile.items {
                    if let Some(tile) = tiles.get(&tile.position) {
                        if tile.get(&layer).is_some() {
                            continue;
                        }
                    }

                    let bundle = Some(DrawingBundle::new(
                        layer,
                        tile.position,
                        AppearanceDescriptor::object(item.id as u32),
                    ));

                    let entity = commands.spawn_empty().id();

                    commands.add(UpdateTileContent(bundle, None).with_entity(entity));
                }
            }
        }
        Err(e) => {
            error!("Failed to read area: {}", e);
        }
    }
}

pub fn lmdb_example() -> Result<(), Box<dyn std::error::Error>> {
    let env = lmdb::create_env(lmdb::get_storage_path())?;
    let item_repository = crate::item::ItemsFromHeedLmdb::new(env.clone());
    let z_size = 1;

    let map = {
        time_test!("Building ryot_compass");
        build_map(z_size)
    };

    {
        time_test!("Writing");
        item_repository.save_from_tiles(map.tiles)?;
    }

    let initial_pos = TilePosition::new(-550, -550, 0);
    let final_pos = TilePosition::new(550, 550, z_size - 1);

    {
        time_test!("Reading");
        let area = item_repository.get_for_area(&Sector::new(initial_pos, final_pos))?;
        println!("Count: {}", area.len());
    }

    // env.prepare_for_closing().wait();
    // lmdb::compact()?;

    {
        time_test!("Compressing");
        compress::<Zstd>(
            lmdb::get_storage_path().join("data.mdb").to_str().unwrap(),
            Some(3),
        )?;
    }

    {
        time_test!("Decompressing");
        decompress::<Zstd>(
            lmdb::get_storage_path()
                .join("data.mdb.snp")
                .to_str()
                .unwrap(),
        )?;
    }

    Ok(())
}
