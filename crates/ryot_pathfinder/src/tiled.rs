use crate::prelude::*;
use ryot_tiled::prelude::*;

impl Pathable for TilePosition {
    fn generate(x: i32, y: i32, z: i32) -> Self {
        TilePosition::new(x, y, z)
    }

    fn coordinates(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }
}
