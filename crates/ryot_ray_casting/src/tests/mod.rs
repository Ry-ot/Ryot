use crate::prelude::RadialArea;
use derive_more::{Deref, DerefMut};
use glam::IVec3;
use ryot_tiled::prelude::*;

mod traversal_test;

impl quickcheck::Arbitrary for RadialArea {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        RadialArea::default()
            .with_range(u8::arbitrary(g) % 10 + 1)
            .with_angle_step(usize::arbitrary(g) % 90 + 1)
            .with_angle_range((u16::arbitrary(g), u16::arbitrary(g)))
            .with_center_pos(TilePosition::new(
                i8::arbitrary(g) as i32,
                i8::arbitrary(g) as i32,
                0,
            ))
    }
}

#[derive(Copy, Clone, Debug, Deref, DerefMut, Eq, PartialEq)]
struct TilePosition3x3(TilePosition);

impl TilePosition3x3 {
    pub const ZERO: TilePosition3x3 = TilePosition3x3(TilePosition(IVec3::ZERO));
}

impl quickcheck::Arbitrary for TilePosition3x3 {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let x = i32::arbitrary(g) % 3;
        let y = i32::arbitrary(g) % 3;
        let z = i32::arbitrary(g) % 3;

        TilePosition3x3(TilePosition::new(x, y, z))
    }
}
