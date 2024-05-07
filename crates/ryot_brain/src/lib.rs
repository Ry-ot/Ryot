#![feature(trait_alias)]
mod find_closest;
mod follow_path;

pub mod prelude {
    pub use crate::{
        find_closest::{
            find_closest_target, find_path_scorer, FindClosestTarget, FindTargetScorer,
            PathFindingThinker, Target,
        },
        follow_path::{
            follow_path, follow_path_scorer, moves_randomly, FollowPath, FollowPathScore,
            MovesRandomly, PathFollowingThinker,
        },
    };
}
