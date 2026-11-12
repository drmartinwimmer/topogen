use crate::traits::{Bounded, Center, Coordinate, Euclidean, Generator, Shape};
use std::marker::PhantomData;

/// Adapts a [`Bounded`] object into a boolean [`Generator`] returning `true` inside its domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Euclidean, Bounded, Center, Shape)]
pub struct Bitmap<G, T, const D: usize> {
    i: PhantomData<[T; D]>,
    /// Underlying bounded object queried via [`Bounded::within`].
    #[wrap]
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
