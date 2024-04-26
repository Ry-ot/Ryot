pub mod layout;
pub mod sprite_sheet;

#[derive(Clone, PartialEq, Default, Debug)]
pub struct SpriteInfo {
    pub ids: Vec<u32>,
    pub layers: u32,
    pub pattern_width: u32,
    pub pattern_height: u32,
    pub pattern_depth: u32,
    pub animation: Option<Animation>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Animation {
    pub start_phase: u32,
    pub synchronized: bool,
    pub is_start_random: bool,
    pub phases: Vec<(u32, u32)>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Component))]
#[repr(usize)]
pub enum FrameGroup {
    #[default]
    Idle = 0,
    Moving = 1,
    Initial = 2,
}

impl FrameGroup {
    pub fn set_moving(&mut self, moving: bool) {
        *self = match self {
            Self::Initial => *self,
            _ if moving => FrameGroup::Moving,
            _ => FrameGroup::Idle,
        };
    }
}
