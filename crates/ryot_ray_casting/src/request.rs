use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_utils::HashSet;
use ryot_core::prelude::{Navigable, Point};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::time::{Duration, Instant};

/// Possible types of execution for a ray casting request, based on time or a single execution.
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug)]
pub enum ExecutionType {
    #[default]
    Once,
    TimeBased(Duration),
}

impl ExecutionType {
    pub fn every_in_ms(ms: u64) -> Self {
        Self::TimeBased(Duration::from_millis(ms))
    }

    pub fn every_in_sec(secs: u64) -> Self {
        Self::TimeBased(Duration::from_secs(secs))
    }
}

/// The entry point for the ray casting system, this component defines the parameters for a ray
/// casting request. A ray casting request triggers the evaluation of one or more rays from the
/// spectator through a given radial area, based on the navigation condition and the ray casting
/// parameters defined in the request.
///
/// Rays can collide while navigating through a plane, and the request allows one to customize the
/// behavior of those collisions, such as the maximum number of collisions allowed before stopping
/// the evaluation and whether the ray should be reversed - starting from the end point and moving
/// towards the start point - or not.
///
/// The propagation of a ray casting request can be shared with other entities, allowing for the
/// sharing of critical spatial information within the game world.
///
/// A ray casting request can be executed once, being removed from the system after execution, or
/// executed periodically based on a time interval.
#[derive(Debug, Clone, Eq, PartialEq, Component)]
pub struct RayCasting<T, P> {
    pub reversed: bool,
    pub max_collisions: i32,
    pub area: RadialArea<P>,
    pub shared_with: HashSet<Entity>,
    pub execution_type: ExecutionType,
    pub condition: fn(&Self, &dyn Navigable, &P) -> bool,
    last_executed_at: Option<Instant>,
    marker: PhantomData<T>,
}

impl<T, P: Point> Default for RayCasting<T, P> {
    fn default() -> Self {
        Self {
            reversed: false,
            max_collisions: 0,
            area: RadialArea::<P>::default(),
            shared_with: HashSet::default(),
            condition: |_, _, _| true,
            execution_type: ExecutionType::Once,
            last_executed_at: None,
            marker: PhantomData,
        }
    }
}

impl<T, P: Point> RayCasting<T, P> {
    pub fn new(area: RadialArea<P>, condition: fn(&Self, &dyn Navigable, &P) -> bool) -> Self {
        Self {
            area,
            condition,
            ..Default::default()
        }
    }

    pub fn can_execute(&self) -> bool {
        match self.execution_type {
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
        self.max_collisions = max_collisions;
        self
    }

    pub fn reversed(mut self) -> Self {
        self.reversed = true;
        self
    }

    pub fn with_execution_type(mut self, execution_type: ExecutionType) -> Self {
        self.execution_type = execution_type;
        self
    }

    pub fn last_execution(&self) -> Option<Instant> {
        self.last_executed_at
    }

    pub fn execution_type(&self) -> ExecutionType {
        self.execution_type
    }
}

impl<T, P: Copy> RayCasting<T, P> {
    pub fn meets_condition<N: Navigable>(&self, flags: &N, position: &P) -> bool {
        (self.condition)(self, flags, position)
    }
}

impl<T: Copy, P: RayCastingPoint> RayCasting<T, P> {
    pub fn execute<N: Navigable>(
        &mut self,
        from: &P,
        intersections_per_ray: &Vec<Vec<P>>,
        get_nav_for_position: impl Fn(&P) -> N,
    ) -> Option<RayPropagation<T, P>> {
        let mut collisions = VecDeque::new();
        let mut impact_area = VecDeque::new();

        if !self.can_execute() {
            return None;
        }

        for intersections in intersections_per_ray {
            let reversed = self.reversed;
            let mut max_collisions = self.max_collisions;

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

        Some(RayPropagation::new(collisions, impact_area))
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

pub fn visible_ray_casting<T, P: Point>(area: RadialArea<P>) -> RayCasting<T, P> {
    RayCasting::<T, P>::new(area, |_, flags, _pos| !flags.blocks_sight())
}

pub fn walkable_ray_casting<T, P: Point>(area: RadialArea<P>) -> RayCasting<T, P> {
    RayCasting::<T, P>::new(area, |_, flags, _pos| flags.is_walkable())
}
