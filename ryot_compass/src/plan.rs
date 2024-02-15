use crate::{Header, House, Item, RegionType, Spawn, Town, Waypoint, Zone};
use ryot::layer::Layer;
use ryot::position::TilePosition;
use std::collections::HashMap;

#[derive(Debug)]
pub enum MapComponent {
    Tile(Tile),
    Spawn(Spawn),
    Region(RegionType),
}

#[derive(Debug)]
pub struct Plan {
    pub header: Header,
    pub tiles: Vec<Tile>,
    pub spawns: Vec<Spawn>,
    pub regions: Regions,
}

impl Default for Plan {
    fn default() -> Self {
        Self {
            header: Default::default(),
            tiles: Vec::new(),
            regions: Regions::new(),
            spawns: Vec::new(),
        }
    }
}

impl Plan {
    pub fn add(&mut self, item: MapComponent) {
        match item {
            MapComponent::Tile(tile) => self.tiles.push(tile),
            MapComponent::Region(region) => self.regions.add(region),
            MapComponent::Spawn(spawn) => self.spawns.push(spawn),
        }
    }
}

#[derive(Debug, Default)]
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

#[derive(Debug, Clone)]
pub struct Regions {
    pub towns: Vec<Town>,
    pub waypoints: Vec<Waypoint>,
    pub zones: Vec<Zone>,
    pub houses: Vec<House>,
}

impl Default for Regions {
    fn default() -> Self {
        Self::new()
    }
}

impl Regions {
    pub fn new() -> Self {
        Self {
            towns: Vec::new(),
            waypoints: Vec::new(),
            zones: Vec::new(),
            houses: Vec::new(),
        }
    }

    pub fn add(&mut self, region: RegionType) {
        match region {
            RegionType::Town(town) => self.towns.push(town),
            RegionType::Waypoint(waypoint) => self.waypoints.push(waypoint),
            RegionType::Zone(zone) => self.zones.push(zone),
            RegionType::House(house) => self.houses.push(house),
        }
    }
}
