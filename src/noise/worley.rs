use crate::{
    ops::CenterDistanceMap,
    shapes::Point,
    traits::{Center, Coordinate, Euclidean, EuclideanVec, EuclideanVecFloat, Generator, WrapToU8},
};
use num_traits::{Float, One, Zero};
use rand::{Rng, SeedableRng, distributions::uniform::SampleUniform};
use rand_pcg::Pcg32;
use std::fmt::Debug;

use super::PermutationTable;

/// Seeded cellular (Worley / Voronoi) noise generator in `D`-dimensional Euclidean space.
#[derive(Clone, Debug, PartialEq, Eq)]
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
    /// Creates a new `Worley` noise generator from a 32-byte seed, custom distance generator, and `cell_size`.
    pub fn new(seed: [u8; 32], distance_gen: G, cell_size: [T; D]) -> Self {
        let (pt_seed, perm_seed) = seed.split_at(16);
        let mut rng = Pcg32::from_seed(pt_seed.try_into().expect("16-byte seed slice"));
        let mut positions = [[T::zero(); D]; 256];
        positions.iter_mut().for_each(|p| {
            p.iter_mut()
                .for_each(|d| *d = rng.gen_range(T::zero()..T::one()))
        });
        Self {
            distance_gen,
            cell_size,
            permutation_table: PermutationTable::from_seed(
                perm_seed.try_into().expect("16-byte seed slice"),
            ),
            offsets: positions,
        }
    }

    /// Returns a copy of this generator with the updated per-axis `cell_size`.
    pub fn set_cell_size(mut self, cell_size: [T; D]) -> Self {
        self.cell_size = cell_size;
        self
    }

    /// Replaces the distance metric generator evaluated relative to each cell's feature point.
    pub fn set_distance_gen<N>(self, distance_gen: N) -> Worley<N, T, D> {
        Worley {
            distance_gen,
            cell_size: self.cell_size,
            permutation_table: self.permutation_table,
            offsets: self.offsets,
        }
    }

    fn get_distance_for_cell(&self, cell: [u8; D], mut point: [T; D]) -> (G::Output, u8) {
        let index = self.permutation_table.generate(cell);
        if let Some(offsets) = self.offsets.get(usize::from(index)) {
            point
                .iter_mut()
                .zip(offsets)
                .zip(self.cell_size)
                .for_each(|((p, o), s)| *p -= *o * s);
        }
        (self.distance_gen.generate(point), index)
    }

    fn get_combined_distance_for_cell_d(
        &self,
        mut cell: [u8; D],
        mut point: [T; D],
        d: usize,
    ) -> (G::Output, u8) {
        let (Some(&cell_d), Some(&point_d), Some(&cell_size_d)) =
            (cell.get(d), point.get(d), self.cell_size.get(d))
        else {
            return self.get_distance_for_cell(cell, point);
        };
        let baseline = self.get_combined_distance_for_cell_d(cell, point, d + 1);
        if let Some(slot) = cell.get_mut(d) {
            *slot = cell_d.wrapping_add(1);
        }
        if let Some(slot) = point.get_mut(d) {
            *slot = point_d - cell_size_d;
        }
        let right = self.get_combined_distance_for_cell_d(cell, point, d + 1);
        if let Some(slot) = cell.get_mut(d) {
            *slot = cell_d.wrapping_sub(1);
        }
        if let Some(slot) = point.get_mut(d) {
            *slot = point_d + cell_size_d;
        }
        let left = self.get_combined_distance_for_cell_d(cell, point, d + 1);
        let min = match left.0 < baseline.0 {
            true => left,
            false => baseline,
        };
        match right.0 < min.0 {
            true => right,
            false => min,
        }
    }

    fn get_combined_distance_for_cell(&self, cell: [u8; D], point: [T; D]) -> (G::Output, u8) {
        self.get_combined_distance_for_cell_d(cell, point, 0)
    }
}

impl<G, T, const D: usize> Euclidean<T, D> for Worley<G, T, D> where T: Coordinate {}

impl<G, T, const D: usize> Generator<[T; D]> for Worley<G, T, D>
where
    G: Generator<[T; D]>,
    G::Output: Float + Debug,
    T: Coordinate + Zero + SampleUniform + Float + WrapToU8,
{
    type Output = (G::Output, u8);

    fn generate(&self, mut point: [T; D]) -> Self::Output {
        let cell = point.element_div(self.cell_size).floor();
        let cell_u8 = cell.wrap_to_u8();
        point.sub_assign(cell.element_mul(self.cell_size));
        self.get_combined_distance_for_cell(cell_u8, point)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "palette")]
    mod golden {
        use super::super::Worley;
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
        fn renders_worley_golden_image() -> Result<()> {
            let golden = Golden::new(Path::new("noise/worley.png"));
            let worley = Worley::seed_from_u64(12345)
                .set_cell_size([50.0; 2])
                .map(|(d, _)| Hsl::<encoding::Srgb, f32>::new(RgbHue::new(176.), 0.19, d / 100.0))
                .bound_to(Cuboid {
                    center: [0.0; 2],
                    extents: [300.0; 2],
                });
            worley.save_image(golden.write_path(), [1.0; 2])?;
            expect_that!(golden.is_identical(), is_true());
            Ok(())
        }
    }
}
