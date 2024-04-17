use bevy::prelude::{Deref, DerefMut, Event};
use std::path::PathBuf;

mod assets;
pub use assets::*;

mod camera;
pub use camera::*;

mod cursor;
pub use cursor::*;

mod gui;
pub use gui::*;

mod palette;
pub use palette::*;

mod drawing;
pub use drawing::*;

mod hud;
pub use hud::*;

mod inputs;
pub use inputs::*;

#[derive(Event, Debug, Clone, Default, Deref, DerefMut)]
pub struct ExportMap(pub PathBuf);

#[derive(Event, Debug, Clone, Default, Deref, DerefMut)]
pub struct LoadMap(pub PathBuf);
