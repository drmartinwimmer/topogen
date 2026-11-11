use crate::traits::{Coordinate, Euclidean, Generator};
use noise::{NoiseFn, Perlin as RawPerlin, Simplex as RawSimplex};
use rand::SeedableRng;

pub struct Noise<N>(N);
pub type Perlin = Noise<RawPerlin>;
pub type Simplex = Noise<RawSimplex>;

impl<N> Noise<N> {
    pub fn new(noise: N) -> Self {
        Self(noise)
    }
}

impl<N, T, const D: usize> Euclidean<T, D> for Noise<N>
where
    T: Coordinate,
    N: NoiseFn<T, D>,
{
}

impl SeedableRng for Noise<RawPerlin> {
    // The noise crate currently hard-codes a 32-bit seed.
    // Assuming https://github.com/Razaekel/noise-rs/issues/197 is fixed, this can be replaced.
    type Seed = [u8; 4];

    fn from_seed(seed: Self::Seed) -> Self {
        Noise::new(RawPerlin::new(u32::from_le_bytes(seed)))
    }
}

impl SeedableRng for Noise<RawSimplex> {
    // The noise crate currently hard-codes a 32-bit seed.
    // Assuming https://github.com/Razaekel/noise-rs/issues/197 is fixed, this can be replaced.
    type Seed = [u8; 4];

    fn from_seed(seed: Self::Seed) -> Self {
        Noise::new(RawSimplex::new(u32::from_le_bytes(seed)))
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

#[cfg(test)]
mod tests {
    #[cfg(feature = "palette")]
    mod golden {
        use crate::image::Image;
        use crate::noise::noise_fn::Noise;
        use crate::{
            shapes::Cuboid,
            testing::Golden,
            traits::{Euclidean, Generator},
        };
        use ::palette::{Hsl, RgbHue, encoding};
        use googletest::prelude::*;
        use rand::SeedableRng;
        use std::path::Path;

        #[googletest::test]
        fn renders_perlin_golden_image() -> Result<()> {
            let golden = Golden::new(Path::new("noise/perlin_noise_fn.png"));
            let perlin = Noise::<noise::Perlin>::seed_from_u64(12345)
                .scale([10.0; 2])
                .map(|d: f64| {
                    // Palette HSL expects f32 channels while noise::NoiseFn outputs f64.
                    #[expect(
                        clippy::cast_possible_truncation,
                        reason = "palette Hsl channels use f32 values"
                    )]
                    let lightness = (d as f32 + 1.0) / 2.0;
                    Hsl::<encoding::Srgb, f32>::new(RgbHue::new(176.), 0.19, lightness)
                })
                .bound_to(Cuboid {
                    center: [0.0; 2],
                    extents: [300.0; 2],
                });
            perlin.save_image(golden.write_path(), [1.0; 2])?;
            expect_that!(golden.is_identical(), is_true());
            Ok(())
        }
    }
}
