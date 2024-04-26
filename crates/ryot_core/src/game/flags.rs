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
