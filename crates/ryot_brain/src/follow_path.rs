use crate::find_closest::Target;
use bevy::prelude::*;
use big_brain::actions::ActionState;
use big_brain::prelude::*;
use rand::prelude::SliceRandom;
use ryot_core::prelude::*;
use ryot_pathfinder::prelude::*;
use ryot_tiled::prelude::*;
use ryot_utils::prelude::*;
use std::ops::Deref;

pub trait PathFollowingThinker {
    fn follows_path(self) -> Self;
    fn follows_path_with_fallback(self, default: impl ActionBuilder + 'static) -> Self;
}

impl PathFollowingThinker for ThinkerBuilder {
    fn follows_path(self) -> Self {
        self.when(
            FollowPathScore::default(),
            Steps::build().label("FollowPath").step(FollowPath),
        )
    }

    fn follows_path_with_fallback(self, default: impl ActionBuilder + 'static) -> Self {
        self.when(
            FollowPathScore::default(),
            Steps::build()
                .label("FollowPath")
                .step(FollowPath)
                .step(default),
        )
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct FollowPathScore(Timer);

impl Default for FollowPathScore {
    fn default() -> Self {
        Self(Timer::from_seconds(1., TimerMode::Repeating))
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct FollowPath;

pub fn follow_path_scorer(
    time: Res<Time>,
    targets: Query<&Target>,
    positions: Query<&TilePosition>,
    mut query: Query<(&Actor, &mut Score, &mut FollowPathScore)>,
) {
    for (Actor(actor), mut score, mut scorer) in &mut query {
        let actor_position = positions.get(*actor).expect("actor has no position");

        let is_far_from_target = targets.get(*actor).map_or(true, |target| {
            let target_position = positions
                .get(*target.deref())
                .expect("target has no position");

            actor_position.distance(target_position) > 1.42
        });

        if scorer.0.tick(time.delta()).just_finished() && is_far_from_target {
            scorer.0.reset();
            score.set(0.7);
        } else {
            score.set(0.);
        }
    }
}

pub fn follow_path<T: From<OrdinalDirection>>(
    mut cmd: Commands,
    positions: Query<&TilePosition>,
    mut paths: Query<&mut TiledPath>,
    flags_cache: Res<Cache<TilePosition, Flags>>,
    mut action_query: Query<(&Actor, &mut ActionState, &FollowPath, &ActionSpan)>,
) -> Vec<(Entity, T)> {
    let mut result = Vec::new();

    for (Actor(actor), mut action_state, _, span) in &mut action_query {
        let _guard = span.span().enter();

        match *action_state {
            ActionState::Requested => {
                debug!("Let's follow the path!");
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let actor_position = positions.get(*actor).expect("actor has no position");
                let Ok(mut path) = paths.get_mut(*actor) else {
                    debug!("Actor has no path.");
                    cmd.entity(*actor).remove::<Target>();
                    *action_state = ActionState::Success;
                    continue;
                };

                let Some(next_pos) = path.first().copied() else {
                    debug!("Actor path is empty.");
                    cmd.entity(*actor).remove::<Target>();
                    *action_state = ActionState::Success;
                    continue;
                };

                path.remove(0);

                if next_pos == *actor_position {
                    if path.is_empty() {
                        *action_state = ActionState::Cancelled;
                    }

                    continue;
                }

                if !next_pos.is_navigable(&flags_cache) {
                    debug!("Next position is not valid, failing.");
                    *action_state = ActionState::Cancelled;
                } else {
                    debug!("Moving to {:?}", next_pos);
                    result.push((*actor, T::from(actor_position.direction_to(next_pos))));
                    *action_state = ActionState::Failure;
                }
            }
            ActionState::Cancelled => {
                cmd.entity(*actor).remove::<Target>();
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }

    result
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MovesRandomly;

pub fn moves_randomly<T: From<OrdinalDirection>>(
    positions: Query<&TilePosition>,
    flags_cache: Res<Cache<TilePosition, Flags>>,
    mut action_query: Query<(&Actor, &mut ActionState, &MovesRandomly, &ActionSpan)>,
) -> Vec<(Entity, T)> {
    let mut result = Vec::new();

    for (Actor(actor), mut action_state, _, span) in &mut action_query {
        let _guard = span.span().enter();

        match *action_state {
            ActionState::Requested => {
                debug!("Let's move randomly!");
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let actor_position = positions.get(*actor).expect("actor has no position");
                let random_neighbor =
                    get_weighted_random_neighbor(actor_position, 1, 2, &flags_cache);

                if let Some(next) = random_neighbor {
                    debug!("Moving actor randomly to {:?}", next);
                    result.push((*actor, T::from(actor_position.direction_to(next))));
                    *action_state = ActionState::Success;
                } else {
                    debug!("No valid position found for actor {:?}", actor);
                    *action_state = ActionState::Failure;
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }

    result
}

fn get_weighted_random_neighbor(
    pos: &TilePosition,
    cardinal_cost: u32,
    diagonal_cost: u32,
    flags_cache: &Res<Cache<TilePosition, Flags>>,
) -> Option<TilePosition> {
    let mut cardinal = weighted_neighbors_2d_generator(
        pos,
        &|pos| pos.is_navigable(flags_cache),
        cardinal_cost,
        diagonal_cost,
    )
    .into_iter()
    .filter(|(_, cost)| *cost == cardinal_cost)
    .collect::<Vec<(TilePosition, u32)>>();

    let mut rng = rand::thread_rng();

    cardinal.shuffle(&mut rng);
    cardinal.last().map(|(pos, _)| *pos)
}
