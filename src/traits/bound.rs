use super::{euclidean::Euclidean, Coordinate};
use crate::{
    geometry::object::BoundingBox,
    ops::{Union, UnionError},
    Generator,
};

pub trait Bounded<T, const D: usize>: Euclidean<T, D>
where
    T: Coordinate,
{
    type Bitmap: Generator<[T; D]>;

    fn bounding_box(&self) -> BoundingBox<T, D>;

    fn within(&self, input: [T; D]) -> bool;
    fn union<F>(self, other: Vec<Self>, reducer: F) -> Result<Union<T, Self, F, D>, UnionError<T>>;
    //fn intersect(self, other: Vec<Self>) -> Self;

    fn bitmap(self) -> Self::Bitmap;
}
