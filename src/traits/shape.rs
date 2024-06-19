use super::{Bounded, Coordinate};
use crate::ops::DistanceMap;

pub use generative_map_derive::Shape;

pub trait Shape<T, const D: usize>: Bounded<T, D>
where
    T: Coordinate,
{
    fn distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float;

    fn distance_map(self) -> DistanceMap<Self, T, D> {
        DistanceMap::new(self)
    }
}
