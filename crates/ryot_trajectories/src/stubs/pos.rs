use bevy_ecs::prelude::Component;
use bevy_math::bounding::Aabb3d;
use bevy_math::*;
use ryot_core::prelude::Point;

/// This is an example on how to use the Pathable trait to define a point for pathfinding.
/// Pos is a simple 3D point with x, y, and z coordinates.
#[derive(Eq, PartialEq, Ord, PartialOrd, Component, Default, Clone, Copy, Debug, Hash)]
pub struct Pos(i32, i32, i32);

impl From<Pos> for Vec2 {
    fn from(pos: Pos) -> Self {
        Vec2::new(pos.0 as f32, pos.1 as f32 - 1.) * 32.
    }
}

impl From<Pos> for Vec3 {
    fn from(pos: Pos) -> Self {
        Vec3::new(pos.0 as f32, pos.1 as f32, pos.2 as f32)
    }
}

/// We are implementing the Pathable trait for Pos, where the coordinates are generated based on
/// the x, y, and z values, and the coordinates are returned as a tuple.
///
/// There is also a default implementation for path_to, which is focused on 2D pathfinding, which
/// for this example is sufficient. If you need 3D pathfinding or other scenarios, you can override
/// this implementation.
impl Point for Pos {
    fn generate(x: i32, y: i32, z: i32) -> Self {
        Pos(x, y, z)
    }

    fn coordinates(&self) -> (i32, i32, i32) {
        (self.0, self.1, self.2)
    }
}

impl From<Pos> for Aabb3d {
    fn from(tile_pos: Pos) -> Self {
        let vec3 = Vec3::new(
            tile_pos.x() as f32,
            tile_pos.y() as f32,
            tile_pos.z() as f32,
        );
        Aabb3d::new(vec3, Vec3::new(0.35, 0.35, 0.))
    }
}
