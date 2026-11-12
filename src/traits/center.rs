use super::{Coordinate, Euclidean};
use crate::core::Sin;
use crate::ops::CenterDistanceMap;

pub use topogen_derive::Center;

/// A spatial object with a well-defined center point in `D`-dimensional space.
pub trait Center<T: Coordinate, const D: usize>: Euclidean<T, D> {
    /// Returns the center coordinates of this object.
    fn center(&self) -> [T; D];

    /// Computes the Euclidean distance from `self.center()` to `point`.
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

    /// Wraps `self` into a generator yielding [`Self::center_distance`] at each coordinate.
    fn center_distance_map(self) -> CenterDistanceMap<Self, T, D> {
        CenterDistanceMap::new(self)
    }

    /// Creates a radial [`Sin`] wave generator centered at `self.center()`.
    fn center_sin(self) -> Sin<T, D> {
        Sin::new(self.center())
    }
}
