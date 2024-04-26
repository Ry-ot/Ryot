#[cfg(feature = "bevy")]
use bevy_ecs::change_detection::ResMut;
#[cfg(feature = "bevy")]
use bevy_ecs::prelude::NextState;

/// The states that the content loading process can be in.
/// This is used to track the progress of the content loading process.
/// It's also used to determine if the content is ready to be used.
/// It's internally used by the `ContentPlugin` and should not be manipulated directly.
/// Can be checked by applications to perform actions that depend on the state of the content.
#[derive(Default, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::States))]
pub enum RyotContentState {
    #[default]
    LoadingContent,
    PreparingContent,
    Ready,
}

#[cfg(feature = "bevy")]
pub fn transition_to_ready(mut state: ResMut<NextState<RyotContentState>>) {
    state.set(RyotContentState::Ready);
}
