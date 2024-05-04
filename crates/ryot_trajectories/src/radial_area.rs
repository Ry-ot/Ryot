//! This module introduces the concepts of `RadialArea` a primary representation of an area of
//! interest in the game world, that can be used for determining trajectories. It's the main way
//! of generating the [Perspective] from a spectator in a given position.
//!
//! Radial area tries to represent an area based on angles, like a circle or a sector, with a
//! predefined range and center position.
use bevy_math::bounding::RayCast3d;
use bevy_math::Ray3d;
use glam::Vec3;
use ryot_core::prelude::Point;

use crate::prelude::*;

/// Defines a radial area of interest from a specific point in the game world, characterized by
/// a range, center position, step angle, and an angle range. This struct is used to generate
/// [Perspective] objects that represent the observable area from the center position.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RadialArea<P> {
    pub range: u8,
    pub center_pos: P,
    pub angle_step: usize,
    pub angle_range: (u16, u16),
    pub extra_rays: bool,
}

impl<P: Point> Default for RadialArea<P> {
    fn default() -> Self {
        Self {
            range: 1,
            center_pos: P::generate(0, 0, 0),
            angle_step: 10,
            angle_range: (0, 90),
            extra_rays: false,
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
        Self::default().with_angle_range((0, 360))
    }

    pub fn sector(start_angle: u16, end_angle: u16) -> Self {
        Self::default().with_angle_range((start_angle, end_angle))
    }

    pub fn with_range_and_auto_angle_step(self, range: u8) -> Self {
        Self {
            range,
            angle_step: match range {
                0..=4 => 10,
                5..=8 => 5,
                _ => 1,
            },
            extra_rays: match range {
                0..=11 => false,
                _ => true,
            },
            ..self
        }
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

    pub fn get_rays_to_tile(&self, arc_tile: &P) -> Vec<Vec3> {
        let main_sub = Vec3::new(
            arc_tile.x() as f32 - self.center_pos.x() as f32,
            arc_tile.y() as f32 - self.center_pos.y() as f32,
            arc_tile.z() as f32 - self.center_pos.z() as f32,
        )
        .normalize();

        if !self.extra_rays {
            vec![main_sub]
        } else {
            vec![0.5, -0.5]
                .into_iter()
                .map(|angle_adjustment| {
                    let adjusted_angle = (main_sub.y.atan2(main_sub.x)
                        + (angle_adjustment as f32).to_radians())
                    .to_degrees();

                    Vec3::new(
                        adjusted_angle.to_radians().cos() * self.range as f32,
                        adjusted_angle.to_radians().sin() * self.range as f32,
                        main_sub.z,
                    )
                    .normalize()
                })
                .collect::<Vec<_>>()
        }
    }
}

/// Implements conversion from `RadialArea` to [Perspective], allowing easy creation of
/// perspective objects based on radial descriptions. This facilitates the dynamic generation
/// of visible areas based on the position and defined view angle of entities.
impl<P: TrajectoryPoint> From<RadialArea<P>> for Perspective<P> {
    fn from(radial_area: RadialArea<P>) -> Self {
        let RadialArea {
            range,
            center_pos,
            angle_step,
            angle_range: (start_angle, end_angle),
            ..
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
                .filter_map(|arc_tile| {
                    let rays = radial_area
                        .get_rays_to_tile(&arc_tile)
                        .into_iter()
                        .filter_map(|sub| {
                            if sub == Vec3::ZERO {
                                return None;
                            }

                            Some((
                                RayCast3d::from_ray(Ray3d::new(center_pos_vec3, sub), range as f32),
                                center_pos.draw_line_to(arc_tile),
                            ))
                        })
                        .collect::<Vec<_>>();

                    Some(rays)
                })
                .flatten()
                .collect::<Vec<_>>(),
        )
    }
}
