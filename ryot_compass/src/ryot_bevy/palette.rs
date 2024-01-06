/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */


use bevy::prelude::Resource;
use bevy::utils::HashMap;
use crate::TilesetCategory;

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