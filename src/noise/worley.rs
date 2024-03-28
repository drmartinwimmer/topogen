use num_traits::{AsPrimitive, Float, One, Zero};
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
    G::Output: Float,
    T: Coordinate + Zero + SampleUniform + Float,
{
    pub fn new(seed: [u8; 32], distance_gen: G, cell_size: [T; D]) -> Self {
        let pt_seed: [u8; 16] = seed[0..16].try_into().unwrap();
        let mut rng = Pcg32::from_seed(pt_seed);
        let mut positions = [[T::zero(); D]; 256];
        for p in positions.iter_mut() {
            for j in 0..D {
                p[j] = rng.gen_range(T::zero()..cell_size[j]);
            }
        }
        Self {
            distance_gen,
            cell_size,
            permutation_table: PermutationTable::from_seed(seed[16..32].try_into().unwrap()),
            offsets: positions,
        }
    }

    fn get_distance_for_cell(&self, cell: [u8; D], mut point: [T; D]) -> G::Output {
        let index = self.permutation_table.generate(cell);
        let offsets = self.offsets[index as usize];
        point.iter_mut().zip(offsets).for_each(|(p, o)| *p -= o);
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
        let baseline = self.get_combined_distance_for_cell_d(cell, point, d + 1);
        cell[d] += 1;
        point[d] -= T::one();
        let right = self.get_combined_distance_for_cell_d(cell, point, d + 1);
        cell[d] -= 2;
        point[d] += T::one() + T::one();
        let left = self.get_combined_distance_for_cell_d(cell, point, d + 1);
        baseline.min(right).min(left)
    }

    fn get_combined_distance_for_cell(&self, cell: [u8; D], point: [T; D]) -> G::Output {
        self.get_combined_distance_for_cell_d(cell, point, 0)
    }
}

impl<G, T, const D: usize> Euclidean<T, D> for Worley<G, T, D> where T: Coordinate {}

impl<G, T, const D: usize> Generator<[T; D]> for Worley<G, T, D>
where
    G: Generator<[T; D]>,
    G::Output: Float,
    T: Coordinate + Zero + SampleUniform + Float + AsPrimitive<u8>,
{
    type Output = G::Output;

    fn generate(&self, mut point: [T; D]) -> Self::Output {
        let mut cell = point;
        cell.iter_mut()
            .zip(point)
            .zip(self.cell_size)
            .for_each(|((i, p), s)| *i = (p / s).floor());
        let mut cell_u8 = [0u8; D];
        cell_u8.iter_mut().zip(cell).for_each(|(c, p)| *c = p.as_());

        point
            .iter_mut()
            .zip(cell)
            .zip(self.cell_size)
            .for_each(|((p, c), s)| *p = (*p / s) - c);
        self.get_combined_distance_for_cell(cell_u8, point)
    }
}
