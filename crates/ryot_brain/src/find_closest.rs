use bevy::prelude::*;
use big_brain::actions::ActionState;
use big_brain::prelude::*;
use derive_more::{Deref, DerefMut};
use ryot_core::prelude::*;
use ryot_pathfinder::prelude::*;
use ryot_tiled::prelude::*;
use ryot_utils::prelude::*;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::time::Duration;

pub trait Thinking = Clone + Debug + ThreadSafe;

#[derive(Clone, Copy, Component, Debug, Deref, DerefMut)]
pub struct Target(Entity);

pub trait PathFindingThinker {
    fn find_path<T: Thinking>(self) -> Self;
}

impl PathFindingThinker for ThinkerBuilder {
    fn find_path<T: Thinking>(self) -> Self {
        self.when(
            FindTargetScorer::<T>::default(),
            Steps::build()
                .label("FindClosestTarget")
                .step(FindClosestTarget::<T>::default()),
        )
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct FindTargetScorer<T: Thinking>(Timer, PhantomData<T>);

impl<T: Thinking> Default for FindTargetScorer<T> {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating), PhantomData)
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct FindClosestTarget<T: Thinking>(PhantomData<T>);

impl<T: Thinking> Default for FindClosestTarget<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

pub fn find_path_scorer<T: Thinking + Component>(
    time: Res<Time>,
    mut query: Query<(&Actor, &mut Score, &mut FindTargetScorer<T>)>,
) {
    for (Actor(_actor), mut score, mut scorer) in &mut query {
        if scorer.0.tick(time.delta()).just_finished() {
            score.set(0.7);
        } else {
            score.set(0.);
        }
    }
}

pub fn find_closest_target<T: Component + Thinking>(
    mut cmd: Commands,
    positions: Query<&TilePosition>,
    flags_cache: Res<Cache<TilePosition, Flags>>,
    q_target_positions: Query<(Entity, &TilePosition), With<T>>,
    mut action_query: Query<(&Actor, &mut ActionState, &FindClosestTarget<T>, &ActionSpan)>,
) {
    for (actor, mut action_state, _, span) in &mut action_query {
        let _guard = span.span().enter();

        match *action_state {
            ActionState::Requested => {
                debug!("Finding closest target!");
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let actor_position = positions.get(actor.0).expect("actor has no position");
                let closest_target = get_closest_target(actor_position, &q_target_positions);

                let Some((target, closest_target)) = closest_target else {
                    debug!("No closest target found, failing.");
                    *action_state = ActionState::Failure;
                    continue;
                };

                let target_pos = get_closest_valid_surrounding_position(
                    &closest_target,
                    actor_position,
                    1,
                    2,
                    &flags_cache,
                );

                let Some(target_pos) = target_pos else {
                    debug!("Unreachable path, failing.");
                    *action_state = ActionState::Failure;
                    continue;
                };

                if *actor_position == target_pos {
                    debug!("Target is already at the closest valid position, failing.");
                    *action_state = ActionState::Failure;
                    continue;
                };

                cmd.entity(actor.0).insert((
                    Target(target),
                    TiledPathFindingQuery::new(target_pos)
                        .with_success_range((0., 0.))
                        .with_timeout(Duration::from_secs(2)),
                ));

                debug!("Calculating path towards {:?}", target_pos);
                *action_state = ActionState::Success;
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

fn get_closest_target<T: Component>(
    actor_position: &TilePosition,
    q_target_positions: &Query<(Entity, &TilePosition), With<T>>,
) -> Option<(Entity, TilePosition)> {
    q_target_positions
        .iter()
        .min_by(|(_, &a), (_, &b)| {
            a.distance(actor_position)
                .partial_cmp(&b.distance(actor_position))
                .unwrap()
        })
        .map(|(entity, pos)| (entity, *pos))
}

fn get_closest_valid_surrounding_position(
    destination_pos: &TilePosition,
    actor_pos: &TilePosition,
    cardinal_cost: u32,
    diagonal_cost: u32,
    flags_cache: &Res<Cache<TilePosition, Flags>>,
) -> Option<TilePosition> {
    let surrounding_positions = weighted_neighbors_2d_generator(
        destination_pos,
        &|pos| pos.is_navigable(flags_cache) || pos == actor_pos,
        cardinal_cost,
        diagonal_cost,
    );

    surrounding_positions
        .iter()
        .min_by(|(a, a_weight), (b, b_weight)| {
            (a.distance(actor_pos) * *a_weight as f32)
                .partial_cmp(&(b.distance(actor_pos) * *b_weight as f32))
                .unwrap()
        })
        .map(|(pos, _)| *pos)
}
