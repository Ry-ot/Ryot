use crate::prelude::TilePosition;
use bevy_app::App;
use ryot_core::game::Navigable;
use ryot_core::prelude::Flags;
use ryot_ray_casting::prelude::*;

pub trait TiledRayCastingApp {
    fn add_tiled_ray_casting<Marker: Copy + Send + Sync + 'static>(&mut self) -> &mut Self;
}

impl TiledRayCastingApp for App {
    fn add_tiled_ray_casting<Marker: Copy + Send + Sync + 'static>(&mut self) -> &mut Self {
        self.add_ray_casting::<Marker, TilePosition, Flags>()
    }
}

pub type TiledRayCasting<Marker> = RayCasting<Marker, TilePosition>;

pub fn tiled_ray_casting<Marker>(
    area: RadialArea<TilePosition>,
    condition: fn(&TiledRayCasting<Marker>, &dyn Navigable, &TilePosition) -> bool,
) -> TiledRayCasting<Marker> {
    RayCasting::<Marker, TilePosition>::new(area, condition)
}

pub fn tiled_visible_ray_casting<Marker>(
    area: RadialArea<TilePosition>,
) -> TiledRayCasting<Marker> {
    visible_ray_casting::<Marker, TilePosition>(area)
}

pub fn tiled_walkable_ray_casting<Marker>(
    area: RadialArea<TilePosition>,
) -> TiledRayCasting<Marker> {
    walkable_ray_casting::<Marker, TilePosition>(area)
}
