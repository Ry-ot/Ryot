use crate::position::TilePosition;
use bevy::prelude::{Deref, DerefMut, Resource};

mod diamond;
pub use diamond::*;

mod round;
use egui::ImageSource;
pub use round::*;

mod square;
pub use square::*;

mod random;
pub use random::*;

#[macro_export]
macro_rules! include_svg {
    ($svg_content: literal) => {
        egui::ImageSource::Bytes {
            uri: ::std::borrow::Cow::Borrowed(concat!("bytes://", $svg_content, ".svg")),
            bytes: egui::load::Bytes::Static($svg_content.as_bytes()),
        }
    };
}

pub trait BrushItem: PartialEq + Copy + Clone {
    fn from_position(original: Self, pos: TilePosition) -> Self;
    fn get_position(&self) -> TilePosition;
}

#[derive(Clone)]
pub struct Brush<E: BrushItem> {
    func: fn(size: i32, center: E) -> Vec<E>,
    icon: ImageSource<'static>,
    name: String,
}

impl<E: BrushItem> Brush<E> {
    pub fn new(func: fn(i32, E) -> Vec<E>, name: &str, icon: ImageSource<'static>) -> Self {
        Self {
            func,
            icon,
            name: name.into(),
        }
    }

    pub fn button(&self) -> egui::ImageButton {
        egui::ImageButton::new(self.icon.clone())
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<E: BrushItem> Default for Brush<E> {
    fn default() -> Self {
        Diamond.into()
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct Brushes<E: BrushItem + std::cmp::PartialEq>(pub Vec<Brush<E>>);

impl<E: BrushItem + std::cmp::PartialEq> Default for Brushes<E> {
    fn default() -> Self {
        Self::new()
            .insert(Diamond)
            .insert(Round)
            .insert(Square)
            .insert(Random)
    }
}

impl<E: BrushItem> Brushes<E> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn insert(mut self, brush: impl Into<Brush<E>>) -> Self {
        self.push(brush.into());
        self
    }

    pub fn next_index(&self, current: usize) -> usize {
        match current + 1 {
            next if next < self.len() => next,
            _ => 0,
        }
    }
}

impl<E: BrushItem> FnOnce<(i32, E)> for Brush<E> {
    type Output = Vec<E>;

    extern "rust-call" fn call_once(self, args: (i32, E)) -> Self::Output {
        (self.func)(args.0, args.1)
    }
}

impl<E: BrushItem> FnMut<(i32, E)> for Brush<E> {
    extern "rust-call" fn call_mut(&mut self, args: (i32, E)) -> Self::Output {
        (self.func)(args.0, args.1)
    }
}

impl<E: BrushItem> Fn<(i32, E)> for Brush<E> {
    extern "rust-call" fn call(&self, args: (i32, E)) -> Self::Output {
        (self.func)(args.0, args.1)
    }
}

impl<E: BrushItem> FnOnce<(usize, i32, E)> for Brushes<E> {
    type Output = Vec<E>;

    extern "rust-call" fn call_once(self, args: (usize, i32, E)) -> Self::Output {
        match self.0.get(args.0) {
            Some(brush) => brush.call_once((args.1, args.2)),
            None => Vec::new(),
        }
    }
}

impl<E: BrushItem> FnMut<(usize, i32, E)> for Brushes<E> {
    extern "rust-call" fn call_mut(&mut self, args: (usize, i32, E)) -> Self::Output {
        match self.0.get(args.0) {
            Some(mut brush) => brush.call_mut((args.1, args.2)),
            None => Vec::new(),
        }
    }
}

impl<E: BrushItem> Fn<(usize, i32, E)> for Brushes<E> {
    extern "rust-call" fn call(&self, args: (usize, i32, E)) -> Self::Output {
        match self.0.get(args.0) {
            Some(brush) => brush.call((args.1, args.2)),
            None => Vec::new(),
        }
    }
}
