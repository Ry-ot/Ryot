use crate::prelude::*;
use crate::request::Params;
use bevy_ecs::prelude::*;
use bevy_utils::HashSet;
use ryot_core::prelude::{Navigable, Point};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::time::Instant;

/// Represents entities that can provide a `RadialArea` for perspective calculation.
///
/// This trait facilitates the generation of a radial area based on an entity's current state or
/// position. It is used to abstract the way different entities determine their perspective in the
/// world. The `meets_condition` method allows for additional checks on environmental or
/// entity-specific conditions that may affect whether a position is considered valid for certain
/// operations within the trajectory area, like visibility checks or interactions.
#[derive(Debug, Clone, Eq, PartialEq, Component)]
pub struct TrajectoryRequest<T, P> {
    pub area: RadialArea<P>,
    pub shared_with: HashSet<Entity>,
    condition: fn(&Self, &dyn Navigable, &P) -> bool,
    params: Params,
    last_executed_at: Option<Instant>,
    marker: PhantomData<T>,
}

impl<T, P: Point> Default for TrajectoryRequest<T, P> {
    fn default() -> Self {
        Self {
            area: RadialArea::<P>::default(),
            condition: |_, _, _| true,
            params: Params::default(),
            shared_with: HashSet::default(),
            last_executed_at: None,
            marker: PhantomData,
        }
    }
}

impl<T, P> TrajectoryRequest<T, P> {
    pub fn new(area: RadialArea<P>, condition: fn(&Self, &dyn Navigable, &P) -> bool) -> Self {
        Self {
            area,
            condition,
            params: Params::default(),
            shared_with: HashSet::default(),
            last_executed_at: None,
            marker: PhantomData,
        }
    }

    pub fn can_execute(&self) -> bool {
        match self.params.execution_type {
            ExecutionType::Once => self.last_executed_at.is_none(),
            ExecutionType::TimeBased(duration) => {
                self.last_executed_at.map_or(true, |last_executed_at| {
                    last_executed_at.elapsed() >= duration
                })
            }
        }
    }

    pub fn share_with(mut self, entities: Vec<Entity>) -> Self {
        self.shared_with.extend(entities);
        self
    }

    pub fn with_max_collisions(mut self, max_collisions: i32) -> Self {
        self.params.max_collisions = max_collisions;
        self
    }

    pub fn reversed(mut self) -> Self {
        self.params.reversed = true;
        self
    }

    pub fn with_execution_type(mut self, execution_type: ExecutionType) -> Self {
        self.params.execution_type = execution_type;
        self
    }

    pub fn last_execution(&self) -> Option<Instant> {
        self.last_executed_at
    }

    pub fn execution_type(&self) -> ExecutionType {
        self.params.execution_type
    }
}

impl<T, P: Copy> TrajectoryRequest<T, P> {
    pub fn meets_condition<N: Navigable>(&self, flags: &N, position: &P) -> bool {
        (self.condition)(self, flags, position)
    }
}

impl<T: Copy, P: TrajectoryPoint> TrajectoryRequest<T, P> {
    pub fn execute<N: Navigable>(
        &mut self,
        from: &P,
        intersections_per_trajectory: &Vec<Vec<P>>,
        get_nav_for_position: impl Fn(&P) -> N,
    ) -> Option<TrajectoryResult<T, P>> {
        let mut collisions = VecDeque::new();
        let mut impact_area = VecDeque::new();

        if !self.can_execute() {
            return None;
        }

        for intersections in intersections_per_trajectory {
            let reversed = self.params.reversed;
            let mut max_collisions = self.params.max_collisions;

            let mut previous_pos = from;

            let mut executor = |pos| {
                let Some((intersection, collided)) = self.execute_for_position(
                    from,
                    pos,
                    previous_pos,
                    &mut max_collisions,
                    &get_nav_for_position,
                ) else {
                    return;
                };

                if intersection.pierced {
                    impact_area.push_back(intersection.position);
                }

                if collided {
                    collisions.push_back(intersection);
                }

                previous_pos = pos;
            };

            if reversed {
                intersections.iter().rev().for_each(&mut executor);
            } else {
                intersections.iter().for_each(&mut executor);
            }
        }

        self.last_executed_at = Some(Instant::now());

        Some(TrajectoryResult::new(collisions, impact_area))
    }

    fn execute_for_position<N: Navigable>(
        &mut self,
        from: &P,
        pos: &P,
        previous_pos: &P,
        remaining_collisions: &mut i32,
        get_nav_for_position: impl Fn(&P) -> N,
    ) -> Option<(Collision<T, P>, bool)> {
        let flags = get_nav_for_position(pos);
        let collided = !self.meets_condition(&flags, pos);

        if collided {
            *remaining_collisions -= 1;
        }

        let intersection = Collision::new(*pos, from.distance_2d(pos), *previous_pos);

        if *remaining_collisions >= 0 {
            Some((intersection.pierced(), collided))
        } else if *remaining_collisions == -1 && collided {
            Some((intersection, true))
        } else {
            None
        }
    }
}

pub fn visible_trajectory<T, P>(area: RadialArea<P>) -> TrajectoryRequest<T, P> {
    TrajectoryRequest::<T, P>::new(area, |_, flags, _pos| !flags.blocks_sight())
}

pub fn walkable_trajectory<T, P>(area: RadialArea<P>) -> TrajectoryRequest<T, P> {
    TrajectoryRequest::<T, P>::new(area, |_, flags, _pos| flags.is_walkable())
}
