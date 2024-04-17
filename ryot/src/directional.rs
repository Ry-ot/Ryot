use std::ops::Deref;

use glam::{IVec2, Vec2, Vec3};
use rand::distributions::Distribution;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

#[cfg(feature = "bevy")]
use bevy::ecs::component::Component;

use crate::prelude::*;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub enum CardinalDirection {
    #[default]
    North,
    East,
    South,
    West,
}

impl From<CardinalDirection> for IVec2 {
    fn from(value: CardinalDirection) -> Self {
        OrdinalDirection::from(value).into()
    }
}

impl From<IVec2> for CardinalDirection {
    fn from(value: IVec2) -> Self {
        OrdinalDirection::from(value).into()
    }
}

#[derive(Hash, PartialEq, Eq, EnumIter, Default, Clone, Copy, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum OrdinalDirection {
    North = 0,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    #[default]
    None,
}

impl OrdinalDirection {
    pub fn get_next(&self) -> Self {
        Self::iter()
            .find(|&dir| dir as u8 == (*self as u8 + 1) % 8)
            .unwrap_or_default()
    }

    pub fn get_previous(&self) -> Self {
        Self::iter()
            .find(|&dir| dir as u8 == (*self as u8 + 7) % 8)
            .unwrap_or_default()
    }
}

impl Distribution<OrdinalDirection> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> OrdinalDirection {
        match rng.gen_range(0..8) {
            0 => OrdinalDirection::North,
            1 => OrdinalDirection::NorthEast,
            2 => OrdinalDirection::East,
            3 => OrdinalDirection::SouthEast,
            4 => OrdinalDirection::South,
            5 => OrdinalDirection::SouthWest,
            6 => OrdinalDirection::West,
            7 => OrdinalDirection::NorthWest,
            _ => OrdinalDirection::None,
        }
    }
}

impl From<OrdinalDirection> for CardinalDirection {
    fn from(value: OrdinalDirection) -> Self {
        match value {
            OrdinalDirection::North => CardinalDirection::North,
            OrdinalDirection::NorthEast => CardinalDirection::East,
            OrdinalDirection::East => CardinalDirection::East,
            OrdinalDirection::SouthEast => CardinalDirection::East,
            OrdinalDirection::South => CardinalDirection::South,
            OrdinalDirection::SouthWest => CardinalDirection::West,
            OrdinalDirection::West => CardinalDirection::West,
            OrdinalDirection::NorthWest => CardinalDirection::West,
            OrdinalDirection::None => CardinalDirection::North,
        }
    }
}

impl From<CardinalDirection> for OrdinalDirection {
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::North => OrdinalDirection::North,
            CardinalDirection::East => OrdinalDirection::East,
            CardinalDirection::South => OrdinalDirection::South,
            CardinalDirection::West => OrdinalDirection::West,
        }
    }
}

impl From<OrdinalDirection> for IVec2 {
    fn from(value: OrdinalDirection) -> Self {
        match value {
            OrdinalDirection::North => [0, 1].into(),
            OrdinalDirection::NorthEast => [1, 1].into(),
            OrdinalDirection::East => [1, 0].into(),
            OrdinalDirection::SouthEast => [1, -1].into(),
            OrdinalDirection::South => [0, -1].into(),
            OrdinalDirection::SouthWest => [-1, -1].into(),
            OrdinalDirection::West => [-1, 0].into(),
            OrdinalDirection::NorthWest => [-1, 1].into(),
            OrdinalDirection::None => [0, 0].into(),
        }
    }
}

impl From<IVec2> for OrdinalDirection {
    fn from(value: IVec2) -> Self {
        match value.clamp(IVec2::splat(-1), IVec2::splat(1)) {
            IVec2 { x: 0, y: 1 } => OrdinalDirection::North,
            IVec2 { x: 1, y: 1 } => OrdinalDirection::NorthEast,
            IVec2 { x: 1, y: 0 } => OrdinalDirection::East,
            IVec2 { x: 1, y: -1 } => OrdinalDirection::SouthEast,
            IVec2 { x: 0, y: -1 } => OrdinalDirection::South,
            IVec2 { x: -1, y: -1 } => OrdinalDirection::SouthWest,
            IVec2 { x: -1, y: 0 } => OrdinalDirection::West,
            IVec2 { x: -1, y: 1 } => OrdinalDirection::NorthWest,
            _ => OrdinalDirection::None,
        }
    }
}

impl From<Vec2> for OrdinalDirection {
    fn from(value: Vec2) -> Self {
        match value.clamp(Vec2::splat(-1.0), Vec2::splat(1.0)).as_ivec2() {
            IVec2 { x: 0, y: 1 } => OrdinalDirection::North,
            IVec2 { x: 1, y: 1 } => OrdinalDirection::NorthEast,
            IVec2 { x: 1, y: 0 } => OrdinalDirection::East,
            IVec2 { x: 1, y: -1 } => OrdinalDirection::SouthEast,
            IVec2 { x: 0, y: -1 } => OrdinalDirection::South,
            IVec2 { x: -1, y: -1 } => OrdinalDirection::SouthWest,
            IVec2 { x: -1, y: 0 } => OrdinalDirection::West,
            IVec2 { x: -1, y: 1 } => OrdinalDirection::NorthWest,
            _ => OrdinalDirection::None,
        }
    }
}

impl From<Vec3> for OrdinalDirection {
    fn from(value: Vec3) -> Self {
        value.truncate().into()
    }
}

impl From<TilePosition> for OrdinalDirection {
    fn from(value: TilePosition) -> Self {
        value.deref().truncate().into()
    }
}

impl From<OrdinalDirection> for (u16, u16) {
    fn from(ordinal: OrdinalDirection) -> Self {
        match ordinal {
            OrdinalDirection::East => (0, 1),
            OrdinalDirection::NorthEast => (45, 46),
            OrdinalDirection::North => (90, 91),
            OrdinalDirection::NorthWest => (135, 136),
            OrdinalDirection::West => (180, 181),
            OrdinalDirection::SouthWest => (225, 226),
            OrdinalDirection::South => (270, 271),
            OrdinalDirection::SouthEast => (315, 316),
            _ => (0, 0),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Component))]
pub enum Directional {
    Cardinal(CardinalDirection),
    Ordinal(OrdinalDirection),
}

impl Default for Directional {
    fn default() -> Self {
        Directional::Cardinal(CardinalDirection::default())
    }
}

impl Directional {
    pub fn index(self) -> usize {
        match self {
            Directional::Cardinal(cardinal) => cardinal as usize,
            Directional::Ordinal(ordinal) => ordinal as usize,
        }
    }
}

impl From<Directional> for IVec2 {
    fn from(value: Directional) -> Self {
        match value {
            Directional::Cardinal(cardinal) => cardinal.into(),
            Directional::Ordinal(ordinal) => ordinal.into(),
        }
    }
}
