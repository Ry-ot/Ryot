use crate::prelude::*;
use glam::IVec3;
use ryot_core::prelude::Point;
use std::collections::HashSet;

impl TilePosition {
    pub fn distance(self, other: &Self) -> f32 {
        self.truncate()
            .as_vec2()
            .distance(other.truncate().as_vec2())
    }

    pub fn direction_to(self, other: Self) -> OrdinalDirection {
        (other - self).into()
    }

    /// Checks if there's a direct path between `self` and `target` without any interruptions.
    /// Utilizes the Bresenham's line algorithm to compute the straight line path and checks
    /// if all positions along the path are contained within a specified set of positions.
    pub fn is_directly_connected(
        self,
        target: TilePosition,
        positions: &HashSet<TilePosition>,
    ) -> bool {
        if self.z != target.z {
            return false;
        }

        for pos in self.draw_line_to(target) {
            if !positions.contains(&pos) {
                return false;
            }
        }

        true
    }

    pub fn get_surroundings(&self) -> Vec<TilePosition> {
        let mut surroundings = Vec::with_capacity(8);

        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }

                surroundings.push(TilePosition(self.0 + IVec3::new(x, y, 0)));
            }
        }

        surroundings
    }
}
