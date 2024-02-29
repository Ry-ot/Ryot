//! This module provides a way to send events between systems asynchronously.
//! It's useful to send events between threads that perform asynchronous tasks, such as loading
//! content from disk or loading sprites from a sprite sheet before rendering.
use std::{hash::Hasher, marker::PhantomData};

use bevy::app::App;
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

// TODO: doc.
#[derive(SystemSet)]
pub struct AsyncEventSet<T: Event>(PhantomData<T>);

impl<T: Event> std::fmt::Debug for AsyncEventSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AsyncEventSet<{}>", &std::any::type_name::<T>())
    }
}

impl<T: Event> std::hash::Hash for AsyncEventSet<T> {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        // all systems of a given type are the same
    }
}

impl<T: Event> Clone for AsyncEventSet<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Event> Copy for AsyncEventSet<T> {}

impl<T: Event> PartialEq for AsyncEventSet<T> {
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        // all systems of a given type are the same
        true
    }
}

impl<T: Event> Eq for AsyncEventSet<T> {}

/// A resource that emits asynchronous events of a given type.
/// It's a bevy friendly wrapper around a `crossbeam_channel::Sender`.
#[derive(Resource, Deref, DerefMut)]
pub struct AsyncEventSender<T: Event>(pub Sender<T>);

/// A resource that receives asynchronous events of a given type.
/// It's a bevy friendly wrapper around a `crossbeam_channel::Receiver`.
#[derive(Resource, Deref, DerefMut)]
struct AsyncEventReceiver<T: Event>(Receiver<T>);

/// A trait to add an asynchronous event to an App.
pub trait AsyncEventApp {
    fn add_async_event<T: Event>(&mut self) -> &mut Self;
}

/// Sets up the necessary systems to receive events of type `T` asynchronously
/// within Bevy's ecosystem. It sets up both sender and receiver resources and
/// the system that sends events from the receiver to Bevy's event system.
impl AsyncEventApp for App {
    fn add_async_event<T: Event>(&mut self) -> &mut Self {
        if self.world.contains_resource::<AsyncEventReceiver<T>>() {
            return self;
        }

        let (sender, receiver) = crossbeam_channel::unbounded::<T>();

        self.add_event::<T>()
            .add_systems(
                PreUpdate,
                unbounded_channel_to_event::<T>.in_set(AsyncEventSet::<T>(PhantomData)),
            )
            .insert_resource(AsyncEventSender(sender))
            .insert_resource(AsyncEventReceiver(receiver));

        self
    }
}

/// A system that sends events from the receiver to Bevy's event system.
/// Converts the asynchronous event into a bevy event and sends it to the event system.
fn unbounded_channel_to_event<T: Event>(
    receiver: Res<AsyncEventReceiver<T>>,
    mut writer: EventWriter<T>,
) {
    writer.send_batch(receiver.try_iter());
}
