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
