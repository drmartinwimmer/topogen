use super::{euclidean::Euclidean, Coordinate};
use crate::{
    ops::{Bitmap, Union, UnionError},
    shapes::{BoundingBox, WithBoundingBox},
};

pub trait Bounded<T, const D: usize>: Euclidean<T, D>
where
    T: Coordinate,
{
    fn within(&self, input: [T; D]) -> bool;
    fn bounding_box(&self) -> BoundingBox<T, D>;

    fn union<F>(
        self,
        mut others: Vec<Self>,
        reducer: F,
    ) -> Result<Union<Self, F, T, D>, UnionError<T>> {
        others.push(self);
        Union::new(others, reducer)
    }
    //fn intersect(self, other: Vec<Self>) -> Self;

    fn with_bounding_box(self, bounding_box: BoundingBox<T, D>) -> WithBoundingBox<Self, T, D> {
        WithBoundingBox::new(self, bounding_box)
    }

    fn bitmap(self) -> Bitmap<Self, T, D> {
        Bitmap::new(self)
    }
}
