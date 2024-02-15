#[cfg(feature = "bevy")]
use bevy::prelude::*;
use std::hash::Hash;
use std::{
    fmt::{self, Formatter},
    ops::Deref,
    time::Duration,
};

use crate::layer::Layer;
use derive_more::{Add, Sub};
use glam::{IVec3, UVec2, Vec2, Vec3};
use serde::{Deserialize, Serialize};

/// A 2d position in the tile grid. This is is not the position of the tile on
/// the screen, because it doesn't take into account the tile size. Z is used to
/// calculate the rendering order of the tile.
#[derive(Eq, PartialEq, Deserialize, Serialize, Default, Clone, Copy, Debug, Hash, Add, Sub)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
pub struct TilePosition(pub IVec3);

#[cfg(feature = "bevy")]
#[derive(Component, Debug, Clone)]
pub struct SpriteMovement {
    pub origin: TilePosition,
    pub timer: Timer,
    pub delete_on_end: bool,
}

#[cfg(feature = "bevy")]
impl SpriteMovement {
    pub fn new(origin: TilePosition, duration: Duration) -> Self {
        Self {
            origin,
            timer: Timer::new(duration, TimerMode::Once),
            delete_on_end: false,
        }
    }

    pub fn delete_on_end(self, delete_on_end: bool) -> Self {
        Self {
            delete_on_end,
            ..self
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

        pos.extend(match layer {
            // Static objects are drawn on top of the ground, so we don't need to tweak the Z based
            // on the tile position.
            Layer::Fixed(z) => *z as f32,
            // z for 2d sprites define the rendering order, for 45 degrees top-down
            // perspective we always want right bottom items to be drawn on top.
            // Calculations must be done in f32 otherwise decimals are lost.
            Layer::TopDown45(z) => *z as f32 + 1. + pos.x / weight - pos.y / weight,
        })
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

#[derive(Eq, PartialEq, Component, Reflect, Default, Clone, Copy, Debug)]
pub struct Edges {
    pub min: TilePosition,
    pub max: TilePosition,
}

impl fmt::Display for Edges {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Edges({}, {})", self.min, self.max)
    }
}

impl Edges {
    pub fn from_transform_and_projection(
        transform: &Transform,
        projection: &OrthographicProjection,
    ) -> Self {
        let visible_width = projection.area.max.x - projection.area.min.x;
        let visible_height = projection.area.max.y - projection.area.min.y;

        // Adjust by the camera scale if necessary
        let visible_width = visible_width * transform.scale.x;
        let visible_height = visible_height * transform.scale.y;

        // Calculate boundaries based on the camera's position
        let camera_position = transform.translation;
        let left_bound = camera_position.x - visible_width / 2.0;
        let right_bound = camera_position.x + visible_width / 2.0;
        let bottom_bound = camera_position.y - visible_height / 2.0;
        let top_bound = camera_position.y + visible_height / 2.0;

        Self {
            min: TilePosition::from(Vec2::new(left_bound, bottom_bound)),
            max: TilePosition::from(Vec2::new(right_bound, top_bound)),
        }
    }

    pub fn size(&self) -> IVec2 {
        IVec2::new(self.max.x - self.min.x, self.max.y - self.min.y)
    }

    pub fn area(&self) -> u32 {
        (self.size().x * self.size().y).unsigned_abs()
    }
}

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
                if movement.delete_on_end {
                    commands.entity(entity).despawn_recursive();
                } else {
                    commands.entity(entity).remove::<SpriteMovement>();
                }
            }
        } else {
            transform.translation = tile_pos.to_vec3(layer)
        }
    }
}

type MovingSpriteFilter = Or<(
    Changed<TilePosition>,
    Added<TilePosition>,
    With<SpriteMovement>,
)>;
