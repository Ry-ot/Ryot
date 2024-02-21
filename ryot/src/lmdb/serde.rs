use crate::layer::Layer;
use crate::position::TilePosition;
use bevy::math::i32;
use heed::{BoxedError, BytesDecode, BytesEncode};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Default, Clone)]
pub struct Tile {
    pub position: TilePosition,
    pub items: HashMap<Layer, Item>,
}

impl Tile {
    pub fn new(position: TilePosition, items: HashMap<Layer, Item>) -> Self {
        Self { position, items }
    }

    pub fn from_pos(position: TilePosition) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    pub fn set_item(&mut self, item: Item, layer: Layer) {
        self.items.insert(layer, item);
    }
}

pub struct SerdePostcard<T>(PhantomData<T>);

impl<'a, T: 'a> BytesEncode<'a> for SerdePostcard<T>
where
    T: Serialize,
{
    type EItem = T;

    fn bytes_encode(item: &Self::EItem) -> Result<Cow<[u8]>, BoxedError> {
        postcard::to_allocvec(item)
            .map(Cow::Owned)
            .map_err(Into::into)
    }
}

impl<'a, T: 'a> BytesDecode<'a> for SerdePostcard<T>
where
    T: Deserialize<'a>,
{
    type DItem = T;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, BoxedError> {
        postcard::from_bytes(bytes).map_err(Into::into)
    }
}

unsafe impl<T> Send for SerdePostcard<T> {}

unsafe impl<T> Sync for SerdePostcard<T> {}

pub trait GetKey {
    fn get_binary_key(&self) -> Vec<u8>;
    fn from_binary_key(key: &[u8]) -> Self;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub z: u8,
}

impl GetKey for TilePosition {
    fn get_binary_key(&self) -> Vec<u8> {
        let mut key = Vec::with_capacity(9);
        key.extend_from_slice(&self.x.to_be_bytes());
        key.extend_from_slice(&self.y.to_be_bytes());
        key.extend_from_slice(&(self.z as i8).to_be_bytes());
        key
    }

    fn from_binary_key(key: &[u8]) -> Self {
        let x = i32::from_be_bytes([key[0], key[1], key[2], key[3]]);
        let y = i32::from_be_bytes([key[4], key[5], key[6], key[7]]);
        let z = i8::from_be_bytes([key[8]]) as i32;
        Self::new(x, y, z)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    // header
    pub width: u16,
    pub height: u16,
    pub floors: u8,
    pub version: u8,
    pub description: String,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            width: u16::MAX,
            height: u16::MAX,
            floors: 15,
            version: 1,
            description: "This is a new ryot_compass".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u16,
    pub attributes: Vec<ItemAttribute>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ItemAttribute {
    Count(u8),
    DoorId(u8),
    HouseId(u16),
    Charges(u8),
    ActionId(u16),
    UniqueId(u16),
    DepotId(u16),
    Text(String),
    Description(String),
    Flags(u32),
    Teleport(TilePosition),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Spawn {
    pub uid: u16,
    pub radius: u8,
    pub spawn_time: u16,
    pub entities: Vec<SpawnType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SpawnType {
    Monster(Monster),
    Npc(Npc),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Monster {
    pub name: String,
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Npc {
    pub name: String,
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegionType {
    Town(Town),
    Waypoint(Waypoint),
    Zone(Zone),
    House(House),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Town {
    id: u8,
    name: String,
    position: TilePosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    name: String,
    position: TilePosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    id: u8,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct House {
    pub id: u32,
    pub name: String,
    pub entry_position: TilePosition,
    pub rent: u32,
    pub guild_hall: bool,
    pub town_id: u8,
    pub size: u16,
    pub beds: u8,
}
