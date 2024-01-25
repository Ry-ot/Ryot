mod appearances;
pub use appearances::*;

mod async_events;
pub use async_events::*;

mod configs;
pub use configs::*;

pub mod content;

pub mod sprites;

mod palette;
pub use palette::*;

use bevy::prelude::Event;
use std::marker::PhantomData;

#[derive(Debug, Clone, Event)]
pub struct LoadAssetCommand<T> {
    pub path: String,
    _marker: PhantomData<T>,
}
