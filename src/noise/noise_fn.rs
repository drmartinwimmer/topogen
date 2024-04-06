use super::Noise;
use crate::traits::{Coordinate, Generator};
use noise::NoiseFn;

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
        use crate::noise::Noise;
        use crate::{
            shapes::Cuboid,
            testing::Golden,
            traits::{Euclidean, Generator},
        };
        use ::palette::{encoding, Hsl, RgbHue};
        use rand::SeedableRng;
        use std::path::Path;

        #[test]
        fn perlin_golden() {
            let golden = Golden::new(Path::new("noise/perlin_noise_fn.png"));
            let perlin = Noise::<noise::Perlin>::seed_from_u64(12345)
                .scale([10.0; 2])
                .map(|d: f64| {
                    Hsl::<encoding::Srgb, f32>::new(RgbHue::new(176.), 0.19, (d as f32 + 1.0) / 2.0)
                })
                .bound_to(Cuboid {
                    center: [0.0; 2],
                    extents: [300.0; 2],
                });
            perlin.save_image(golden.write_path(), [1.0; 2]).unwrap();
            assert!(golden.is_identical());
        }
    }
}
