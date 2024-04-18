use crate::appearances::{Flags, Frame, VisualElement};
use ryot_core::is_true;
use ryot_grid::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct PreparedAppearance {
    pub id: u32,
    pub name: String,
    pub layer: Layer,
    pub main_sprite_id: u32,
    pub frame_groups: Vec<Frame>,
    pub flags: Option<Flags>,
}

impl From<VisualElement> for Option<PreparedAppearance> {
    fn from(item: VisualElement) -> Self {
        let id = item.id?;
        let main_frame = item.frames.first()?.clone();
        let main_sprite_id = *main_frame.sprite_info?.sprite_ids.first()?;

        Some(PreparedAppearance {
            id: item.id?,
            name: item.name.unwrap_or(id.to_string()),
            layer: appearance_flags_to_layer(item.flags.clone()),
            main_sprite_id,
            frame_groups: item.frames.clone(),
            flags: item.flags.clone(),
        })
    }
}

pub fn appearance_flags_to_layer(flags: Option<Flags>) -> Layer {
    match flags {
        Some(flags) if is_true(flags.is_top) => Layer::Top,
        Some(flags) if flags.ground.is_some() => Layer::Ground,
        Some(flags) if is_true(flags.is_ground) => Layer::Ground,
        Some(flags) if is_true(flags.is_edge) => Layer::Edge,
        _ => Layer::Bottom(BottomLayer::new(0, RelativeLayer::Object)),
    }
}
