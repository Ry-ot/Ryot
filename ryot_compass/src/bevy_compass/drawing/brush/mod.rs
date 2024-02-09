use bevy::prelude::*;
use ryot::prelude::drawing::*;

mod diamond;
pub use diamond::*;

mod round;
pub use round::*;

mod square;
pub use square::*;

mod random;
pub use random::*;
use ryot::position::TilePosition;

mod systems;
pub use systems::update_brush;

#[derive(Resource, Deref, DerefMut)]
pub struct Brushes<E: BrushItem>(pub Vec<Brush<E>>);

impl<E: BrushItem> Brushes<E> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn insert(mut self, brush: impl Into<Brush<E>>) -> Self {
        self.push(brush.into());
        self
    }
}

impl<E: BrushItem> Default for Brushes<E> {
    fn default() -> Self {
        Self::new()
            .insert(Diamond)
            .insert(Round)
            .insert(Square)
            .insert(Random)
    }
}

pub trait BrushItem: Copy + Clone {
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

impl Default for Brush<DrawingBundle> {
    fn default() -> Self {
        Diamond.into()
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

impl BrushItem for DrawingBundle {
    fn from_position(original: Self, tile_pos: TilePosition) -> Self {
        let DrawingBundle {
            layer,
            appearance,
            visibility,
            tile,
            ..
        } = original;

        DrawingBundle {
            layer,
            tile_pos,
            appearance,
            visibility,
            tile,
        }
    }

    fn get_position(&self) -> TilePosition {
        self.tile_pos
    }
}
