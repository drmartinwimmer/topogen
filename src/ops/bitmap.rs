use crate::traits::{Bounded, Coordinate, Euclidean, Generator};
use std::marker::PhantomData;

use super::{Union, UnionError};

#[derive(Debug)]
pub struct Bitmap<T, G, const D: usize> {
    i: PhantomData<[T; D]>,
    pub generator: G,
}

impl<T, G, const D: usize> Bitmap<T, G, D> {
    pub(crate) fn new(generator: G) -> Self {
        Self {
            i: PhantomData,
            generator,
        }
    }
}

impl<T, G, const D: usize> Generator<[T; D]> for Bitmap<T, G, D>
where
    T: Coordinate,
    G: Bounded<T, D>,
{
    type Output = bool;

    fn generate(&self, input: [T; D]) -> Self::Output {
        self.generator.within(input)
    }
}

impl<T: Coordinate, G, const D: usize> Euclidean<T, D> for Bitmap<T, G, D> {}

impl<T, G, const D: usize> Bounded<T, D> for Bitmap<T, G, D>
where
    T: Coordinate,
    G: Bounded<T, D>,
{
    type Bitmap = Self;

    fn within(&self, input: [T; D]) -> bool {
        self.generator.within(input)
    }

    fn union<F>(
        self,
        mut others: Vec<Self>,
        reducer: F,
    ) -> Result<Union<T, Self, F, D>, UnionError<T>> {
        others.push(self);
        Union::new(others, reducer)
    }

    // fn intersect(self, other: Self) -> Self {
    //     self.generator.intersect(other.generator).bitmap()
    // }

    // fn sub(self, other: Self) -> Self {
    //     self.generator.sub(other.generator).bitmap()
    // }

    fn bitmap(self) -> Self::Bitmap {
        self
    }

    fn bounding_box(&self) -> crate::geometry::object::BoundingBox<T, D> {
        self.generator.bounding_box()
    }
}
