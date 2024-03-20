use crate::traits::{Center, Coordinate, Euclidean, Generator, Shape};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct DistanceMap<S, T, const D: usize> {
    i: PhantomData<[T; D]>,
    pub shape: S,
}

impl<S, T, const D: usize> DistanceMap<S, T, D> {
    pub(crate) fn new(shape: S) -> Self {
        Self {
            i: PhantomData,
            shape,
        }
    }
}

impl<S, T, const D: usize> Generator<[T; D]> for DistanceMap<S, T, D>
where
    T: Coordinate + num_traits::Float,
    S: Shape<T, D>,
{
    type Output = T;

    fn generate(&self, input: [T; D]) -> Self::Output {
        self.shape.distance(input)
    }
}

impl<E, T: Coordinate, const D: usize> Euclidean<T, D> for DistanceMap<E, T, D> where
    E: Euclidean<T, D>
{
}

impl<G, T, const D: usize> Center<T, D> for DistanceMap<G, T, D>
where
    T: Coordinate,
    G: Center<T, D>,
{
    fn center(&self) -> [T; D] {
        self.shape.center()
    }
}
