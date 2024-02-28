//! This module provides a way to send events between systems asynchronously.
//! It's useful to send events between threads that perform asynchronous tasks, such as loading
//! content from disk or loading sprites from a sprite sheet before rendering.
use bevy::app::App;
use bevy::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;

/// A resource that emits asynchronous events of a given type.
/// It's a bevy friendly wrapper around a `std::sync::mpsc::Sender`.
#[derive(Resource, Deref, DerefMut)]
pub struct EventSender<T>(pub Sender<T>);

/// A resource that receives asynchronous events of a given type.
/// It's a bevy friendly wrapper around a `std::sync::mpsc::Receiver`.
/// It's wrapped in a `Mutex` to allow multiple systems to safely access it.
#[derive(Resource, Deref, DerefMut)]
struct EventReceiver<T>(Mutex<Receiver<T>>);

/// A trait to add an asynchronous event to an App.
pub trait AsyncEventApp {
    fn add_async_event<T: Event>(&mut self) -> &mut Self;
}

/// Sets up the necessary systems to receive events of type `T` asynchronously
/// within Bevy's ecosystem. It sets up both sender and receiver resources and
/// the system that sends events from the receiver to Bevy's event system.
impl AsyncEventApp for App {
    fn add_async_event<T: Event>(&mut self) -> &mut Self {
        if self.world.contains_resource::<EventReceiver<T>>() {
            return self;
        }

        let (sender, receiver) = channel::<T>();

        self.add_event::<T>()
            .add_systems(Update, channel_to_event::<T>)
            .insert_resource(EventSender(sender))
            .insert_resource(EventReceiver(Mutex::new(receiver)));

        self
    }
}

/// A system that sends events from the receiver to Bevy's event system.
/// Converts the asynchronous event into a bevy event and sends it to the event system.
fn channel_to_event<T: Event>(receiver: Res<EventReceiver<T>>, mut writer: EventWriter<T>) {
    // this should be the only system working with the receiver,
    // thus we always expect to get this lock
    let events = receiver.lock().expect("unable to acquire mutex lock");

    writer.send_batch(events.try_iter());
}
