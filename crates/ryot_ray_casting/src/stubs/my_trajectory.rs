use crate::prelude::{RadialArea, Trajectory};
use crate::Pos;
use bevy_ecs::prelude::Component;
use ryot_core::game::Navigable;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Component)]
pub struct MyTrajectory<Marker>(pub RadialArea<Pos>, PhantomData<Marker>);

impl<Marker> MyTrajectory<Marker> {
    pub fn new(radial_area: RadialArea<Pos>) -> Self {
        Self(radial_area, PhantomData)
    }
}

impl<Marker: Copy + Send + Sync + 'static> Trajectory for MyTrajectory<Marker> {
    type Position = Pos;

    fn get_area(&self) -> RadialArea<Self::Position> {
        self.0
    }

    fn meets_condition(&self, nav: &impl Navigable, _: &Self::Position) -> bool {
        !nav.blocks_sight()
    }
}
