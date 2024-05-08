use bevy::prelude::*;
use big_brain::actions::ActionState;
use big_brain::prelude::*;
use derive_more::{Deref, DerefMut};
use ryot_core::prelude::*;
use ryot_tiled::prelude::*;
use ryot_utils::prelude::*;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::time::Duration;

pub trait Thinking = Clone + Debug + ThreadSafe;

#[derive(Clone, Copy, Component, Debug, Deref, DerefMut)]
pub struct Target(pub TilePosition);

pub trait PathFindingThinker {
    fn find_path<T: Thinking>(self) -> Self;
}

impl PathFindingThinker for ThinkerBuilder {
    fn find_path<T: Thinking>(self) -> Self {
        self.when(
            FindTargetScorer::<T>::with_duration_in_seconds(1.),
            Steps::build()
                .label("FindClosestTarget")
                .step(FindClosestTarget::<T>::default()),
        )
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct FindTargetScorer<T: Thinking>(pub Timer, PhantomData<T>);

impl<T: Thinking> Default for FindTargetScorer<T> {
    fn default() -> Self {
        Self::with_duration_in_seconds(0.5)
    }
}

impl<T: Thinking> FindTargetScorer<T> {
    fn with_duration_in_seconds(duration: f32) -> Self {
        Self(
            Timer::from_seconds(duration, TimerMode::Repeating),
            PhantomData,
        )
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
    targets: Query<&Target>,
    mut query: Query<(&Actor, &mut Score, &mut FindTargetScorer<T>)>,
) {
    for (Actor(_actor), mut score, mut scorer) in &mut query {
        let tick = if targets.get(*_actor).is_err() { 3 } else { 1 } * time.delta();

        if scorer.0.tick(tick).just_finished() {
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
    for (Actor(actor), mut action_state, _, span) in &mut action_query {
        let _guard = span.span().enter();

        match *action_state {
            ActionState::Requested => {
                debug!("Finding closest target!");
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let actor_position = positions.get(*actor).expect("actor has no position");
                let closest_target = get_closest_target(actor_position, &q_target_positions);

                let Some((_, closest_target)) = closest_target else {
                    debug!("No closest target found, failing.");
                    *action_state = ActionState::Cancelled;
                    continue;
                };

                let target_pos = get_closest_valid_surrounding_position(
                    &closest_target,
                    actor_position,
                    &flags_cache,
                );

                let Some(target_pos) = target_pos else {
                    debug!("Unreachable path, failing.");
                    *action_state = ActionState::Cancelled;
                    continue;
                };

                if closest_target.distance(actor_position) > 300. {
                    debug!("Target is too far away, failing.");
                    *action_state = ActionState::Cancelled;
                    continue;
                };

                if *actor_position == target_pos {
                    debug!("Target is already at the closest valid position, failing.");
                    *action_state = ActionState::Failure;
                    continue;
                };

                cmd.entity(*actor).insert((
                    Target(target_pos),
                    TiledPathFindingQuery::new(target_pos)
                        .with_success_range((0., 0.))
                        .with_timeout(Duration::from_secs(2)),
                ));

                debug!("Calculating path towards {:?}", target_pos);
                *action_state = ActionState::Success;
            }
            ActionState::Cancelled => {
                cmd.entity(*actor).remove::<Target>();
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
    flags_cache: &Res<Cache<TilePosition, Flags>>,
) -> Option<TilePosition> {
    destination_pos
        .get_surroundings()
        .iter()
        .filter(|&pos| pos.is_navigable(flags_cache) || pos == actor_pos)
        .map(|&pos| (pos, pos.distance(actor_pos)))
        .min_by(|(a, a_weight), (b, b_weight)| {
            (a.distance(actor_pos) * *a_weight)
                .partial_cmp(&(b.distance(actor_pos) * *b_weight))
                .unwrap()
        })
        .map(|(pos, _)| pos)
}
