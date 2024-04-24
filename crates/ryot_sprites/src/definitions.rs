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
