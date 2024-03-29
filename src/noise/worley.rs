use std::fmt::Debug;

use num_traits::{Float, One, Zero};
use rand::{distributions::uniform::SampleUniform, Rng, SeedableRng};
use rand_pcg::Pcg32;

use crate::{
    ops::CenterDistanceMap,
    shapes::Point,
    traits::{Center, Coordinate, Euclidean, Generator},
};

use super::PermutationTable;

pub struct Worley<G, T, const D: usize> {
    distance_gen: G,
    cell_size: [T; D],
    permutation_table: PermutationTable,
    offsets: [[T; D]; 256],
}

impl<T, const D: usize> SeedableRng for Worley<CenterDistanceMap<Point<T, D>, T, D>, T, D>
where
    T: Coordinate + Zero + One + SampleUniform + Float,
{
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        Worley::new(
            seed,
            Point::new([T::zero(); D]).center_distance_map(),
            [T::one(); D],
        )
    }
}

impl<G, T, const D: usize> Worley<G, T, D>
where
    G: Generator<[T; D]>,
    G::Output: Float + Debug,
    T: Coordinate + Zero + SampleUniform + Float,
{
    pub fn new(seed: [u8; 32], distance_gen: G, cell_size: [T; D]) -> Self {
        let pt_seed: [u8; 16] = seed[0..16].try_into().unwrap();
        let mut rng = Pcg32::from_seed(pt_seed);
        let mut positions = [[T::zero(); D]; 256];
        positions.iter_mut().for_each(|p| {
            p.iter_mut()
                .for_each(|d| *d = rng.gen_range(T::zero()..T::one()))
        });
        Self {
            distance_gen,
            cell_size,
            permutation_table: PermutationTable::from_seed(seed[16..32].try_into().unwrap()),
            offsets: positions,
        }
    }

    pub fn set_cell_size(mut self, cell_size: [T; D]) -> Self {
        self.cell_size = cell_size;
        self
    }

    pub fn set_distance_gen<N>(self, distance_gen: N) -> Worley<N, T, D> {
        Worley {
            distance_gen,
            cell_size: self.cell_size,
            permutation_table: self.permutation_table,
            offsets: self.offsets,
        }
    }

    fn get_distance_for_cell(&self, cell: [u8; D], mut point: [T; D]) -> G::Output {
        let index = self.permutation_table.generate(cell);
        let offsets = &self.offsets[index as usize];
        point
            .iter_mut()
            .zip(offsets)
            .zip(self.cell_size)
            .for_each(|((p, o), s)| *p -= *o * s);
        self.distance_gen.generate(point)
    }

    fn get_combined_distance_for_cell_d(
        &self,
        mut cell: [u8; D],
        mut point: [T; D],
        d: usize,
    ) -> G::Output {
        if d == D {
            return self.get_distance_for_cell(cell, point);
        }
        let cell_d = cell[d];
        let point_d = point[d];
        let baseline = self.get_combined_distance_for_cell_d(cell, point, d + 1);
        cell[d] = cell_d.wrapping_add(1);
        point[d] = point_d - self.cell_size[d];
        let right = self.get_combined_distance_for_cell_d(cell, point, d + 1);
        cell[d] = cell_d.wrapping_sub(1);
        point[d] = point_d + self.cell_size[d];
        let left = self.get_combined_distance_for_cell_d(cell, point, d + 1);
        baseline.min(right).min(left)
    }

    fn get_combined_distance_for_cell(&self, cell: [u8; D], point: [T; D]) -> G::Output {
        self.get_combined_distance_for_cell_d(cell, point, 0)
    }
}

impl<G, T, const D: usize> Euclidean<T, D> for Worley<G, T, D> where T: Coordinate {}

trait WrapToU8 {
    fn wrap_to_u8(self) -> u8;
}

impl WrapToU8 for f32 {
    fn wrap_to_u8(self) -> u8 {
        self.rem_euclid(256.0) as u8
    }
}

impl WrapToU8 for f64 {
    fn wrap_to_u8(self) -> u8 {
        self.rem_euclid(256.0) as u8
    }
}

impl<G, T, const D: usize> Generator<[T; D]> for Worley<G, T, D>
where
    G: Generator<[T; D]>,
    G::Output: Float + Debug,
    T: Coordinate + Zero + SampleUniform + Float + WrapToU8,
{
    type Output = G::Output;

    fn generate(&self, mut point: [T; D]) -> Self::Output {
        let mut cell = point;
        cell.iter_mut()
            .zip(point)
            .zip(self.cell_size)
            .for_each(|((i, p), s)| *i = (p / s).floor());
        let mut cell_u8 = [0u8; D];
        cell_u8
            .iter_mut()
            .zip(cell)
            .for_each(|(c, p)| *c = p.wrap_to_u8());

        point
            .iter_mut()
            .zip(cell)
            .zip(self.cell_size)
            .for_each(|((p, c), s)| *p -= c * s);
        self.get_combined_distance_for_cell(cell_u8, point)
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use crate::traits::Generator;

    use super::Worley;

    #[test]
    fn no_sharp_edges_around_cell_boundaries() {
        let worley = Worley::seed_from_u64(12345).set_cell_size([10.0; 2]);
        let mut line = [0.0; 10];
        for (i, l) in line.iter_mut().enumerate() {
            *l = worley.generate([i as f32 * 0.1 - 0.5, 0.5 - i as f32 * 0.3]);
        }
        for w in line.windows(2) {
            assert!((w[0] - w[1]).abs() < 0.3);
        }
    }

    #[cfg(feature = "palette")]
    mod golden {
        use super::super::Worley;
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
        fn worley_golden() {
            let golden = Golden::new(Path::new("noise/worley.png"));
            let worley = Worley::seed_from_u64(12345)
                .set_cell_size([50.0; 2])
                .map(|d| Hsl::<encoding::Srgb, f32>::new(RgbHue::new(176.), 0.19, d / 100.0))
                .bound_to(Cuboid {
                    center: [0.0; 2],
                    extents: [300.0; 2],
                });
            worley.save_image(golden.write_path(), [1.0; 2]).unwrap();
            assert!(golden.is_identical());
        }
    }
}
