use super::{euclidean::Euclidean, Coordinate, Generator};
use crate::{
    ops::{Union, UnionError},
    shapes::{BoundingBox, WithBoundingBox},
};

pub trait Bounded<T, const D: usize>: Euclidean<T, D>
where
    T: Coordinate,
{
    type Bitmap: Generator<[T; D]>;

    fn within(&self, input: [T; D]) -> bool;
    fn union<F>(self, others: Vec<Self>, reducer: F)
        -> Result<Union<Self, F, T, D>, UnionError<T>>;
    //fn intersect(self, other: Vec<Self>) -> Self;

    fn bounding_box(&self) -> BoundingBox<T, D>;
    fn with_bounding_box(self, bounding_box: BoundingBox<T, D>) -> WithBoundingBox<Self, T, D> {
        WithBoundingBox::new(self, bounding_box)
    }
    fn bitmap(self) -> Self::Bitmap;
}
