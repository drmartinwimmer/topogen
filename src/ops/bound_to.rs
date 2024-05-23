use std::marker::PhantomData;

use crate::{
    shapes::BoundingBox,
    traits::{Bounded, Center, Coordinate, Euclidean, Generator, Shape},
};

#[derive(Euclidean)]
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

impl<W, B, T, const D: usize> Bounded<T, D> for BoundTo<W, B, T, D>
where
    W: Euclidean<T, D>,
    B: Bounded<T, D>,
    T: Coordinate,
{
    fn bounding_box(&self) -> BoundingBox<T, D> {
        self.bound_to.bounding_box()
    }

    fn within(&self, point: [T; D]) -> bool {
        self.bound_to.within(point)
    }
}

impl<C, B, T, const D: usize> Center<T, D> for BoundTo<C, B, T, D>
where
    C: Center<T, D>,
    B: Bounded<T, D>,
    T: Coordinate,
{
    fn center(&self) -> [T; D] {
        self.wrapped.center()
    }
}

impl<S, B, T, const D: usize> Shape<T, D> for BoundTo<S, B, T, D>
where
    S: Shape<T, D>,
    B: Bounded<T, D>,
    T: Coordinate,
{
    fn distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float,
    {
        self.wrapped.distance(point)
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
