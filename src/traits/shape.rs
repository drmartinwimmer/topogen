use super::{Bounded, Coordinate};
use crate::ops::{DistanceMap, SignedDistanceMap};

pub use topogen_derive::Shape;

/// A bounded geometric shape supporting unsigned exterior distance and signed distance queries.
pub trait Shape<T, const D: usize>: Bounded<T, D>
where
    T: Coordinate,
{
    /// Returns the unsigned exterior Euclidean distance to `self` (`0` inside or on the boundary).
    fn distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float;

    /// Returns the signed Euclidean distance to `self`'s boundary (negative inside, `0` on the
    /// surface, positive outside).
    fn signed_distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float;

    /// Wraps `self` into a generator evaluating [`Self::distance`] at each coordinate.
    fn distance_map(self) -> DistanceMap<Self, T, D> {
        DistanceMap::new(self)
    }

    /// Wraps `self` into a generator evaluating [`Self::signed_distance`] at each coordinate.
    fn signed_distance_map(self) -> SignedDistanceMap<Self, T, D> {
        SignedDistanceMap::new(self)
    }
}
