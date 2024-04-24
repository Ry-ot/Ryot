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
