mod fixed_gradient_table;

use self::fixed_gradient_table::FixedGradientTable;

use super::PermutationTable;
use crate::traits::{Coordinate, Euclidean, EuclideanVec, EuclideanVecFloat, Generator, WrapToU8};
use num_traits::Float;
use rand::SeedableRng;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Interpolation {
    #[default]
    Quintic,
    Smoothstep,
    Linear,
}

pub trait GradientTable<O: Float, const D: usize> {
    fn get(&self, entry: u8) -> [O; D];
}

struct Perlin<T: Coordinate + Float, const D: usize, G: GradientTable<T, D>> {
    gradient_table: G,
    permutation_table: PermutationTable,
    _output: PhantomData<T>,
}

fn interpolate_linear<T: Coordinate + Float>(a: T, b: T, x: T) -> T {
    a * (T::one() - x) + b * x
}

fn interpolate_smoothstep<T: Coordinate + Float>(a: T, b: T, x: T) -> T {
    let two = T::one() + T::one();
    let three = two + T::one();
    let weight = x * x * (three - two * x);
    a * (T::one() - weight) + b * weight
}

fn interpolate_quintic<T: Coordinate + Float>(a: T, b: T, x: T) -> T {
    let two = T::one() + T::one();
    let three = two + T::one();
    let five = three + two;
    let six = five + T::one();
    let ten = five + five;
    let fifteen = ten + five;
    let weight = x * x * x * (ten + x * (-fifteen + x * six));
    a * (T::one() - weight) + b * weight
}

impl<T: Coordinate + Float + WrapToU8, const D: usize, G: GradientTable<T, D>> Perlin<T, D, G> {
    pub fn new(seed: [u8; 16], gradient_table: G) -> Self {
        Self {
            gradient_table,
            permutation_table: PermutationTable::from_seed(seed),
            _output: PhantomData,
        }
    }

    fn get_corner_value(&self, corner: [T; D], point: &[T; D]) -> T {
        let corner_u8 = corner.wrap_to_u8();
        let perm = self.permutation_table.generate(corner_u8);
        let gradient = self.gradient_table.get(perm);
        let offset = point.sub(corner);
        gradient.dot(offset)
    }

    fn get_corner_value_d(
        &self,
        mut corner: [T; D],
        point: &[T; D],
        d: usize,
        interpolate: fn(T, T, T) -> T,
    ) -> T {
        let (Some(&point_d), Some(&corner_d)) = (point.get(d), corner.get(d)) else {
            return self.get_corner_value(corner, point);
        };
        let a = self.get_corner_value_d(corner, point, d + 1, interpolate);
        let corner_next = corner_d + T::one();
        if let Some(slot) = corner.get_mut(d) {
            *slot = corner_next;
        }
        let b = self.get_corner_value_d(corner, point, d + 1, interpolate);
        interpolate(a, b, point_d + T::one() - corner_next)
    }

    pub fn generate(&self, point: [T; D], interpolation: Interpolation) -> T {
        let interpolate = match interpolation {
            Interpolation::Quintic => interpolate_quintic,
            Interpolation::Smoothstep => interpolate_smoothstep,
            Interpolation::Linear => interpolate_linear,
        };
        let corner = point.floor();
        self.get_corner_value_d(corner, &point, 0, interpolate)
    }
}

pub struct ImprovedPerlin<
    T: Coordinate + Float,
    const D: usize,
    G: GradientTable<T, D> = FixedGradientTable<T, D>,
> {
    perlin: Perlin<T, D, G>,
    interpolation: Interpolation,
}

impl<T: Coordinate + Float, const D: usize, G: GradientTable<T, D>> ImprovedPerlin<T, D, G> {
    pub fn with_interpolation(mut self, interpolation: Interpolation) -> Self {
        self.interpolation = interpolation;
        self
    }

    pub fn interpolation(&self) -> Interpolation {
        self.interpolation
    }
}

