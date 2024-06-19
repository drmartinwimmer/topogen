use crate::traits::{Bounded, Center, Coordinate, Euclidean, Generator, Shape};
use std::marker::PhantomData;

#[derive(Euclidean, Center, Bounded, Shape)]
pub struct BoundTo<W, B, T, const D: usize>
where
    W: Euclidean<T, D>,
    B: Bounded<T, D>,
    T: Coordinate,
{
    t: PhantomData<T>,
    #[wrap]
    wrapped: W,
    bound_to: B,
}

impl<W, B, T, const D: usize> BoundTo<W, B, T, D>
where
    W: Euclidean<T, D>,
    B: Bounded<T, D>,
    T: Coordinate,
{
    pub fn new(wrapped: W, bound_to: B) -> Self {
        Self {
            t: PhantomData,
            wrapped,
            bound_to,
        }
    }
}

impl<G, B, T, const D: usize> Generator<[T; D]> for BoundTo<G, B, T, D>
where
    G: Generator<[T; D]> + Euclidean<T, D>,
    G::Output: Default,
    B: Bounded<T, D>,
    T: Coordinate,
{
    type Output = G::Output;

    fn generate(&self, point: [T; D]) -> Self::Output {
        if !self.bound_to.within(point) {
            return Self::Output::default();
        }
        self.wrapped.generate(point)
    }
}
