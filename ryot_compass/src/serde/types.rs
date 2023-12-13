use serde::{Deserialize, Serialize};

pub trait GetKey {
    fn get_binary_key(&self) -> Vec<u8>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub z: u8,
}

impl GetKey for Position {
    fn get_binary_key(&self) -> Vec<u8> {
        let mut key = Vec::with_capacity(5);
        key.extend_from_slice(&self.x.to_be_bytes());
        key.extend_from_slice(&self.y.to_be_bytes());
        key.push(self.z);
        key
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: u16::MIN,
            y: u16::MIN,
            z: 7,
        }
    }
}

impl Position {
    pub fn new(x: u16, y: u16, z: u8) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header { // header
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
    pub items: Vec<Item>,
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
    Teleport(Position),
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
    position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    name: String,
    position: Position,
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
    pub entry_position: Position,
    pub rent: u32,
    pub guild_hall: bool,
    pub town_id: u8,
    pub size: u16,
    pub beds: u8,
}