impl<T: Coordinate + Float + WrapToU8> ImprovedPerlin<T, 1, FixedGradientTable<T, 1>> {
    pub fn new(seed: [u8; 16]) -> Self {
        Self {
            perlin: Perlin::new(seed, FixedGradientTable::new()),
            interpolation: Interpolation::default(),
        }
    }
}

impl<T: Coordinate + Float + WrapToU8> ImprovedPerlin<T, 2, FixedGradientTable<T, 2>> {
    pub fn new(seed: [u8; 16]) -> Self {
        Self {
            perlin: Perlin::new(seed, FixedGradientTable::new()),
            interpolation: Interpolation::default(),
        }
    }
}

impl<T: Coordinate + Float + WrapToU8> ImprovedPerlin<T, 3, FixedGradientTable<T, 3>> {
    pub fn new(seed: [u8; 16]) -> Self {
        Self {
            perlin: Perlin::new(seed, FixedGradientTable::new()),
            interpolation: Interpolation::default(),
        }
    }
}

impl<T: Coordinate + Float + WrapToU8, const D: usize, G: GradientTable<T, D>> Generator<[T; D]>
    for ImprovedPerlin<T, D, G>
{
    type Output = T;

    fn generate(&self, point: [T; D]) -> Self::Output {
        self.perlin.generate(point, self.interpolation)
    }
}

impl<T> SeedableRng for ImprovedPerlin<T, 1, FixedGradientTable<T, 1>>
where
    T: Coordinate + Float + WrapToU8,
{
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(seed)
    }
}

impl<T> SeedableRng for ImprovedPerlin<T, 2, FixedGradientTable<T, 2>>
where
    T: Coordinate + Float + WrapToU8,
{
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(seed)
    }
}

impl<T> SeedableRng for ImprovedPerlin<T, 3, FixedGradientTable<T, 3>>
where
    T: Coordinate + Float + WrapToU8,
{
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(seed)
    }
}

impl<T, const D: usize, G> Euclidean<T, D> for ImprovedPerlin<T, D, G>
where
    T: Coordinate + Float,
    G: GradientTable<T, D>,
{
}

#[cfg(test)]
mod tests {
    use super::{ImprovedPerlin, Interpolation};
    use crate::traits::Generator;
    use googletest::prelude::*;
    use rand::SeedableRng;

    #[googletest::test]
    fn improved_perlin_supports_configurable_interpolation() {
        let quintic: ImprovedPerlin<f64, 2> = ImprovedPerlin::seed_from_u64(42);
        let smoothstep: ImprovedPerlin<f64, 2> =
            ImprovedPerlin::seed_from_u64(42).with_interpolation(Interpolation::Smoothstep);
        let linear: ImprovedPerlin<f64, 2> =
            ImprovedPerlin::seed_from_u64(42).with_interpolation(Interpolation::Linear);

        expect_that!(quintic.interpolation(), eq(Interpolation::Quintic));
        expect_that!(smoothstep.interpolation(), eq(Interpolation::Smoothstep));
        expect_that!(linear.interpolation(), eq(Interpolation::Linear));

        // At lattice midpoints (x = 0.5), quintic, smoothstep, and linear weights all equal 0.5;
        // off-midpoint coordinates produce distinct values across interpolation curves.
        let sample = [0.25, 0.7];
        let q = quintic.generate(sample);
        let s = smoothstep.generate(sample);
        let l = linear.generate(sample);
        expect_that!((q - s).abs(), gt(1e-6));
        expect_that!((s - l).abs(), gt(1e-6));
    }

    #[cfg(feature = "palette")]
    mod golden {
        use super::super::ImprovedPerlin;
        use crate::image::Image;
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
        fn improved_perlin_golden() -> Result<()> {
            let golden = Golden::new(Path::new("noise/improved_perlin.png"));
            let perlin = ImprovedPerlin::seed_from_u64(12345)
                .scale([10.0; 2])
                .map(|d| Hsl::<encoding::Srgb, f32>::new(RgbHue::new(176.), 0.19, (d + 1.0) / 2.0))
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
