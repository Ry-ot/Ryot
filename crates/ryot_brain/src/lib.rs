#![feature(trait_alias)]
#![feature(let_chains)]

mod find_closest;
mod follow_path;
mod pickers;

pub mod prelude {
    pub use crate::{
        find_closest::{
            find_closest_target, find_path_scorer, FindClosestTarget, FindTargetScore,
            PathFindingThinker, ThinkerBundle,
        },
        follow_path::{
            follow_path, follow_path_scorer, moves_randomly, walk_to_direction, FollowPath,
            FollowPathScore, MovesRandomly, PathFollowingThinker, WalkTo,
        },
        pickers::HighestWithThreshold,
    };
}
