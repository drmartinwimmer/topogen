use crate::traits::{Coordinate, Euclidean, Generator};
use noise::NoiseFn;
use rand::SeedableRng;

pub struct Noise<N>(N);
pub type Perlin = Noise<noise::Perlin>;
pub type Simplex = Noise<noise::Simplex>;

impl<N> Noise<N> {
    pub fn new(noise: N) -> Self {
        Self(noise)
    }
}

impl<N, T, const D: usize> Generator<[T; D]> for Noise<N>
where
    T: Coordinate,
    N: NoiseFn<T, D>,
{
    type Output = f64;

    fn generate(&self, point: [T; D]) -> Self::Output {
        self.0.get(point)
    }
}

impl<N, T, const D: usize> Euclidean<T, D> for Noise<N>
where
    T: Coordinate,
    N: NoiseFn<T, D>,
{
}

impl SeedableRng for Noise<noise::Perlin> {
    // The noise crate currently hard-codes a 32-bit seed.
    // Assuming https://github.com/Razaekel/noise-rs/issues/197 is fixed, this can be replaced.
    type Seed = [u8; 4];

    fn from_seed(seed: Self::Seed) -> Self {
        Noise::new(noise::Perlin::new(u32::from_le_bytes(seed)))
    }
}

impl SeedableRng for Noise<noise::Simplex> {
    // The noise crate currently hard-codes a 32-bit seed.
    // Assuming https://github.com/Razaekel/noise-rs/issues/197 is fixed, this can be replaced.
    type Seed = [u8; 4];

    fn from_seed(seed: Self::Seed) -> Self {
        Noise::new(noise::Simplex::new(u32::from_le_bytes(seed)))
    }
}
