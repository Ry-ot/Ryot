/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */

use bevy::app::App;
use bevy::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;

#[derive(Resource, Deref, DerefMut)]
pub struct EventSender<T>(pub Sender<T>);

#[derive(Resource, Deref, DerefMut)]
struct EventReceiver<T>(Mutex<Receiver<T>>);

pub trait AsyncEventsExtension {
    fn add_async_event<T: Event>(&mut self) -> &mut Self;
}

impl AsyncEventsExtension for App {
    fn add_async_event<T: Event>(&mut self) -> &mut Self {
        let (sender, receiver) = channel::<T>();

        assert!(
            !self.world.contains_resource::<EventReceiver<T>>(),
            "this event channel is already initialized",
        );

        self.add_event::<T>()
            .add_systems(Update, channel_to_event::<T>)
            .insert_resource(EventSender(sender))
            .insert_resource(EventReceiver(Mutex::new(receiver)));

        self
    }
}

fn channel_to_event<T: Event>(receiver: Res<EventReceiver<T>>, mut writer: EventWriter<T>) {
    // this should be the only system working with the receiver,
    // thus we always expect to get this lock
    let events = receiver.lock().expect("unable to acquire mutex lock");

    writer.send_batch(events.try_iter());
}
