//! # Conversion Implementations from Cipsoft Assets to Ryot Visual Elements
//!
//! This module provides implementations of the `From` trait to convert types from the `cip` module,
//! which represents assets and appearances from Cipsoft (an external dependency), into corresponding
//! domain-specific types defined within the Ryot system. These conversions help abstract and adapt
//! the external Cipsoft data formats to the internal representations used by the Ryot application,
//! facilitating easier management and integration of visual elements.
//!
//! ## Overview
//!
//! Each implementation of the `From` trait here is designed to transform a specific `cip` type into a
//! more functionally relevant type for the Ryot application. This includes:
//! - Visual elements such as `VisualElement`, `FrameType`, `SpriteInfo`, and `Animation`.
//! - Handling of nullable types and ensuring sensible defaults for optional data from `cip`.
//! - Mapping complex flags and properties from `cip` types into simplified, application-specific flags
//!   and categories.
//!
//! These conversions play a crucial role in separating external data dependencies from the core logic
//! of the application, thus maintaining a clean architecture and ensuring that changes in external
//! data structures have minimal impact on internal business logic.
//!
//! ## Custom Traits and Macros
//!
//! The file also includes a macro `option_flag_to_element!` that generates default implementations for
//! converting optional `cip::Flags` into domain-specific classes, providing a standardized approach to
//! handling optional data with defaults.
//!
//! ## Usage
//!
//! The `From` trait implementations are intended for use across the Ryot application wherever
//! conversions from `cip` types to internal Ryot types are necessary. By centralizing these conversions
//! in one module, we ensure consistent behavior and data transformations throughout the application,
//! facilitating easier updates and maintenance when dealing with external asset changes.
use crate as cip;
use ryot_assets::prelude::*;

impl From<cip::VisualElements> for VisualElements {
    fn from(item: cip::VisualElements) -> Self {
        let convert = |item: &cip::VisualElement| -> VisualElement { item.clone().into() };

        VisualElements {
            objects: item.objects.iter().map(convert).collect(),
            outfits: item.outfits.iter().map(convert).collect(),
            effects: item.effects.iter().map(convert).collect(),
            missiles: item.missiles.iter().map(convert).collect(),
        }
    }
}

impl From<cip::VisualElement> for VisualElement {
    fn from(item: cip::VisualElement) -> Self {
        fn from_flags<T: Clone + Default + From<cip::Flags>>(flags: &Option<cip::Flags>) -> T {
            match flags {
                Some(flags) => flags.clone().into(),
                None => T::default(),
            }
        }

        let id = item.id();
        let name: String = item.name.clone().unwrap_or(id.to_string());
        let flags: Flags = from_flags(&item.flags);
        let category: Category = from_flags(&item.flags);
        let properties: Properties = from_flags(&item.flags);
        let sprites_info: Vec<SpriteInfo> = item
            .frames
            .iter()
            .map(|frame| frame.sprite_info.clone().unwrap().into())
            .collect();

        let main_sprite_id = sprites_info
            .first()
            .and_then(|main_sprite| main_sprite.ids.first().copied());

        VisualElement {
            id,
            name,
            main_sprite_id,
            sprites_info,
            flags,
            category,
            properties,
        }
    }
}

impl From<cip::FrameType> for FrameGroup {
    fn from(item: cip::FrameType) -> Self {
        match item {
            cip::FrameType::OutfitIdle => FrameGroup::Idle,
            cip::FrameType::OutfitMoving => FrameGroup::Moving,
            cip::FrameType::ObjectInitial => FrameGroup::Initial,
        }
    }
}

impl From<cip::SpriteInfo> for SpriteInfo {
    fn from(item: cip::SpriteInfo) -> Self {
        let ids = item.sprite_ids.clone();
        let layers = item.layers();
        let pattern_width = item.pattern_width();
        let pattern_height = item.pattern_height();
        let pattern_depth = item.pattern_depth();
        let animation = item.animation.map(|a| a.into());

        SpriteInfo {
            ids,
            layers,
            pattern_width,
            pattern_height,
            pattern_depth,
            animation,
        }
    }
}

impl From<cip::Animation> for Animation {
    fn from(item: cip::Animation) -> Self {
        let start_phase = item.start_phase();
        let synchronized = item.synchronized();
        let is_start_random = item.is_start_random();
        let phases = item.phases.iter().map(|p| (p.min(), p.max())).collect();

        Animation {
            start_phase,
            synchronized,
            is_start_random,
            phases,
        }
    }
}

impl From<cip::Flags> for Flags {
    fn from(item: cip::Flags) -> Self {
        Flags {
            is_walkable: !item.is_not_walkable(),
            blocks_sight: item.blocks_sight(),
        }
    }
}

impl From<cip::Flags> for Properties {
    fn from(item: cip::Flags) -> Self {
        Properties {
            ground_speed: item.ground.clone().unwrap_or_default().speed(),
            elevation: item.elevation.clone().unwrap_or_default().height(),
        }
    }
}

impl From<cip::Flags> for Category {
    fn from(flags: cip::Flags) -> Self {
        // Market has categories, so we can use it to determine the category of the item.
        // If the item has a market flag, it's category is prioritized over the other category.
        if let Some(market) = &flags.market_info {
            if let Some(category) = market.category {
                return Category::Custom(category);
            }
        }

        match flags {
            _ if flags.is_bottom() => Category::Bottom,
            _ if flags.is_edge() => Category::Edges,
            _ if flags.is_ground() => Category::Ground,
            _ if flags.is_top() => Category::Top,
            // Corpses are also containers, so they need to be checked first
            _ if flags.is_corpse() => Category::Corpses,
            _ if flags.is_container() => Category::Containers,
            _ if flags.can_be_hanged() || flags.can_rotate() || flags.hook_info.is_some() => {
                Category::Decor
            }
            _ if flags.slot.is_some() => Category::Wearable,
            _ => Category::Miscellaneous,
        }
    }
}
