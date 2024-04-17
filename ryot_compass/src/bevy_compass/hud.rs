use ryot::prelude::*;

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HudLayers {
    /// 0 is reserved for the grid which is provided by the root ryot library
    _Grid,
    #[default]
    BrushPreview,
    Cursor,
}

impl From<HudLayers> for Layer {
    fn from(layer: HudLayers) -> Self {
        Layer::Hud(layer as Order)
    }
}
