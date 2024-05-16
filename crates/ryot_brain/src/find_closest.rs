use crate::follow_path::MovesRandomly;
use bevy::prelude::*;
use big_brain::actions::ActionState;
use big_brain::prelude::*;
use ryot_core::prelude::*;
use ryot_pathfinder::prelude::Pathable;
use ryot_tiled::prelude::*;
use ryot_utils::prelude::*;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

/*
Thinker Bundle {
    Thinker,
    Thinker timer (every X seconds),
}

struct
 */

#[derive(Bundle, Debug)]
pub struct ThinkerBundle {
    pub thinker: ThinkerBuilder,
    pub cooldown: Cooldown<Thinker>,
}

impl ThinkerBundle {
    pub fn new(thinker: ThinkerBuilder, cooldown: Cooldown<Thinker>) -> Self {
        Self { thinker, cooldown }
    }
}

pub trait Thinking = Clone + Debug + ThreadSafe;

pub trait PathFindingThinker {
    fn find_path<T: Thinking>(self) -> Self;
}

impl PathFindingThinker for ThinkerBuilder {
    fn find_path<T: Thinking>(self) -> Self {
        self.when(
            FindTargetScore::<T>::default(),
            Steps::build()
                .label("FindClosestTarget")
                .step(FindClosestTarget::<T>::default())
                .step(MovesRandomly),
        )
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct FindTargetScore<T: Thinking>(PhantomData<T>);

impl<T: Thinking> Default for FindTargetScore<T> {
    fn default() -> Self {
        Self(PhantomData)
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
    cooldowns: Query<&Cooldown<Thinker>>,
    mut query: Query<(&Actor, &mut Score), With<FindTargetScore<T>>>,
) {
    for (Actor(actor), mut score) in &mut query {
        if is_valid_cooldown_for_entity(actor, &cooldowns) {
            score.set(0.3);
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

                // This action is successful by default
                *action_state = ActionState::Success;

                let Some((_, closest_target)) = closest_target else {
                    debug!("No closest target found.");
                    continue;
                };

                let target_pos = get_closest_valid_surrounding_position(
                    &closest_target,
                    actor_position,
                    &flags_cache,
                );

                let Some(target_pos) = target_pos else {
                    debug!("Unreachable path.");
                    continue;
                };

                if closest_target.distance(actor_position) > 300. {
                    debug!("Target is too far away.");
                    continue;
                };

                *action_state = ActionState::Failure;

                if *actor_position == target_pos {
                    debug!("Target is already at the closest valid position.");
                    continue;
                };

                cmd.entity(*actor)
                    .insert((TiledPathFindingQuery::new(target_pos)
                        .with_success_range((0., 0.))
                        .with_timeout(Duration::from_secs(2)),));

                debug!("Calculating path towards {:?}", target_pos);
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
    flags_cache: &Res<Cache<TilePosition, Flags>>,
) -> Option<TilePosition> {
    let flags_cache_arc = Arc::clone(&flags_cache);

    destination_pos
        .get_surroundings()
        .iter()
        .filter(|&pos| pos.can_be_navigated(flags_cache_arc.clone()) || pos == actor_pos)
        .map(|&pos| (pos, pos.distance(actor_pos)))
        .min_by(|(a, a_weight), (b, b_weight)| {
            (a.distance(actor_pos) * *a_weight)
                .partial_cmp(&(b.distance(actor_pos) * *b_weight))
                .unwrap()
        })
        .map(|(pos, _)| pos)
}
