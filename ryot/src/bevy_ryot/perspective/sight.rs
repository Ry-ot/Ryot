use std::marker::PhantomData;

use bevy::prelude::*;

use crate::bevy_ryot::perspective::*;
use crate::bevy_ryot::tile_flags::TileFlags;
use crate::position::TilePosition;

/// Generic sight component that can be used to define a view point for different contexts.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
pub struct Sight<T>(pub RadialViewPoint, PhantomData<T>);

impl<T> Sight<T> {
    pub fn new(vp: RadialViewPoint) -> Self {
        Self(vp, PhantomData::<T>)
    }
}

impl<T: Copy + Send + Sync + 'static> ConditionalViewPoint for Sight<T> {
    fn get_view_point(&self) -> RadialViewPoint {
        (*self).into()
    }

    fn meets_condition(&self, flags: &TileFlags, _: &TilePosition) -> bool {
        !flags.blocks_sight
    }
}

impl<T> From<Sight<T>> for RadialViewPoint {
    fn from(sight: Sight<T>) -> Self {
        sight.0
    }
}

/// Generic path component that can be used to define a view point for different contexts.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
pub struct Path<T>(pub RadialViewPoint, PhantomData<T>);

impl<T> Path<T> {
    pub fn new(vp: RadialViewPoint) -> Self {
        Self(vp, PhantomData::<T>)
    }
}

impl<T: Copy + Send + Sync + 'static> ConditionalViewPoint for Path<T> {
    fn get_view_point(&self) -> RadialViewPoint {
        (*self).into()
    }
}

impl<T> From<Path<T>> for RadialViewPoint {
    fn from(path: Path<T>) -> Self {
        path.0
    }
}
