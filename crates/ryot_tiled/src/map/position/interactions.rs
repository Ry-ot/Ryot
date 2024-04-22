use crate::prelude::*;
use std::collections::HashSet;
use std::f32::consts::PI;

impl TilePosition {
    pub fn distance(self, other: Self) -> f32 {
        self.truncate()
            .as_vec2()
            .distance(other.truncate().as_vec2())
    }

    pub fn direction_to(self, other: Self) -> OrdinalDirection {
        (other - self).into()
    }

    /// Generates a straight line from `self` to `end` using Bresenham's line algorithm.
    /// This algorithm determines the points of an n-dimensional raster that should be selected
    /// to form a close approximation to a straight line between two points.
    pub fn bresenhams_line(self, end: Self) -> Vec<TilePosition> {
        let mut points = Vec::new();

        let dx = (end.x - self.x).abs();
        let sx = if self.x < end.x { 1 } else { -1 };
        let dy = -(end.y - self.y).abs();
        let sy = if self.y < end.y { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = self.x;
        let mut y = self.y;
        loop {
            points.push(TilePosition::new(x, y, self.z));
            if x == end.x && y == end.y {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }

        points
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

        for pos in self.bresenhams_line(target) {
            if !positions.contains(&pos) {
                return false;
            }
        }

        true
    }

    /// Calculates the positions on the circumference of an arc with the given parameters.
    /// This function is useful for creating fields of view or effect areas in game mechanics.
    pub fn tiles_on_arc_circumference(
        self,
        radius: u8,
        start_angle_deg: u16,
        end_angle_deg: u16,
        angle_step: usize,
    ) -> Vec<TilePosition> {
        if angle_step == 0 || radius == 0 {
            return vec![];
        }

        if start_angle_deg == end_angle_deg {
            return vec![];
        }

        let mut tiles = Vec::new();

        for angle_deg in (start_angle_deg..=end_angle_deg).step_by(angle_step) {
            let angle = (angle_deg as f32).to_radians();
            let dx = (angle.cos() * radius as f32).round() as i32;
            let dy = (angle.sin() * radius as f32).round() as i32;
            tiles.push(TilePosition::new(self.x + dx, self.y + dy, self.z));
        }

        tiles.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
        tiles.dedup();

        tiles
    }

    pub fn get_angle_between(&self, target: Self) -> u16 {
        let dx = target.x - self.x;
        let dy = target.y - self.y;

        let angle = (dy as f32).atan2(dx as f32) * (180.0 / PI);

        if angle < 0.0 {
            (angle + 360.0) as u16
        } else {
            angle as u16
        }
    }
}
