use crate::traits::{Center, Coordinate, Euclidean, EuclideanVec, Generator};
use num_traits::One;
use std::{
    marker::PhantomData,
    ops::{AddAssign, Div, Mul, MulAssign},
};

pub struct Fbm<G, T, const D: usize>
where
    G: Generator<[T; D]> + Euclidean<T, D>,
    T: Coordinate,
{
    generator: G,
    octaves: usize,
    lacunarity: T,
    gain: G::Output,
    _phantom: PhantomData<(T, [u8; D])>,
}

impl<G, T, const D: usize> Fbm<G, T, D>
where
    G: Generator<[T; D]> + Euclidean<T, D>,
    T: Coordinate,
{
    pub fn new(generator: G, octaves: usize, lacunarity: T, gain: G::Output) -> Self {
        Self {
            generator,
            octaves,
            lacunarity,
            gain,
            _phantom: PhantomData,
        }
    }
}

impl<G, T, const D: usize> Generator<[T; D]> for Fbm<G, T, D>
where
    G: Generator<[T; D]> + Euclidean<T, D>,
    G::Output: Copy
        + Default
        + AddAssign
        + Mul<Output = G::Output>
        + MulAssign
        + Div<Output = G::Output>
        + One,
    T: Coordinate,
{
    type Output = G::Output;

    fn generate(&self, mut input: [T; D]) -> Self::Output {
        let mut output = Self::Output::default();
        let mut amplitude = Self::Output::one();
        let mut max_amplitude = Self::Output::one();
        for _ in 0..self.octaves {
            output += self.generator.generate(input) * amplitude;
            input.scalar_mul_assign(self.lacunarity);
            amplitude *= self.gain;
            max_amplitude += amplitude;
        }
        output / max_amplitude
    }
}

impl<G, T, const D: usize> Euclidean<T, D> for Fbm<G, T, D>
where
    G: Generator<[T; D]> + Euclidean<T, D>,
    T: Coordinate,
{
}

impl<G, T, const D: usize> Center<T, D> for Fbm<G, T, D>
where
    G: Generator<[T; D]> + Euclidean<T, D> + Center<T, D>,
    T: Coordinate,
{
    fn center(&self) -> [T; D] {
        self.generator.center()
    }
}
