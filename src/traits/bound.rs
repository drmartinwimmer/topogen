use super::{Coordinate, euclidean::Euclidean};
use crate::{
    ops::{Bitmap, Intersect, IntersectError, Union, UnionError, WithBoundingBox},
    shapes::BoundingBox,
};

pub use topogen_derive::Bounded;

/// A spatial object or generator with membership predicate and conservative axis-aligned bounds.
pub trait Bounded<T, const D: usize>: Euclidean<T, D>
where
    T: Coordinate,
{
    /// Returns `true` when `input` lies inside this object's spatial domain.
    fn within(&self, input: [T; D]) -> bool;

    /// Returns a conservative axis-aligned [`BoundingBox`] enclosing all points where [`Self::within`] holds.
    fn bounding_box(&self) -> BoundingBox<T, D>;

    /// Combines `self` with `others` over the union of their domains using `reducer`.
    fn union<F>(
        self,
        mut others: Vec<Self>,
        reducer: F,
    ) -> Result<Union<Self, F, T, D>, UnionError<T>> {
        others.push(self);
        Union::new(others, reducer)
    }

    /// Combines `self` with `others` over the intersection of their domains using `reducer`.
    fn intersect<F>(
        self,
        others: Vec<Self>,
        reducer: F,
    ) -> Result<Intersect<Self, F, T, D>, IntersectError<T>> {
        let mut elements = Vec::with_capacity(others.len() + 1);
        elements.push(self);
        elements.extend(others);
        Intersect::new(elements, reducer)
    }

    /// Restricts this object's reported bounds and evaluation domain to `bounding_box`.
    fn with_bounding_box(self, bounding_box: BoundingBox<T, D>) -> WithBoundingBox<Self, T, D> {
        WithBoundingBox::new(self, bounding_box)
    }

    /// Converts this bounded object into a boolean [`Bitmap`] generator yielding `self.within(p)`.
    fn bitmap(self) -> Bitmap<Self, T, D> {
        Bitmap::new(self)
    }
}
