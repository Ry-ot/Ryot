#[cfg(feature = "bevy")]
use bevy::prelude::*;
use std::{
    fmt::{self, Formatter},
    ops::Deref,
    time::Duration,
};

use glam::{IVec3, UVec2, Vec2, Vec3};
use serde::{Deserialize, Serialize};

/// A 2d position in the tile grid. This is is not the position of the tile on
/// the screen, because it doesn't take into account the tile size. Z is used to
/// calculate the rendering order of the tile.
#[derive(Eq, PartialEq, Deserialize, Serialize, Default, Clone, Copy, Debug, Hash)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
pub struct TilePosition(pub IVec3);

#[cfg(feature = "bevy")]
#[derive(Component, Debug, Clone)]
pub struct SpriteMovement {
    pub origin: TilePosition,
    pub timer: Timer,
}

#[cfg(feature = "bevy")]
impl SpriteMovement {
    pub fn new(origin: TilePosition, duration: Duration) -> Self {
        Self {
            origin,
            timer: Timer::new(duration, TimerMode::Once),
        }
    }
}

type MovingSpriteFilter = Or<(
    Changed<TilePosition>,
    Added<TilePosition>,
    With<SpriteMovement>,
)>;
/// This system syncs the sprite position with the TilePosition.
/// Every spawned sprite has a Transform component, which is used to position the sprite on
/// the screen. However, in this library our world components are treated in terms of TilePosition.
/// So, we need to sync the sprite position with the TilePosition.
///
/// This system listen to all new and changed TilePosition components and update the Transform
/// component of the sprite accordingly, if it exist. Ideally this should run in the end of
/// the Update stage, so it can be sure that all TilePosition components have been updated.
///
/// ```rust
/// use bevy::prelude::*;
/// use ryot::sprites::position::update_sprite_position;
///
/// App::new()
///     .init_resource::<Time>()
///     .add_systems(PostUpdate, update_sprite_position)
///     .run();
/// ```
#[cfg(feature = "bevy")]
pub fn update_sprite_position(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &TilePosition,
            &Layer,
            &mut Transform,
            Option<&mut SpriteMovement>,
        ),
        MovingSpriteFilter,
    >,
    time: Res<Time>,
) {
    for (entity, tile_pos, layer, mut transform, movement) in query.iter_mut() {
        if let Some(mut movement) = movement {
            movement.timer.tick(time.delta());
            transform.translation = movement
                .origin
                .to_vec3(layer)
                .lerp(tile_pos.to_vec3(layer), movement.timer.percent());
            if movement.timer.just_finished() {
                commands.entity(entity).remove::<SpriteMovement>();
            }
        } else {
            transform.translation = tile_pos.to_vec3(layer)
        }
    }
}

impl TilePosition {
    /// The minimum possible tile position. This has to be something that when multiplied by the tile size does not overflow f32.
    pub const MIN: TilePosition = TilePosition(IVec3::new(i16::MIN as i32, i16::MIN as i32, 0));
    /// The maximum possible tile position. This has to be something that when multiplied by the tile size does not overflow f32.
    pub const MAX: TilePosition = TilePosition(IVec3::new(i16::MAX as i32, i16::MAX as i32, 0));

    pub const ZERO: TilePosition = TilePosition(IVec3::ZERO);

    const BOTTOM_RIGHT_OFFSET: Vec2 = Vec2::new(0., -1.);

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(IVec3::new(x, y, z))
    }

    pub fn with_z(self, z: i32) -> Self {
        Self(self.0.truncate().extend(z))
    }

    pub fn is_valid(self) -> bool {
        self.clamp(Self::MIN.0, Self::MAX.0).truncate() == self.truncate()
    }

    pub fn distance(self, other: Self) -> f32 {
        self.truncate()
            .as_vec2()
            .distance(other.truncate().as_vec2())
    }

    pub fn to_vec3(self, layer: &Layer) -> Vec3 {
        let pos = Vec2::from(self);
        let weight = u16::MAX as f32;

        // z for 2d sprites define the rendering order, for 45 degrees top-down
        // perspective we always want right bottom items to be drawn on top.
        // Calculations must be done in f32 otherwise decimals are lost.
        pos.extend(layer.z() as f32 + 1. + pos.x / weight - pos.y / weight)
    }
}

impl Deref for TilePosition {
    type Target = IVec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for TilePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<Vec2> for TilePosition {
    fn from(screen_pos: Vec2) -> Self {
        Self(
            ((screen_pos - TilePosition::BOTTOM_RIGHT_OFFSET) / tile_size().as_vec2())
                .ceil()
                .as_ivec2()
                .extend(0),
        )
    }
}

#[cfg(feature = "bevy")]
impl From<Transform> for TilePosition {
    fn from(transform: Transform) -> Self {
        transform.translation.truncate().into()
    }
}

#[cfg(feature = "bevy")]
impl From<&Transform> for TilePosition {
    fn from(transform: &Transform) -> Self {
        TilePosition::from(*transform)
    }
}

impl From<TilePosition> for Vec2 {
    fn from(tile_pos: TilePosition) -> Self {
        (tile_pos.as_vec3().truncate() + TilePosition::BOTTOM_RIGHT_OFFSET) * tile_size().as_vec2()
    }
}

impl From<&TilePosition> for Vec2 {
    fn from(tile_pos: &TilePosition) -> Self {
        Vec2::from(*tile_pos)
    }
}

/// This enum defines the layers that composes a game.
/// The base layers are defined in the enum and custom layers can be added.
/// The layers are used to define the rendering order of the tiles.
/// There are two types of layers: dynamic and static.
///     - Dynamic layers are used for elements that can change their rendering order
///     depending on their attributes like position or floor. E.g.: creatures, items, effects.
///    - Static layers are used for elements that have a fixed rendering order and are always
///    rendered on top of dynamic layers. E.g.: mouse, grid, ui elements.
///
/// Static layers are separated from dynamic layers by the StaticLowerBound layer.
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
pub enum Layer {
    #[default]
    Items,
    Top,
    Bottom,
    Ground,
    Creature,
    Effect,
    StaticLowerBound,
    Max,
    Custom(i32),
}

impl Layer {
    /// Z is calculated based on floor position and layer.
    /// The base Z is floor * 100 and the layer adds an offset.
    /// The offset is used to calculate the rendering order of the tile.
    /// Tiles with higher Z are rendered on top of tiles with lower Z.
    /// The tile Z for the game is always the floor (Z / 100).floor().
    ///
    /// We leave a gap of 10 between layers to allow for more layers to be added
    /// in the future and to make it possible to place custom layers between
    /// the default ones.
    pub fn z(&self) -> i32 {
        match self {
            Self::Top => 90,
            Self::Bottom => 80,
            Self::Effect => 60,
            Self::Creature => 40,
            Self::Items => 20,
            Self::Ground => 0,
            Self::StaticLowerBound => 900,
            Self::Max => 999,
            Self::Custom(z) => *z,
        }
    }
}

#[cfg(not(test))]
use crate::CONTENT_CONFIG;

#[cfg(not(test))]
pub fn tile_size() -> UVec2 {
    CONTENT_CONFIG.sprite_sheet.tile_size
}

#[cfg(test)]
pub fn tile_size() -> UVec2 {
    UVec2::new(32, 32)
}
