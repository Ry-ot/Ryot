use crate::grid::TilePosition;
use glam::{IVec2, IVec3, Vec2};
use std::fmt;
use std::fmt::Formatter;

#[cfg(feature = "bevy")]
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

mod operations;

#[derive(Hash, Eq, PartialEq, Default, Clone, Copy, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Component))]
pub struct Sector {
    pub min: TilePosition,
    pub max: TilePosition,
}

impl Sector {
    pub const ZERO: Sector = Sector {
        min: TilePosition::ZERO,
        max: TilePosition::ZERO,
    };
}

impl Sector {
    pub const BASE_CANVAS_SECTOR: Sector = Sector {
        min: TilePosition(IVec3 {
            x: -30,
            y: -30,
            z: 0,
        }),
        max: TilePosition(IVec3 { x: 30, y: 30, z: 0 }),
    };

    pub fn new(min: TilePosition, max: TilePosition) -> Self {
        Self { min, max }
    }

    #[cfg(feature = "bevy")]
    pub fn from_transform_and_projection(
        transform: &Transform,
        projection: &OrthographicProjection,
    ) -> Self {
        let visible_width = projection.area.max.x - projection.area.min.x;
        let visible_height = projection.area.max.y - projection.area.min.y;

        // Adjust by the camera scale if necessary
        let visible_width = visible_width * transform.scale.x;
        let visible_height = visible_height * transform.scale.y;

        // Calculate boundaries based on the camera's position
        let camera_position = transform.translation;
        let left_bound = camera_position.x - visible_width / 2.0;
        let right_bound = camera_position.x + visible_width / 2.0;
        let bottom_bound = camera_position.y - visible_height / 2.0;
        let top_bound = camera_position.y + visible_height / 2.0;

        Self {
            min: TilePosition::from(Vec2::new(left_bound, bottom_bound)),
            max: TilePosition::from(Vec2::new(right_bound, top_bound)),
        }
    }

    pub fn size(&self) -> IVec2 {
        IVec2::new(self.max.x - self.min.x, self.max.y - self.min.y)
    }

    pub fn area(&self) -> u32 {
        (self.size().x * self.size().y).unsigned_abs()
    }

    pub fn contains(&self, pos: TilePosition) -> bool {
        pos.x >= self.min.x
            && pos.x <= self.max.x
            && pos.y >= self.min.y
            && pos.y <= self.max.y
            && pos.z == self.min.z
    }
}

impl fmt::Display for Sector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Edges({}, {})", self.min, self.max)
    }
}
