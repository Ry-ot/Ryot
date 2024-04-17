//! This module introduces the concepts of `Trajectory` and `RadialArea` for calculating
//! and representing the visible area or perspective from a given position. It utilizes ray casting
//! and angle-based calculations to determine visible tiles in a game world.
use bevy::math::bounding::{Aabb3d, RayCast3d};
use bevy::math::Ray3d;
use bevy::prelude::Component;
use glam::Vec3;

use crate::prelude::perspective::Perspective;
use ryot_grid::prelude::*;

/// Represents an area that can be traversable by a ray cast. This struct is pivotal for
/// calculating which areas of the game world are reachable from a certain position, using ray
/// casting for precise checks.
#[derive(Debug, Clone)]
pub struct Traversal {
    pub ray_cast: RayCast3d,
    pub target_area: Vec<TilePosition>,
}

impl Traversal {
    pub fn new(ray_cast: RayCast3d, target_area: Vec<TilePosition>) -> Self {
        Traversal {
            ray_cast,
            target_area,
        }
    }

    /// Returns a vector of tile positions from the target area that intersect with the ray cast.
    pub fn get_intersections(self) -> Vec<TilePosition> {
        self.get_intersections_with(|pos| Aabb3d::from(*pos))
    }

    /// Allows for custom AABB3d transformations when filtering intersections,
    /// providing flexibility in how intersections are calculated.
    pub fn get_intersections_with(
        self,
        aabb_transformer: impl Fn(&TilePosition) -> Aabb3d,
    ) -> Vec<TilePosition> {
        self.target_area
            .into_iter()
            .filter_map(|pos| {
                self.ray_cast
                    .aabb_intersection_at(&aabb_transformer(&pos))?;
                Some(pos)
            })
            .collect()
    }
}

/// Defines a radial area of interest from a specific point in the game world, characterized by
/// a range, center position, step angle, and an angle range. This struct is used to generate
/// `Perspective` objects that represent the observable area from the center position.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Component)]
pub struct RadialArea {
    pub range: u8,
    pub center_pos: TilePosition,
    pub angle_step: usize,
    pub angle_range: (u16, u16),
}

impl Default for RadialArea {
    fn default() -> Self {
        Self {
            range: 1,
            center_pos: TilePosition::new(0, 0, 0),
            angle_step: 10,
            angle_range: (315, 405),
        }
    }
}

impl RadialArea {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_center_pos(&self) -> TilePosition {
        self.center_pos
    }

    pub fn circle() -> Self {
        Self::default().with_angle_range((45, 405))
    }

    pub fn sector(start_angle: u16, end_angle: u16) -> Self {
        Self::default().with_angle_range((start_angle, end_angle))
    }

    pub fn with_range(self, range: u8) -> Self {
        Self { range, ..self }
    }

    pub fn with_center_pos(self, center_pos: TilePosition) -> Self {
        Self { center_pos, ..self }
    }

    pub fn with_angle_step(self, angle_step: usize) -> Self {
        Self { angle_step, ..self }
    }

    pub fn with_angle_range(self, angle_range: (u16, u16)) -> Self {
        let (start_angle, end_angle) = angle_range;

        Self {
            angle_range: if start_angle < end_angle {
                (start_angle, end_angle)
            } else {
                (end_angle, start_angle)
            },
            ..self
        }
    }
}

/// Implements conversion from `RadialArea` to `Perspective`, allowing easy creation of
/// perspective objects based on radial descriptions. This facilitates the dynamic generation
/// of visible areas based on the position and defined view angle of entities.
impl From<RadialArea> for Perspective {
    fn from(radial_area: RadialArea) -> Self {
        let RadialArea {
            range,
            center_pos,
            angle_step,
            angle_range: (start_angle, end_angle),
        } = radial_area;

        if range == 0 {
            return Perspective::default();
        }

        let center_pos_vec3 = Vec3::new(
            center_pos.x as f32,
            center_pos.y as f32,
            center_pos.z as f32,
        );

        Perspective::new(
            center_pos
                .tiles_on_arc_circumference(range, start_angle, end_angle, angle_step)
                .into_iter()
                .map(|arc_tile| {
                    let ray_cast = RayCast3d::from_ray(
                        Ray3d::new(center_pos_vec3, (arc_tile.0 - center_pos.0).as_vec3()),
                        range as f32,
                    );
                    Traversal::new(ray_cast, center_pos.bresenhams_line(arc_tile))
                })
                .collect::<Vec<_>>(),
        )
    }
}
