use crate::prelude::*;
use crate::request::Params;
use bevy_ecs::prelude::*;
use bevy_utils::HashSet;
use ryot_core::prelude::Navigable;
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
pub struct Trajectory<T, P> {
    pub area: RadialArea<P>,
    pub shared_with: HashSet<Entity>,
    condition: fn(&Self, &dyn Navigable, &P) -> bool,
    params: Params,
    last_executed_at: Option<Instant>,
    marker: PhantomData<T>,
}

impl<T, P> Trajectory<T, P> {
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

    pub fn with_max_hits(mut self, max_hits: i32) -> Self {
        self.params.max_hits = max_hits;
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

impl<T, P: Copy> Trajectory<T, P> {
    pub fn meets_condition<N: Navigable>(&self, flags: &N, position: &P) -> bool {
        (self.condition)(self, flags, position)
    }
}

impl<T: Copy, P: TrajectoryPoint> Trajectory<T, P> {
    pub fn execute<N: Navigable>(
        &mut self,
        from: &P,
        intersections_per_trajectory: &Vec<Vec<P>>,
        get_nav_for_position: impl Fn(&P) -> N,
    ) -> Option<Intersections<T, P>> {
        let mut hits = VecDeque::new();
        let mut impact_area = VecDeque::new();

        if !self.can_execute() {
            return None;
        }

        for intersections in intersections_per_trajectory {
            let reversed = self.params.reversed;
            let mut max_hits = self.params.max_hits;

            let mut previous_pos = from;

            let mut executor = |pos| {
                let Some((intersection, hit)) = self.execute_for_position(
                    from,
                    pos,
                    previous_pos,
                    &mut max_hits,
                    &get_nav_for_position,
                ) else {
                    return;
                };

                if intersection.pierced {
                    impact_area.push_back(intersection.position);
                }

                if hit {
                    hits.push_back(intersection);
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

        Some(Intersections::new(hits, impact_area))
    }

    fn execute_for_position<N: Navigable>(
        &mut self,
        from: &P,
        pos: &P,
        previous_pos: &P,
        remaining_hits: &mut i32,
        get_nav_for_position: impl Fn(&P) -> N,
    ) -> Option<(Intersection<P, T>, bool)> {
        let flags = get_nav_for_position(pos);
        let hit = !self.meets_condition(&flags, pos);

        if hit {
            *remaining_hits -= 1;
        }

        let intersection = Intersection::new(*pos, from.distance_2d(pos), *previous_pos);

        if *remaining_hits >= 0 {
            Some((intersection.pierced(), hit))
        } else if *remaining_hits == -1 && hit {
            Some((intersection, true))
        } else {
            None
        }
    }
}

pub fn visible_trajectory<T, P>(area: RadialArea<P>) -> Trajectory<T, P> {
    Trajectory::<T, P>::new(area, |_, flags, _pos| !flags.blocks_sight())
}

pub fn walkable_trajectory<T, P>(area: RadialArea<P>) -> Trajectory<T, P> {
    Trajectory::<T, P>::new(area, |_, flags, _pos| flags.is_walkable())
}
