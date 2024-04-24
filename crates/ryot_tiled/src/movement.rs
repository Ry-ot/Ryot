use glam::Vec3;
use std::time::Duration;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
pub struct SpriteMovement {
    pub origin: Vec3,
    pub destination: Vec3,
    #[cfg(feature = "bevy")]
    pub timer: bevy_time::Timer,
    pub despawn_on_end: bool,
}

impl SpriteMovement {
    pub fn new(origin: Vec3, destination: Vec3, duration: Duration) -> Self {
        Self {
            origin,
            destination,
            #[cfg(feature = "bevy")]
            timer: bevy_time::Timer::new(duration, bevy_time::TimerMode::Once),
            despawn_on_end: false,
        }
    }

    pub fn despawn_on_end(self, despawn_on_end: bool) -> Self {
        Self {
            despawn_on_end,
            ..self
        }
    }
}
