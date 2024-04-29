use crate::components::PathFindingQuery;
use crate::pathable::Pathable;
use bevy_math::Vec2;
use pathfinding::prelude::astar;
use std::time::Instant;

/// Calculates a 2D path using the A* algorithm, optimized for grid-based environments.
/// This function provides default pathfinding behavior which can be overridden for
/// customized pathfinding logic or non-grid environments.
pub fn find_path_2d<
    P: Pathable,
    FV: Fn(&P) -> bool,
    FN: Fn(&P, &FV, &PathFindingQuery<P>) -> Vec<(P, u32)>,
>(
    from: &P,
    query: &PathFindingQuery<P>,
    validator: &FV,
    neighbors_generator: &FN,
) -> Option<(Vec<P>, u32)> {
    let start = Instant::now();

    let distance = |from: &P, to: &P| {
        let to_coordinates = to.coordinates();
        let from_coordinates = from.coordinates();

        Vec2::new(to_coordinates.0 as f32, to_coordinates.1 as f32).distance(Vec2::new(
            from_coordinates.0 as f32,
            from_coordinates.1 as f32,
        ))
    };

    astar(
        from,
        |next| match query.timeout {
            Some(timeout) if Instant::now().duration_since(start) > timeout => vec![],
            _ => neighbors_generator(next, validator, query),
        },
        |next| distance(&query.to, next) as u32,
        |next| distance(&query.to, next) <= query.success_distance,
    )
}

/// Generates neighbors with their respective costs, facilitating weighted pathfinding
/// that includes considerations for both cardinal and diagonal movements.
pub fn weighted_neighbors_2d_generator<P: Pathable, F: Fn(&P) -> bool + ?Sized>(
    pathable: &P,
    validator: &F,
    query: &PathFindingQuery<P>,
) -> Vec<(P, u32)> {
    let (x, y, z) = pathable.coordinates();

    let mut cardinal = [
        P::generate(x + 1, y, z),
        P::generate(x - 1, y, z),
        P::generate(x, y + 1, z),
        P::generate(x, y - 1, z),
    ]
    .iter()
    .map(|p| (*p, query.cardinal_cost))
    .collect::<Vec<(P, u32)>>();

    cardinal.extend(
        [
            P::generate(x + 1, y + 1, z),
            P::generate(x - 1, y - 1, z),
            P::generate(x + 1, y - 1, z),
            P::generate(x - 1, y + 1, z),
        ]
        .iter()
        .map(|p| (*p, query.diagonal_cost))
        .collect::<Vec<(P, u32)>>(),
    );

    cardinal.into_iter().filter(|(p, _)| validator(p)).collect()
}
