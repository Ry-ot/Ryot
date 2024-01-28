use crate::TilesetCategory;
use bevy::prelude::Resource;
use bevy::utils::HashMap;

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
            ]
            .into(),
        }
    }
}
