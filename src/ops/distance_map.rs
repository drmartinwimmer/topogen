use crate::traits::{Center, Coordinate, Euclidean, Generator, Shape};
use std::marker::PhantomData;

#[derive(Debug, Euclidean, Center)]
pub struct DistanceMap<S, T, const D: usize> {
    i: PhantomData<[T; D]>,
    #[wrap]
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

#[derive(Debug)]
pub struct CenterDistanceMap<C, T, const D: usize> {
    i: PhantomData<[T; D]>,
    pub center: C,
}

impl<C, T, const D: usize> CenterDistanceMap<C, T, D> {
    pub(crate) fn new(center: C) -> Self {
        Self {
            i: PhantomData,
            center,
        }
    }
}

impl<C, T, const D: usize> Generator<[T; D]> for CenterDistanceMap<C, T, D>
where
    T: Coordinate + num_traits::Float,
    C: Center<T, D>,
{
    type Output = T;

    fn generate(&self, input: [T; D]) -> Self::Output {
        self.center.center_distance(input)
    }
}

impl<E, T: Coordinate, const D: usize> Euclidean<T, D> for CenterDistanceMap<E, T, D> where
    E: Euclidean<T, D>
{
}

impl<C, T, const D: usize> Center<T, D> for CenterDistanceMap<C, T, D>
where
    T: Coordinate,
    C: Center<T, D>,
{
    fn center(&self) -> [T; D] {
        self.center.center()
    }
}
