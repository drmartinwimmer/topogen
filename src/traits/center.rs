use super::{Coordinate, Euclidean};
use crate::core::Sin;
use crate::ops::CenterDistanceMap;

pub trait Center<T: Coordinate, const D: usize>: Euclidean<T, D> {
    fn center(&self) -> [T; D];

    fn center_distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float,
    {
        let distance_square: T = point
            .into_iter()
            .zip(self.center())
            .map(|(p, c)| (p - c) * (p - c))
            .sum();
        distance_square.sqrt()
    }

    fn center_distance_map(self) -> CenterDistanceMap<Self, T, D> {
        CenterDistanceMap::new(self)
    }

    fn center_sin(self) -> Sin<T, D> {
        Sin::new(self.center())
    }
}
