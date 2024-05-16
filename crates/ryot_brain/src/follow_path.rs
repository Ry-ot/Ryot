use bevy::prelude::*;
use bevy::utils::HashMap;
use big_brain::actions::ActionState;
use big_brain::prelude::*;
use rand::prelude::SliceRandom;
use ryot_core::prelude::*;
use ryot_pathfinder::prelude::*;
use ryot_tiled::prelude::*;
use ryot_utils::prelude::*;
use std::sync::{Arc, RwLock};
use std::time::Duration;

pub trait PathFollowingThinker {
    fn follows_path(self) -> Self;
}

impl PathFollowingThinker for ThinkerBuilder {
    fn follows_path(self) -> Self {
        self.when(
            FollowPathScore,
            Steps::build().label("FollowPath").step(FollowPath),
        )
    }
}

#[derive(Clone, Component, Default, Debug, ScorerBuilder)]
pub struct FollowPathScore;

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct FollowPath;

pub fn follow_path_scorer(
    paths: Query<&TiledPath>,
    cooldowns: Query<&Cooldown<Thinker>>,
    mut query: Query<(&Actor, &mut Score), With<FollowPathScore>>,
) {
    for (Actor(actor), mut score) in &mut query {
        let can_walk = paths.get(*actor).map_or(false, |path| !path.is_empty());

        if is_valid_cooldown_for_entity(actor, &cooldowns) && can_walk {
            score.set(0.6);
        } else {
            score.set(0.);
        }
    }
}

#[derive(Event, Clone, Copy, Debug)]
pub struct WalkTo(pub Entity, pub TilePosition);

pub fn follow_path(
    mut cmd: Commands,
    mut walk_writer: EventWriter<WalkTo>,
    mut amend_writer: EventWriter<AmendPathCommand<TilePosition>>,
    flags_cache: Res<Cache<TilePosition, Flags>>,
    mut q_actor: Query<(&mut TiledPath, &TilePosition)>,
    mut action_query: Query<(&Actor, &mut ActionState, &FollowPath, &ActionSpan)>,
) {
    let flags_cache_arc = Arc::clone(&flags_cache);

    for (Actor(actor), mut action_state, _, span) in &mut action_query {
        let _guard = span.span().enter();

        match *action_state {
            ActionState::Requested => {
                debug!("Let's follow the path!");
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let Ok((mut path, actor_position)) = q_actor.get_mut(*actor) else {
                    debug!("Invalid actor.");
                    *action_state = ActionState::Success;
                    continue;
                };

                if path.is_empty() {
                    debug!("Actor path is empty.");
                    cmd.entity(*actor).remove::<TiledPath>();
                    *action_state = ActionState::Success;
                    continue;
                };

                let Some((index, next_pos)) =
                    path.iter().copied().enumerate().find(|(_, next_pos)| {
                        *next_pos != *actor_position
                            && next_pos.can_be_navigated(flags_cache_arc.clone())
                    })
                else {
                    cmd.entity(*actor).remove::<TiledPath>();
                    *action_state = ActionState::Success;
                    continue;
                };

                let mut removed = path.remove(0);

                if removed == *actor_position {
                    removed = path.remove(0);
                }

                *action_state = ActionState::Failure;

                if removed != next_pos {
                    debug!("Amending path towards {:?}", next_pos);
                    amend_writer.send(AmendPathCommand {
                        entity: *actor,
                        path_amend_index: index,
                        path_finding_query: TiledPathFindingQuery::new(next_pos)
                            .with_diagonal_cost(3)
                            .with_success_range((0., 0.))
                            .with_timeout(Duration::from_millis(10)),
                    });
                } else {
                    debug!("Moving to {:?}", next_pos);
                    walk_writer.send(WalkTo(*actor, next_pos));
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MovesRandomly;

pub fn moves_randomly(
    positions: Query<&TilePosition>,
    mut walk_writer: EventWriter<WalkTo>,
    flags_cache: Res<Cache<TilePosition, Flags>>,
    mut action_query: Query<(&Actor, &mut ActionState, &MovesRandomly, &ActionSpan)>,
) {
    let flags_cache_arc = Arc::clone(&flags_cache);

    for (Actor(actor), mut action_state, _, span) in &mut action_query {
        let _guard = span.span().enter();

        match *action_state {
            ActionState::Requested => {
                trace!("Let's move randomly!");
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let actor_position = positions.get(*actor).expect("actor has no position");
                let random_neighbor =
                    get_weighted_random_neighbor(actor_position, 1, 2, flags_cache_arc.clone());

                if let Some(next) = random_neighbor {
                    info!("Moving actor randomly to {:?}", next);
                    walk_writer.send(WalkTo(*actor, next));
                    *action_state = ActionState::Success;
                } else {
                    info!("No valid position found for actor {:?}", actor);
                    *action_state = ActionState::Failure;
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

fn get_weighted_random_neighbor(
    pos: &TilePosition,
    cardinal_cost: u32,
    diagonal_cost: u32,
    flags_cache: Arc<RwLock<HashMap<TilePosition, Flags>>>,
) -> Option<TilePosition> {
    let mut cardinal = weighted_neighbors_2d_generator(
        pos,
        &|pos| pos.can_be_navigated(flags_cache.clone()),
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

pub fn walk_to_direction<T: From<OrdinalDirection>>(
    mut walk_reader: EventReader<WalkTo>,
    mut positions: Query<&mut TilePosition>,
) -> Vec<(Entity, T)> {
    walk_reader
        .read()
        .filter_map(|WalkTo(entity, next_pos)| {
            Some((
                *entity,
                positions
                    .get_mut(*entity)
                    .ok()?
                    .direction_to(*next_pos)
                    .into(),
            ))
        })
        .collect::<Vec<_>>()
}
