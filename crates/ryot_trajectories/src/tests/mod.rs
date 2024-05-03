use crate::prelude::RadialArea;
use crate::Pos;
use derive_more::{Deref, DerefMut};
use ryot_core::prelude::Point;

mod traversal_test;

impl quickcheck::Arbitrary for RadialArea<Pos> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        RadialArea::default()
            .with_range(u8::arbitrary(g) % 10 + 1)
            .with_angle_step(usize::arbitrary(g) % 90 + 1)
            .with_angle_range((u16::arbitrary(g), u16::arbitrary(g)))
            .with_center_pos(Pos::generate(
                i8::arbitrary(g) as i32,
                i8::arbitrary(g) as i32,
                0,
            ))
    }
}

#[derive(Copy, Clone, Debug, Deref, DerefMut, Hash, Eq, PartialEq)]
struct Pos3x3(Pos);

impl Point for Pos3x3 {
    fn generate(x: i32, y: i32, z: i32) -> Self {
        Pos3x3(Pos::generate(x, y, z))
    }

    fn coordinates(&self) -> (i32, i32, i32) {
        self.0.coordinates()
    }
}

impl quickcheck::Arbitrary for Pos {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Pos::generate(
            i32::arbitrary(g) % i16::MAX as i32,
            i32::arbitrary(g) % i16::MAX as i32,
            i32::arbitrary(g) % i16::MAX as i32,
        )
    }
}

impl quickcheck::Arbitrary for Pos3x3 {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Pos3x3(Pos::generate(
            i32::arbitrary(g) % 3,
            i32::arbitrary(g) % 3,
            i32::arbitrary(g) % 3,
        ))
    }
}
