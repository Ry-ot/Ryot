use bevy_app::{App, First};
use bevy_ecs::prelude::*;
use bevy_time::*;
use bevy_utils::tracing::warn;
use std::any::type_name;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[derive(Component, Clone, Debug)]
pub struct Cooldown<M>(Timer, PhantomData<M>);

impl<M> Cooldown<M> {
    pub fn from_seconds(seconds: f32) -> Self {
        Self(
            Timer::from_seconds(seconds, TimerMode::Repeating),
            PhantomData,
        )
    }
}

impl<M> Default for Cooldown<M> {
    fn default() -> Self {
        Self::from_seconds(1.)
    }
}

impl<M> Deref for Cooldown<M> {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M> DerefMut for Cooldown<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait CooldownApp {
    fn add_cooldown<M: crate::ThreadSafe>(&mut self) -> &mut Self;
}

impl CooldownApp for App {
    fn add_cooldown<M: crate::ThreadSafe>(&mut self) -> &mut Self {
        self.add_systems(First, cooldown_system::<M>);
        self
    }
}

pub fn cooldown_system<M: crate::ThreadSafe>(time: Res<Time>, mut query: Query<&mut Cooldown<M>>) {
    query.par_iter_mut().for_each(|mut cooldown| {
        cooldown.tick(time.delta());
    });
}

pub fn is_valid_cooldown_for_entity<M: crate::ThreadSafe>(
    entity: &Entity,
    query: &Query<&Cooldown<M>>,
) -> bool {
    query.get(*entity).map_or_else(
        |_| {
            warn!("Cooldown<M> not configured for M = {}", type_name::<M>());
            true
        },
        |cooldown| cooldown.just_finished(),
    )
}
