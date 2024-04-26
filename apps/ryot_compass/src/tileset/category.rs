use egui::WidgetText;
use ryot::prelude::tibia::StoreCategory;
use ryot::prelude::*;
use std::cmp::Ordering;
use strum::{EnumCount, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumCount)]
pub enum TilesetCategory {
    Terrains,
    Edges,
    BaseLayer,
    UpperLayer,
    Decor,
    Corpses,
    Containers,
    Clothes,
    Consumables,
    Tools,
    Miscellaneous,
    Valuables,
    CreatureProducts,
    Weapons,
    Raw,
}

impl From<&Category> for TilesetCategory {
    fn from(category: &Category) -> Self {
        match category {
            Category::Bottom => TilesetCategory::BaseLayer,
            Category::Containers => TilesetCategory::Containers,
            Category::Corpses => TilesetCategory::Corpses,
            Category::Decor => TilesetCategory::Decor,
            Category::Edges => TilesetCategory::Edges,
            Category::Ground => TilesetCategory::Terrains,
            Category::Custom(category) => StoreCategory::try_from(*category).unwrap().into(),
            Category::Miscellaneous => TilesetCategory::Miscellaneous,
            Category::Top => TilesetCategory::UpperLayer,
            Category::Wearable => TilesetCategory::Clothes,
        }
    }
}

impl From<StoreCategory> for TilesetCategory {
    fn from(category: StoreCategory) -> Self {
        match category {
            StoreCategory::Ammunition
            | StoreCategory::Axes
            | StoreCategory::Clubs
            | StoreCategory::DistanceWeapons
            | StoreCategory::Shields
            | StoreCategory::Quiver
            | StoreCategory::Swords
            | StoreCategory::WandsRods => TilesetCategory::Weapons,
            StoreCategory::Armors
            | StoreCategory::Amulets
            | StoreCategory::Boots
            | StoreCategory::HelmetsHats
            | StoreCategory::Legs
            | StoreCategory::Rings => TilesetCategory::Clothes,
            StoreCategory::CreatureProducts => TilesetCategory::CreatureProducts,
            StoreCategory::Containers => TilesetCategory::Containers,
            StoreCategory::Decoration => TilesetCategory::Decor,
            StoreCategory::Food | StoreCategory::Potions | StoreCategory::Runes => {
                TilesetCategory::Consumables
            }
            StoreCategory::PremiumScrolls
            | StoreCategory::TibiaCoins
            | StoreCategory::Valuables => TilesetCategory::Valuables,
            StoreCategory::Others => TilesetCategory::Miscellaneous,
            StoreCategory::Tools => TilesetCategory::Tools,
        }
    }
}

impl From<&VisualElement> for TilesetCategory {
    fn from(visual_element: &VisualElement) -> Self {
        (&visual_element.category).into()
    }
}

impl PartialOrd<Self> for TilesetCategory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TilesetCategory {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_label().cmp(&other.get_label())
    }
}

impl From<&TilesetCategory> for WidgetText {
    fn from(val: &TilesetCategory) -> Self {
        WidgetText::from(val.get_label())
    }
}

impl TilesetCategory {
    pub fn get_label(&self) -> String {
        match self {
            TilesetCategory::Terrains => String::from("Terrains"),
            TilesetCategory::Edges => String::from("Edges"),
            TilesetCategory::BaseLayer => String::from("Base Layer"),
            TilesetCategory::UpperLayer => String::from("Upper Layer"),
            TilesetCategory::Decor => String::from("Decor"),
            TilesetCategory::Corpses => String::from("Corpses"),
            TilesetCategory::Containers => String::from("Containers"),
            TilesetCategory::Clothes => String::from("Clothes"),
            TilesetCategory::Consumables => String::from("Consumables"),
            TilesetCategory::Tools => String::from("Tools"),
            TilesetCategory::Miscellaneous => String::from("Miscellaneous"),
            TilesetCategory::Valuables => String::from("Valuables"),
            TilesetCategory::CreatureProducts => String::from("Creature Products"),
            TilesetCategory::Weapons => String::from("Weapons"),
            TilesetCategory::Raw => String::from("Raw"),
        }
    }
}
