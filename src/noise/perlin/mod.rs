mod fixed_gradient_table;

use self::fixed_gradient_table::FixedGradientTable;

use super::PermutationTable;
use crate::traits::{Coordinate, Euclidean, EuclideanVec, EuclideanVecFloat, Generator, WrapToU8};
use num_traits::Float;
use rand::SeedableRng;
use std::marker::PhantomData;

pub trait GradientTable<O: Float, const D: usize> {
    fn get(&self, entry: u8) -> [O; D];
}

struct Perlin<T: Coordinate + Float, const D: usize, G: GradientTable<T, D>> {
    gradient_table: G,
    permutation_table: PermutationTable,
    _output: PhantomData<T>,
}

#[allow(dead_code)]
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
        // println!(
        //     "Point: {:?},Corner: {:?}, perm: {}, gradient: {:?}",
        //     point, corner, perm, gradient,
        // );
        let offset = point.sub(corner);
        // println!(
        //     "Gradient: {:?}, offset: {:?}, dot: {}",
        //     gradient,
        //     offset,
        //     gradient.dot(offset)
        // );
        gradient.dot(offset)
    }

    fn get_corner_value_d(
        &self,
        mut corner: [T; D],
        point: &[T; D],
        d: usize,
        interpolate: fn(T, T, T) -> T,
    ) -> T {
        // if d == 0 {
        //     println!(
        //         "Get Corner Value corner: {:?}, point: {:?}, d: {}",
        //         corner, point, d
        //     );
        // }
        if d == D {
            return self.get_corner_value(corner, point);
        }
        let a = self.get_corner_value_d(corner, point, d + 1, interpolate);
        corner[d] += T::one();
        let b = self.get_corner_value_d(corner, point, d + 1, interpolate);
        interpolate(a, b, point[d] + T::one() - corner[d])
    }

    pub fn generate(&self, point: [T; D], interpolate: fn(T, T, T) -> T) -> T {
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
}

impl<T: Coordinate + Float + WrapToU8> ImprovedPerlin<T, 1, FixedGradientTable<T, 1>> {
    pub fn new(seed: [u8; 16]) -> Self {
        Self {
            perlin: Perlin::new(seed, FixedGradientTable::new()),
        }
    }
}

impl<T: Coordinate + Float + WrapToU8> ImprovedPerlin<T, 2, FixedGradientTable<T, 2>> {
    pub fn new(seed: [u8; 16]) -> Self {
        Self {
            perlin: Perlin::new(seed, FixedGradientTable::new()),
        }
    }
}

impl<T: Coordinate + Float + WrapToU8> ImprovedPerlin<T, 3, FixedGradientTable<T, 3>> {
    pub fn new(seed: [u8; 16]) -> Self {
        Self {
            perlin: Perlin::new(seed, FixedGradientTable::new()),
        }
    }
}

impl<T: Coordinate + Float + WrapToU8, const D: usize, G: GradientTable<T, D>> Generator<[T; D]>
    for ImprovedPerlin<T, D, G>
{
    type Output = T;

    fn generate(&self, point: [T; D]) -> Self::Output {
        self.perlin.generate(point, interpolate_quintic)
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
    use rand::SeedableRng;

    use crate::traits::Generator;

    use super::ImprovedPerlin;

    #[test]
    fn no_sharp_edges_around_cell_boundaries() {
        let perlin = ImprovedPerlin::seed_from_u64(12345);
        let mut line = [0.0; 10];
        for (i, l) in line.iter_mut().enumerate() {
            *l = perlin.generate([i as f32 * 0.1 - 0.5, 0.55 - i as f32 * 0.3]);
        }
        for w in line.windows(2) {
            println!("{} {}", w[0], w[1]);
            assert!((w[0] - w[1]).abs() < 0.31);
        }
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
        use ::palette::{encoding, Hsl, RgbHue};
        use rand::SeedableRng;
        use std::path::Path;

        #[test]
        fn improved_perlin_golden() {
            let golden = Golden::new(Path::new("noise/improved_perlin.png"));
            let perlin = ImprovedPerlin::seed_from_u64(12345)
                .scale([10.0; 2])
                .map(|d| Hsl::<encoding::Srgb, f32>::new(RgbHue::new(176.), 0.19, (d + 1.0) / 2.0))
                .bound_to(Cuboid {
                    center: [0.0; 2],
                    extents: [300.0; 2],
                });
            perlin.save_image(golden.write_path(), [1.0; 2]).unwrap();
            assert!(golden.is_identical());
        }
    }
}
