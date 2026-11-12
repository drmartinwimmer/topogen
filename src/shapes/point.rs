use crate::traits::{Center, Coordinate, Euclidean};

/// A single `D`-dimensional point in Euclidean space.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point<T, const D: usize> {
    /// Cartesian coordinates of the point.
    pub coordinates: [T; D],
}

impl<T, const D: usize> Point<T, D> {
    /// Creates a point at `coordinates`.
    pub fn new(coordinates: [T; D]) -> Point<T, D> {
        Point { coordinates }
    }
}

impl<T, const D: usize> Default for Point<T, D>
where
    T: Coordinate + num_traits::Zero,
{
    fn default() -> Self {
        Self {
            coordinates: [T::zero(); D],
        }
    }
}

impl<T, const D: usize> Center<T, D> for Point<T, D>
where
    T: Coordinate,
{
    fn center(&self) -> [T; D] {
        self.coordinates
    }
}

impl<T, const D: usize> Euclidean<T, D> for Point<T, D> where T: Coordinate {}
