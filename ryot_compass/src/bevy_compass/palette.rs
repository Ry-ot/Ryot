use crate::TilesetCategory;
use bevy::prelude::Resource;
use bevy::utils::HashMap;

#[derive(Debug, Clone, Resource, Default)]
pub struct Palette {
    tile_set: HashMap<TilesetCategory, Vec<u32>>,
}

impl Palette {
    pub fn add_to_category(&mut self, category: TilesetCategory, id: u32) {
        self.tile_set.entry(category).or_default().push(id);
    }

    pub fn get_categories(&self) -> Vec<&TilesetCategory> {
        let mut categories: Vec<_> = self.tile_set.keys().collect();
        categories.push(&TilesetCategory::Raw);
        categories.sort();
        categories
    }

    pub fn get_for_category(&self, category: &TilesetCategory) -> Vec<u32> {
        match category {
            TilesetCategory::Raw => {
                // get the merge of all arrays
                let mut merged = vec![];
                for (_, v) in self.tile_set.iter() {
                    merged.extend(v);
                }
                merged
            }
            _ => self.tile_set.get(category).unwrap().to_vec(),
        }
    }
}
