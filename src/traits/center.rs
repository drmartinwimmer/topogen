use super::{Coordinate, Euclidean};

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
}
