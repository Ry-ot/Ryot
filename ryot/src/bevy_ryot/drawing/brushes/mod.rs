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

pub trait SelectableTool {
    fn name(&self) -> &str;
    fn icon(&self) -> ImageSource {
        include_svg!(
            r##"
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="#fff6c2" viewBox="0 0 256 256"><path d="M232,32a8,8,0,0,0-8-8c-44.08,0-89.31,49.71-114.43,82.63A60,60,0,0,0,32,164c0,30.88-19.54,44.73-20.47,45.37A8,8,0,0,0,16,224H92a60,60,0,0,0,57.37-77.57C182.3,121.31,232,76.08,232,32ZM124.42,113.55q5.14-6.66,10.09-12.55A76.23,76.23,0,0,1,155,121.49q-5.9,4.94-12.55,10.09A60.54,60.54,0,0,0,124.42,113.55Zm42.7-2.68a92.57,92.57,0,0,0-22-22c31.78-34.53,55.75-45,69.9-47.91C212.17,55.12,201.65,79.09,167.12,110.87Z"></path></svg>
                "##
        )
    }
    fn button(&self) -> egui::ImageButton {
        egui::ImageButton::new(self.icon())
    }
}

pub trait BrushItem: PartialEq + Copy + Clone {
    fn from_position(original: Self, pos: TilePosition) -> Self;
    fn get_position(&self) -> TilePosition;
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct Brush<E: BrushItem> {
    func: fn(size: i32, center: E) -> Vec<E>,
}

impl<E: BrushItem> Brush<E> {
    pub fn new(func: fn(i32, E) -> Vec<E>) -> Self {
        Self { func }
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

    pub fn get_index<B>(&self, target_brush: B) -> Option<usize>
    where
        B: Into<Brush<E>>,
    {
        let target_brush = target_brush.into();
        self.0.iter().position(|brush| *brush == target_brush)
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
