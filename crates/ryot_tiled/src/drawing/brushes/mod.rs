use crate::prelude::*;
use bevy_ecs::prelude::*;
use derive_more::*;

pub mod diamond;

pub mod line;

pub mod round;

pub mod rectangle;

pub mod random;

#[cfg(feature = "egui")]
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

pub enum BrushParams<E: BrushItem> {
    Size(i32),
    Position(TilePosition),
    Element(E),
}

impl<E: BrushItem> BrushParams<E> {
    pub fn get_size(&self, center: E) -> i32 {
        let get_distance = |pos: TilePosition| center.get_position().distance(&pos).abs() as i32;

        match self {
            BrushParams::Size(size) => *size,
            BrushParams::Position(pos) => get_distance(*pos),
            BrushParams::Element(e) => get_distance(e.get_position()),
        }
    }

    pub fn get_ranges(
        &self,
        center: E,
    ) -> (std::ops::RangeInclusive<i32>, std::ops::RangeInclusive<i32>) {
        let center_pos = center.get_position();
        let get_range_for_pos = |pos: TilePosition| {
            (
                center_pos.x.min(pos.x)..=center_pos.x.max(pos.x),
                center_pos.y.min(pos.y)..=center_pos.y.max(pos.y),
            )
        };

        match self {
            BrushParams::Size(size) => (
                center_pos.x.saturating_sub(*size)..=center_pos.x.saturating_add(*size),
                center_pos.y.saturating_sub(*size)..=center_pos.y.saturating_add(*size),
            ),
            BrushParams::Position(pos) => get_range_for_pos(*pos),
            BrushParams::Element(e) => get_range_for_pos(e.get_position()),
        }
    }
}

#[derive(Clone)]
pub struct Brush<E: BrushItem> {
    func: fn(size: BrushParams<E>, center: E) -> Vec<E>,
    #[cfg(feature = "egui")]
    icon: egui::ImageSource<'static>,
    name: String,
}

impl<E: BrushItem> Brush<E> {
    pub fn new(
        func: fn(BrushParams<E>, E) -> Vec<E>,
        name: &str,
        #[cfg(feature = "egui")] icon: egui::ImageSource<'static>,
    ) -> Self {
        #[cfg(feature = "egui")]
        let brush = Self {
            func,
            icon,
            name: name.into(),
        };
        #[cfg(not(feature = "egui"))]
        let brush = Self {
            func,
            name: name.into(),
        };
        brush
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    #[cfg(feature = "egui")]
    pub fn icon(&self) -> egui::ImageSource<'static> {
        self.icon.clone()
    }
}

impl<E: BrushItem> Default for Brush<E> {
    fn default() -> Self {
        Rectangle.into()
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct Brushes<E: BrushItem + std::cmp::PartialEq>(pub Vec<Brush<E>>);

impl<E: BrushItem + std::cmp::PartialEq> Default for Brushes<E> {
    fn default() -> Self {
        Self::new()
            .insert(Rectangle)
            .insert(Diamond)
            .insert(Round)
            .insert(Line)
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

impl<E: BrushItem> FnOnce<(BrushParams<E>, E)> for Brush<E> {
    type Output = Vec<E>;

    extern "rust-call" fn call_once(self, args: (BrushParams<E>, E)) -> Self::Output {
        (self.func)(args.0, args.1)
    }
}

impl<E: BrushItem> FnMut<(BrushParams<E>, E)> for Brush<E> {
    extern "rust-call" fn call_mut(&mut self, args: (BrushParams<E>, E)) -> Self::Output {
        (self.func)(args.0, args.1)
    }
}

impl<E: BrushItem> Fn<(BrushParams<E>, E)> for Brush<E> {
    extern "rust-call" fn call(&self, args: (BrushParams<E>, E)) -> Self::Output {
        (self.func)(args.0, args.1)
    }
}

impl<E: BrushItem> FnOnce<(usize, BrushParams<E>, E)> for Brushes<E> {
    type Output = Vec<E>;

    extern "rust-call" fn call_once(self, args: (usize, BrushParams<E>, E)) -> Self::Output {
        match self.0.get(args.0) {
            Some(brush) => brush.call_once((args.1, args.2)),
            None => Vec::new(),
        }
    }
}

impl<E: BrushItem> FnMut<(usize, BrushParams<E>, E)> for Brushes<E> {
    extern "rust-call" fn call_mut(&mut self, args: (usize, BrushParams<E>, E)) -> Self::Output {
        match self.0.get(args.0) {
            Some(mut brush) => brush.call_mut((args.1, args.2)),
            None => Vec::new(),
        }
    }
}

impl<E: BrushItem> Fn<(usize, BrushParams<E>, E)> for Brushes<E> {
    extern "rust-call" fn call(&self, args: (usize, BrushParams<E>, E)) -> Self::Output {
        match self.0.get(args.0) {
            Some(brush) => brush.call((args.1, args.2)),
            None => Vec::new(),
        }
    }
}
