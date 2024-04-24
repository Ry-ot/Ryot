//! # Conversion Implementations from Tibia Assets to Ryot Visual Elements
//!
//! This module provides implementations of the `From` trait to convert types from the `tibia` proto,
//! which represents assets and appearances from Tibia (an external dependency), into corresponding
//! domain-specific types defined within the Ryot system. These conversions help abstract and adapt
//! the external Tibia data formats to the internal representations used by the Ryot application,
//! facilitating easier management and integration of visual elements.
//!
//! ## Overview
//!
//! Each implementation of the `From` trait here is designed to transform a specific `tibia` type into a
//! more functionally relevant type for the Ryot application. This includes:
//! - Visual elements such as `VisualElement`, `FrameType`, `SpriteInfo`, and `Animation`.
//! - Handling of nullable types and ensuring sensible defaults for optional data from `tibia`.
//! - Mapping complex flags and properties from `tibia` types into simplified, application-specific flags
//!   and categories.
//!
//! These conversions play a crucial role in separating external data dependencies from the core logic
//! of the application, thus maintaining a clean architecture and ensuring that changes in external
//! data structures have minimal impact on internal business logic.
//!
//! ## Custom Traits and Macros
//!
//! The file also includes a macro `option_flag_to_element!` that generates default implementations for
//! converting optional `tibia::Flags` into domain-specific classes, providing a standardized approach to
//! handling optional data with defaults.
//!
//! ## Usage
//!
//! The `From` trait implementations are intended for use across the Ryot application wherever
//! conversions from `tibia` types to internal Ryot types are necessary. By centralizing these conversions
//! in one module, we ensure consistent behavior and data transformations throughout the application,
//! facilitating easier updates and maintenance when dealing with external asset changes.
use crate as tibia;
use ryot_content::prelude::*;
use ryot_sprites::prelude::*;

impl From<tibia::VisualElements> for VisualElements {
    fn from(item: tibia::VisualElements) -> Self {
        fn process_items(
            items: &[tibia::VisualElement],
            entity_type: EntityType,
            visual_elements: &mut VisualElements,
        ) {
            for item in items.iter() {
                let visual_element: VisualElement = item.clone().into();
                if visual_element.id == 0 || visual_element.sprites_info.is_empty() {
                    continue;
                }
                visual_elements
                    .entry(entity_type)
                    .or_default()
                    .insert(visual_element.id, visual_element);
            }
        }

        let mut visual_elements = VisualElements::default();

        process_items(&item.objects, EntityType::Object, &mut visual_elements);
        process_items(&item.outfits, EntityType::Outfit, &mut visual_elements);
        process_items(&item.effects, EntityType::Effect, &mut visual_elements);
        process_items(&item.missiles, EntityType::Missile, &mut visual_elements);

        visual_elements
    }
}

impl From<tibia::VisualElement> for VisualElement {
    fn from(item: tibia::VisualElement) -> Self {
        fn from_flags<T: Clone + Default + From<tibia::Flags>>(flags: &Option<tibia::Flags>) -> T {
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

impl From<tibia::FrameType> for FrameGroup {
    fn from(item: tibia::FrameType) -> Self {
        match item {
            tibia::FrameType::OutfitIdle => FrameGroup::Idle,
            tibia::FrameType::OutfitMoving => FrameGroup::Moving,
            tibia::FrameType::ObjectInitial => FrameGroup::Initial,
        }
    }
}

impl From<tibia::SpriteInfo> for SpriteInfo {
    fn from(item: tibia::SpriteInfo) -> Self {
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

impl From<tibia::Animation> for Animation {
    fn from(item: tibia::Animation) -> Self {
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

impl From<tibia::Flags> for Flags {
    fn from(item: tibia::Flags) -> Self {
        Flags {
            is_walkable: !item.is_not_walkable(),
            blocks_sight: item.blocks_sight(),
        }
    }
}

impl From<tibia::Flags> for Properties {
    fn from(item: tibia::Flags) -> Self {
        Properties {
            ground_speed: item.ground.clone().unwrap_or_default().speed(),
            elevation: item.elevation.clone().unwrap_or_default().height(),
        }
    }
}

impl From<tibia::Flags> for Category {
    fn from(flags: tibia::Flags) -> Self {
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
