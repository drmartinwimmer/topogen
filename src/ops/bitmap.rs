use crate::traits::{Bounded, Center, Coordinate, Euclidean, Generator, Shape};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Bitmap<G, T, const D: usize> {
    i: PhantomData<[T; D]>,
    pub generator: G,
}

impl<G, T, const D: usize> Bitmap<G, T, D> {
    pub(crate) fn new(generator: G) -> Self {
        Self {
            i: PhantomData,
            generator,
        }
    }
}

impl<T, G, const D: usize> Generator<[T; D]> for Bitmap<G, T, D>
where
    T: Coordinate,
    G: Bounded<T, D>,
{
    type Output = bool;

    fn generate(&self, input: [T; D]) -> Self::Output {
        self.generator.within(input)
    }
}

impl<T: Coordinate, G, const D: usize> Euclidean<T, D> for Bitmap<G, T, D> where G: Bounded<T, D> {}

impl<T, G, const D: usize> Bounded<T, D> for Bitmap<G, T, D>
where
    T: Coordinate,
    G: Bounded<T, D>,
{
    fn within(&self, input: [T; D]) -> bool {
        self.generator.within(input)
    }

    // fn intersect(self, other: Self) -> Self {
    //     self.generator.intersect(other.generator).bitmap()
    // }

    // fn sub(self, other: Self) -> Self {
    //     self.generator.sub(other.generator).bitmap()
    // }

    fn bounding_box(&self) -> crate::shapes::BoundingBox<T, D> {
        self.generator.bounding_box()
    }
}

impl<G, T, const D: usize> Center<T, D> for Bitmap<G, T, D>
where
    T: Coordinate,
    G: Bounded<T, D> + Center<T, D>,
{
    fn center(&self) -> [T; D] {
        self.generator.center()
    }
}

impl<G, T, const D: usize> Shape<T, D> for Bitmap<G, T, D>
where
    T: Coordinate,
    G: Shape<T, D>,
{
    fn distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float,
    {
        self.generator.distance(point)
    }
}
