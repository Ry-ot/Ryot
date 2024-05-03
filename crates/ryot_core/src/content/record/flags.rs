use crate::prelude::Navigable;

/// Standard implementation of `Navigable` used within the Ryot framework.
///
/// `Flags` struct serves as the default data structure for encoding walkability and sight-blocking
/// attributes of game elements, particularly those defined in the content catalog. It is used as
/// a component in ECS to assign physical properties to visual elements like tiles and objects,
/// supporting straightforward integration with pathfinding and visibility checks.
///
/// # Attributes
/// * `is_walkable` - Indicates whether the element permits movement over it.
/// * `blocks_sight` - Determines if the element impedes vision.
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

impl Flags {
    pub fn new(is_walkable: bool, blocks_sight: bool) -> Self {
        Flags {
            is_walkable,
            blocks_sight,
        }
    }

    pub fn walkable() -> Self {
        Flags {
            is_walkable: true,
            ..Flags::default()
        }
    }

    pub fn visible() -> Self {
        Flags {
            blocks_sight: true,
            ..Flags::default()
        }
    }

    pub fn with_walkable(self, is_walkable: bool) -> Self {
        Flags {
            is_walkable,
            ..self
        }
    }

    pub fn with_blocks_sight(self, blocks_sight: bool) -> Self {
        Flags {
            blocks_sight,
            ..self
        }
    }
}

impl Navigable for Flags {
    fn is_walkable(&self) -> bool {
        self.is_walkable
    }

    fn blocks_sight(&self) -> bool {
        self.blocks_sight
    }

    fn set_walkable(&mut self, walkable: bool) {
        self.is_walkable = walkable;
    }

    fn set_blocks_sight(&mut self, blocks_sight: bool) {
        self.blocks_sight = blocks_sight;
    }

    fn is_default(&self) -> bool {
        *self == Flags::default()
    }
}
