use super::{Bounded, Coordinate};
use crate::ops::{DistanceMap, SignedDistanceMap};

pub use topogen_derive::Shape;

pub trait Shape<T, const D: usize>: Bounded<T, D>
where
    T: Coordinate,
{
    fn distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float;

    fn signed_distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float;

    fn distance_map(self) -> DistanceMap<Self, T, D> {
        DistanceMap::new(self)
    }

    fn signed_distance_map(self) -> SignedDistanceMap<Self, T, D> {
        SignedDistanceMap::new(self)
    }
}
