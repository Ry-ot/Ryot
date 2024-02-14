use crate::item::ItemRepository;
use crate::{build_map, create_and_send_update_command, Cursor, GetKey};
use bevy::prelude::{
    Camera, Changed, Commands, Deref, DerefMut, IVec2, Query, ResMut, Resource, Visibility, With,
    Without,
};
use heed::Env;
use ryot::bevy_ryot::map::MapTiles;
use ryot::lmdb;
use ryot::position::{Edges, TilePosition};
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
    env: ResMut<LmdbEnv>,
    mut commands: Commands,
    mut tiles: ResMut<MapTiles>,
    edges_query: Query<&Edges, (With<Camera>, Changed<Edges>)>,
    current_appearance_query: Query<(&mut AppearanceDescriptor, &Visibility), Without<Cursor>>,
) {
    let Ok(edges) = edges_query.get_single() else {
        return;
    };

    let size = edges.size() / IVec2::new(2, 2);
    let min = TilePosition::new(edges.min.x - size.x, edges.min.y - size.y, 0);
    let max = TilePosition::new(edges.max.x + size.x, edges.max.y + size.y, 0);

    time_test!("Reading");
    let item_repository = crate::item::ItemsFromHeedLmdb::new(env.clone());
    let area = item_repository.get_for_area(&min, &max).unwrap();

    for (key, item) in area {
        let tile_pos = TilePosition::from_binary_key(&key);
        for (layer, item) in item {
            let bundle = DrawingBundle::new(
                layer,
                tile_pos,
                AppearanceDescriptor::object(item.id as u32),
            );

            create_and_send_update_command(
                layer,
                bundle,
                &mut commands,
                &mut tiles,
                &current_appearance_query,
            );
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
        let area = item_repository.get_for_area(&initial_pos, &final_pos)?;
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
