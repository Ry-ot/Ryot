use bevy::prelude::*;
use ryot::prelude::drawing::*;

mod diamond;
pub use diamond::*;

mod round;
pub use round::*;

mod square;
pub use square::*;

mod systems;
pub use systems::update_brush;

#[derive(Debug, Eq, Default, PartialEq, Reflect, Copy, Clone, Hash)]
pub enum BrushType {
    Round,
    Square,
    #[default]
    Diamond,
}

#[derive(Resource, Deref, DerefMut)]
pub struct Brushes(pub Vec<Brush>);

impl Brushes {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn insert(mut self, brush: impl Into<Brush>) -> Self {
        self.push(brush.into());
        self
    }
}

impl Default for Brushes {
    fn default() -> Self {
        Self::new().insert(Diamond).insert(Round).insert(Square)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct Brush {
    func: fn(size: i32, center: DrawingBundle) -> Vec<DrawingBundle>,
}

impl Brush {
    pub fn new(func: fn(i32, DrawingBundle) -> Vec<DrawingBundle>) -> Self {
        Self { func }
    }
}

impl Default for Brush {
    fn default() -> Self {
        Diamond.into()
    }
}

impl FnMut<(i32, DrawingBundle)> for Brush {
    extern "rust-call" fn call_mut(&mut self, args: (i32, DrawingBundle)) -> Self::Output {
        (self.func)(args.0, args.1)
    }
}

impl FnOnce<(i32, DrawingBundle)> for Brush {
    type Output = Vec<DrawingBundle>;

    extern "rust-call" fn call_once(self, args: (i32, DrawingBundle)) -> Self::Output {
        (self.func)(args.0, args.1)
    }
}

impl Fn<(i32, DrawingBundle)> for Brush {
    extern "rust-call" fn call(&self, args: (i32, DrawingBundle)) -> Self::Output {
        (self.func)(args.0, args.1)
    }
}
