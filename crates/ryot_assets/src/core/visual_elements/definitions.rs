use derive_more::{Deref, DerefMut};

#[cfg(feature = "bevy")]
use bevy_utils::HashMap;

#[cfg(not(feature = "bevy"))]
use std::collections::HashMap;

#[derive(Hash, Eq, Default, PartialEq, Debug, Copy, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[repr(usize)]
pub enum EntityType {
    #[default]
    Object,
    Outfit,
    Effect,
    Missile,
}

#[derive(Clone, PartialEq, Default, Debug, Deref, DerefMut)]
#[cfg_attr(
    feature = "bevy",
    derive(bevy_ecs::prelude::Resource, bevy_reflect::TypePath, bevy_asset::Asset)
)]
pub struct VisualElements(HashMap<EntityType, HashMap<u32, VisualElement>>);

impl VisualElements {
    pub fn get_all_for_group(&self, group: EntityType) -> Option<&HashMap<u32, VisualElement>> {
        self.get(&group)
    }

    pub fn get_for_group_and_id(&self, group: EntityType, id: u32) -> Option<&VisualElement> {
        self.get(&group)?.get(&id)
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct VisualElement {
    pub id: u32,
    pub name: String,
    pub main_sprite_id: Option<u32>,
    pub sprites_info: Vec<SpriteInfo>,
    pub flags: Flags,
    pub category: Category,
    pub properties: Properties,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Component))]
#[repr(usize)]
pub enum FrameGroup {
    #[default]
    Idle = 0,
    Moving = 1,
    Initial = 2,
}

impl FrameGroup {
    pub fn set_moving(&mut self, moving: bool) {
        *self = match self {
            Self::Initial => *self,
            _ if moving => FrameGroup::Moving,
            _ => FrameGroup::Idle,
        };
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct SpriteInfo {
    pub ids: Vec<u32>,
    pub layers: u32,
    pub pattern_width: u32,
    pub pattern_height: u32,
    pub pattern_depth: u32,
    pub animation: Option<Animation>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Animation {
    pub start_phase: u32,
    pub synchronized: bool,
    pub is_start_random: bool,
    pub phases: Vec<(u32, u32)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Component))]
pub struct Flags {
    pub is_walkable: bool,
    pub blocks_sight: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Flags {
            is_walkable: true,
            blocks_sight: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Properties {
    pub ground_speed: u32,
    pub elevation: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Category {
    Bottom,
    Containers,
    Corpses,
    Decor,
    Edges,
    Ground,
    #[default]
    Miscellaneous,
    Top,
    Wearable,
    Custom(i32),
}
