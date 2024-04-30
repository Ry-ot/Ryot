use bevy::math::Vec2;
use bevy::prelude::Component;
use ryot_core::prelude::Point;
use ryot_derive::Pathable;

/// This is an example on how to use the Pathable trait to define a point for pathfinding.
/// Pos is a simple 3D point with x, y, and z coordinates.
#[derive(Eq, PartialEq, Component, Pathable, Default, Clone, Copy, Debug, Hash)]
pub struct Pos(i32, i32, i32);

impl From<Pos> for Vec2 {
    fn from(pos: Pos) -> Self {
        Vec2::new(pos.0 as f32, pos.1 as f32 - 1.) * 32.
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
