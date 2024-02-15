use glam::IVec2;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[cfg(feature = "bevy")]
use bevy::ecs::component::Component;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Default, Debug)]
#[cfg_attr(feature = "bevy", derive(Serialize, Deserialize))]
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

#[derive(Hash, PartialEq, Eq, EnumIter, Default, Clone, Copy, Debug)]
#[cfg_attr(feature = "bevy", derive(Serialize, Deserialize))]
pub enum OrdinalDirection {
    NorthWest,
    North,
    NorthEast,
    West,
    #[default]
    None,
    East,
    SouthWest,
    South,
    SouthEast,
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
        match value {
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

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
#[cfg_attr(feature = "bevy", derive(Component, Serialize, Deserialize))]
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
