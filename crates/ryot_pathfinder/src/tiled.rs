use crate::Pathable;
use pathfinding::prelude::astar;
use ryot_grid::position::TilePosition;
use std::time::{Duration, Instant};

impl Pathable for TilePosition {
    fn path_to<F: Fn(&Self) -> bool>(
        &self,
        to: Self,
        is_walkable: F,
        timeout: Option<Duration>,
    ) -> Option<(Vec<Self>, u32)> {
        let start = Instant::now();

        astar(
            self,
            |p| match timeout {
                Some(timeout) if Instant::now().duration_since(start) > timeout => vec![],
                _ => p.get_weighted_neighbors(&is_walkable),
            },
            |p| p.distance(to) as u32,
            |p| p.distance(to) <= 1.,
        )
    }

    /// Returns the positions of the tiles that are directly adjacent to the current tile.
    /// This includes all cardinal directions (N, S, E, W) and the diagonals (NE, NW, SE, SW).
    /// Each cardinal direction has a weight of 1, while the diagonals have a weight of 500.
    fn get_weighted_neighbors<F: Fn(&Self) -> bool + ?Sized>(
        &self,
        validator: &F,
    ) -> Vec<(Self, u32)> {
        let mut cardinal = [
            TilePosition::new(self.x + 1, self.y, 0),
            TilePosition::new(self.x - 1, self.y, 0),
            TilePosition::new(self.x, self.y + 1, 0),
            TilePosition::new(self.x, self.y - 1, 0),
        ]
        .iter()
        .map(|p| (*p, 1))
        .collect::<Vec<(TilePosition, u32)>>();

        cardinal.extend(
            [
                TilePosition::new(self.x + 1, self.y + 1, 0),
                TilePosition::new(self.x - 1, self.y - 1, 0),
                TilePosition::new(self.x + 1, self.y - 1, 0),
                TilePosition::new(self.x - 1, self.y + 1, 0),
            ]
            .iter()
            // TODO: Balance diagonals better
            .map(|p| (*p, 500))
            .collect::<Vec<(TilePosition, u32)>>(),
        );

        cardinal.into_iter().filter(|(p, _)| validator(p)).collect()
    }
}
