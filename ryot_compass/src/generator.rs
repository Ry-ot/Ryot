/*
"kv storage" MapName
header -> {
  width: u16,
  height: u16,
  floors: u8,
  version: u8,
  description: string,
}

x:y:z:tile -> {
    uid: int,
    tile_id: u16,
    flags: u32,
}

parent_id:item -> {
    uid: int
    item_id: u16,
    count: u8,
    charges: u8, // maybe could merge with count? same idea of stacking
    depot_id: u16,
    text: String,
}

parent_id:house -> {
    id: u32,
    door_id: u8,
}

item_id:action -> {
    action_id: u16,
    unique_id: u16,
    teleport: Position,
}

wp:name -> {
    name: String,
    position: Position,
}

town:name -> {
    id: u8,
    name: String,
    position: Position,
}

x:y:z:spawn -> {
    radius: u8,
    spawn_time: u16,
}

spawn_id:ncp:id -> {
    name: String,
    x: u8,
    y: u8,
    z: u8,
}

spawn_id:monster:id -> {
    name: String,
    x: u8,
    y: u8,
    z: u8,
}

zone:id -> {
    name: String,
}

house:id -> {
    id: u32,
    name: String,
    position: Position,
    rent: u32,
    guild_hall: bool,
    town_id: u8,
    size: u16,
    beds: u8,
}
 */
use crate::{Item, ItemAttribute, MapComponent, Plan, Tile};
use rand::Rng;
use ryot::layer::CipLayer;
use ryot::position::{Edges, TilePosition};

pub fn build_map(z_size: i32) -> Plan {
    let mut map = Plan::default();

    for x in -550..550 {
        for y in -550..550 {
            for z in 0..z_size {
                let mut tile = Tile::from_pos(TilePosition::new(x, y, z));
                tile.set_item(
                    Item {
                        id: rand::thread_rng().gen_range(1261..=1269),
                        attributes: get_attribute_array(),
                    },
                    CipLayer::Ground.into(),
                );

                tile.set_item(
                    Item {
                        id: rand::thread_rng().gen_range(660..=810),
                        attributes: get_attribute_array(),
                    },
                    CipLayer::Items.into(),
                );

                tile.set_item(
                    Item {
                        id: rand::thread_rng().gen_range(2012..=2017),
                        attributes: get_attribute_array(),
                    },
                    CipLayer::Bottom.into(),
                );

                map.add(MapComponent::Tile(tile));
            }
        }
    }

    map
}

pub fn get_attribute_array() -> Vec<ItemAttribute> {
    let chance = rand::thread_rng().gen_range(0..=100);
    if chance < 1 {
        return vec![ItemAttribute::Count(rand::thread_rng().gen_range(0..=5))];
    }

    if chance < 2 {
        return vec![ItemAttribute::HouseId(
            rand::thread_rng().gen_range(0..=u16::MAX),
        )];
    }

    if chance < 3 {
        return vec![ItemAttribute::Charges(rand::thread_rng().gen_range(0..=5))];
    }

    if chance < 4 {
        return vec![ItemAttribute::ActionId(
            rand::thread_rng().gen_range(300..=305),
        )];
    }

    if chance < 5 {
        return vec![ItemAttribute::UniqueId(
            rand::thread_rng().gen_range(300..=305),
        )];
    }

    if chance < 6 {
        return vec![ItemAttribute::DepotId(
            rand::thread_rng().gen_range(300..=305),
        )];
    }

    if chance < 7 {
        return vec![ItemAttribute::Text(
            rand::thread_rng().gen_range(10000..=10005).to_string(),
        )];
    }

    if chance < 8 {
        return vec![ItemAttribute::Flags(
            rand::thread_rng().gen_range(300..=305),
        )];
    }

    Vec::new()
}

pub fn get_chunks_per_z(edges: &Edges) -> Vec<Edges> {
    let mut chunks = Vec::new();
    let n = 1;

    for z in edges.min.z..=edges.max.z {
        for i in 1..=n {
            let y_divided_by_6 = (edges.max.y - edges.min.y) / n;
            let chunk_start =
                TilePosition::new(edges.min.x, edges.min.y + y_divided_by_6 * (i - 1), z);
            let chunk_end = TilePosition::new(edges.max.x, edges.min.y + y_divided_by_6 * i, z);
            chunks.push(Edges::new(chunk_start, chunk_end));
        }
    }

    chunks
}
