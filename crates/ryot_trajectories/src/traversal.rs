//! This module introduces the concepts of `Trajectory` and `RadialArea` for calculating
//! and representing the visible area or perspective from a given position. It utilizes ray casting
//! and angle-based calculations to determine visible tiles in a game world.
use bevy_ecs::prelude::Component;
use bevy_math::bounding::RayCast3d;
use bevy_math::Ray3d;
use glam::Vec3;
use ryot_core::prelude::Point;

use crate::prelude::*;

/// Defines a radial area of interest from a specific point in the game world, characterized by
/// a range, center position, step angle, and an angle range. This struct is used to generate
/// `Perspective` objects that represent the observable area from the center position.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Component)]
pub struct RadialArea<P> {
    pub range: u8,
    pub center_pos: P,
    pub angle_step: usize,
    pub angle_range: (u16, u16),
}

impl<P: Point> Default for RadialArea<P> {
    fn default() -> Self {
        Self {
            range: 1,
            center_pos: P::generate(0, 0, 0),
            angle_step: 10,
            angle_range: (315, 405),
        }
    }
}

impl<P: Point> RadialArea<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_center_pos(&self) -> &P {
        &self.center_pos
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

    pub fn with_center_pos(self, center_pos: P) -> Self {
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
impl<P: RayCastingPoint> From<RadialArea<P>> for Perspective<P> {
    fn from(radial_area: RadialArea<P>) -> Self {
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
            center_pos.x() as f32,
            center_pos.y() as f32,
            center_pos.z() as f32,
        );

        Perspective::new(
            center_pos
                .tiles_on_arc_circumference(range, start_angle, end_angle, angle_step)
                .into_iter()
                .map(|arc_tile| {
                    let sub = Vec3::new(
                        arc_tile.x() as f32 - center_pos.x() as f32,
                        arc_tile.y() as f32 - center_pos.y() as f32,
                        arc_tile.z() as f32 - center_pos.z() as f32,
                    );

                    let ray_cast =
                        RayCast3d::from_ray(Ray3d::new(center_pos_vec3, sub), range as f32);
                    (ray_cast, center_pos.draw_line_to(arc_tile))
                })
                .collect::<Vec<_>>(),
        )
    }
}
