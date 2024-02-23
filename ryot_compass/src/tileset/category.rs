use egui::WidgetText;
use ryot::appearances::{Appearance, AppearanceFlags, ItemCategory};
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

impl From<&AppearanceFlags> for TilesetCategory {
    fn from(flags: &AppearanceFlags) -> Self {
        // Market has categories, so we can use it to determine the category of the item.
        // If the item has a market flag, it's category is prioritized over the other flags.
        if let Some(market) = &flags.market {
            if let Some(category) = market.category {
                return (&ItemCategory::try_from(category).unwrap()).into();
            }
        }

        if flags.bank.is_some() {
            return TilesetCategory::Terrains;
        }

        if flags.clip.is_some() {
            return TilesetCategory::Edges;
        }

        if flags.bottom.is_some() {
            return TilesetCategory::BaseLayer;
        }

        if flags.top.is_some() {
            return TilesetCategory::UpperLayer;
        }

        // Corpses are also containers, so they need to be checked first
        if flags.corpse.is_some() || flags.player_corpse.is_some() {
            return TilesetCategory::Corpses;
        }

        if flags.container.is_some() {
            return TilesetCategory::Containers;
        }

        if flags.hang.is_some() || flags.hook.is_some() || flags.rotate.is_some() {
            return TilesetCategory::Decor;
        }

        if flags.clothes.is_some() {
            return TilesetCategory::Clothes;
        }

        TilesetCategory::Miscellaneous
    }
}

impl From<&ItemCategory> for TilesetCategory {
    fn from(category: &ItemCategory) -> Self {
        match category {
            ItemCategory::Ammunition
            | ItemCategory::Axes
            | ItemCategory::Clubs
            | ItemCategory::DistanceWeapons
            | ItemCategory::Shields
            | ItemCategory::Quiver
            | ItemCategory::Swords
            | ItemCategory::WandsRods => TilesetCategory::Weapons,
            ItemCategory::Armors
            | ItemCategory::Amulets
            | ItemCategory::Boots
            | ItemCategory::HelmetsHats
            | ItemCategory::Legs
            | ItemCategory::Rings => TilesetCategory::Clothes,
            ItemCategory::CreatureProducts => TilesetCategory::CreatureProducts,
            ItemCategory::Containers => TilesetCategory::Containers,
            ItemCategory::Decoration => TilesetCategory::Decor,
            ItemCategory::Food | ItemCategory::Potions | ItemCategory::Runes => {
                TilesetCategory::Consumables
            }
            ItemCategory::PremiumScrolls | ItemCategory::TibiaCoins | ItemCategory::Valuables => {
                TilesetCategory::Valuables
            }
            ItemCategory::Others => TilesetCategory::Miscellaneous,
            ItemCategory::Tools => TilesetCategory::Tools,
        }
    }
}

impl From<&Appearance> for TilesetCategory {
    fn from(appearance: &Appearance) -> Self {
        if let Some(flags) = &appearance.flags {
            return flags.into();
        }

        TilesetCategory::Miscellaneous
    }
}

impl From<&PreparedAppearance> for TilesetCategory {
    fn from(appearance: &PreparedAppearance) -> Self {
        if let Some(flags) = &appearance.flags {
            return flags.into();
        }

        TilesetCategory::Miscellaneous
    }
}

impl PartialOrd<Self> for TilesetCategory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TilesetCategory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
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
