use crate::traits::{Center, Coordinate, Euclidean, EuclideanVec, Generator};
use num_traits::One;
use std::{
    marker::PhantomData,
    ops::{AddAssign, Div, Mul, MulAssign},
};

#[derive(Euclidean)]
pub struct Fbm<G, T, const D: usize>
where
    G: Generator<[T; D]> + Euclidean<T, D>,
    T: Coordinate,
{
    #[wrap]
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
        if self.octaves == 0 {
            return Self::Output::default();
        }
        let mut output = Self::Output::default();
        let mut amplitude = Self::Output::one();
        let mut max_amplitude = Self::Output::default();
        for _ in 0..self.octaves {
            output += self.generator.generate(input) * amplitude;
            max_amplitude += amplitude;
            input.scalar_mul_assign(self.lacunarity);
            amplitude *= self.gain;
        }
        output / max_amplitude
    }
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

#[cfg(test)]
mod tests {
    use crate::{
        core::Constant,
        shapes::Point,
        traits::{Center, Euclidean, Generator},
    };
    use googletest::prelude::*;

    #[googletest::test]
    fn fbm_with_zero_octaves_returns_default() {
        let fbm = Constant::new(4.0_f64).fbm(0, 2.0_f64, 0.5);
        expect_that!(fbm.generate([1.0, 2.0]), eq(0.0));
    }

    #[googletest::test]
    fn fbm_with_single_octave_preserves_amplitude() {
        let fbm = Constant::new(3.0_f64).fbm(1, 2.0_f64, 0.5);
        expect_that!(fbm.generate([1.0, 2.0]), eq(3.0));
    }

    #[googletest::test]
    fn fbm_normalizes_weighted_octaves_and_scales_frequency() {
        let linear = Point::new([0.0_f64]).center_distance_map();
        // At x = 1.0 with lacunarity = 2.0, gain = 0.5, octaves = 3:
        // Octave 0: linear(1.0) * 1.0  = 1.0
        // Octave 1: linear(2.0) * 0.5  = 1.0
        // Octave 2: linear(4.0) * 0.25 = 1.0
        // Sum = 3.0, max_amplitude = 1.0 + 0.5 + 0.25 = 1.75
        let fbm = linear.fbm(3, 2.0, 0.5);
        expect_that!(fbm.center(), eq([0.0]));
        expect_that!(fbm.generate([1.0]), near(3.0 / 1.75, 1e-12));
    }
}
