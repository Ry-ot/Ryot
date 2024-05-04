use glam::Vec2;
use std::f32::consts::PI;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Point: Eq + Hash + Copy + Clone + Debug {
    fn generate(x: i32, y: i32, z: i32) -> Self;
    fn coordinates(&self) -> (i32, i32, i32);

    fn x(&self) -> i32 {
        self.coordinates().0
    }

    fn y(&self) -> i32 {
        self.coordinates().1
    }

    fn z(&self) -> i32 {
        self.coordinates().2
    }

    fn distance_2d(&self, other: &Self) -> f32 {
        Vec2::new(self.x() as f32, self.y() as f32)
            .distance(Vec2::new(other.x() as f32, other.y() as f32))
    }

    /// Generates a straight line from `self` to `end` using Bresenham's line algorithm.
    /// This algorithm determines the points of an n-dimensional raster that should be selected
    /// to form a close approximation to a straight line between two points.
    fn draw_line_to(&self, end: Self) -> Vec<Self> {
        let mut points = Vec::new();

        let dx = (end.x() - self.x()).abs();
        let dy = -(end.y() - self.y()).abs();
        let sx = if self.x() < end.x() { 1 } else { -1 };
        let sy = if self.y() < end.y() { 1 } else { -1 };

        let mut err = dx + dy;
        let mut x = self.x();
        let mut y = self.y();

        loop {
            points.push(Self::generate(x, y, self.z()));
            if x == end.x() && y == end.y() {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                if x == end.x() {
                    break;
                }
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                if y == end.y() {
                    break;
                }
                err += dx;
                y += sy;
            }
        }

        points
    }

    /// Calculates the positions on the circumference of an arc with the given parameters.
    /// This function is useful for creating fields of view or effect areas in game mechanics.
    fn tiles_on_arc_circumference(
        self,
        radius: u8,
        start_angle_deg: u16,
        end_angle_deg: u16,
        angle_step: usize,
    ) -> Vec<Self> {
        if angle_step == 0 || radius == 0 || start_angle_deg == end_angle_deg {
            return vec![];
        }

        let angle_step = angle_step as u16;

        let mut tiles = Vec::new();
        let mut angle_deg = start_angle_deg;

        while angle_deg <= end_angle_deg {
            let angle = (angle_deg as f32).to_radians();
            let dx = (angle.cos() * radius as f32).round() as i32;
            let dy = (angle.sin() * radius as f32).round() as i32;
            tiles.push(Self::generate(self.x() + dx, self.y() + dy, self.z()));
            angle_deg = angle_deg.saturating_add(angle_step);
        }

        // Ensure the last point is always included
        if angle_deg != start_angle_deg && angle_deg - angle_step < end_angle_deg {
            let angle = (end_angle_deg as f32).to_radians();
            let dx = (angle.cos() * radius as f32).round() as i32;
            let dy = (angle.sin() * radius as f32).round() as i32;
            tiles.push(Self::generate(self.x() + dx, self.y() + dy, self.z()));
        }

        tiles.sort_by(|a, b| a.x().cmp(&b.x()).then(a.y().cmp(&b.y())));
        tiles.dedup();

        tiles
    }

    fn get_angle_between(&self, target: Self) -> u16 {
        let dx = target.x() - self.x();
        let dy = target.y() - self.y();

        let angle = (dy as f32).atan2(dx as f32) * (180.0 / PI);

        if angle < 0.0 {
            (angle + 360.0) as u16
        } else {
            angle as u16
        }
    }
}
