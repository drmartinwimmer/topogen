use rand::{seq::SliceRandom, SeedableRng};
use rand_pcg::Pcg32;

use crate::traits::{Euclidean, Generator};

pub struct PermutationTable {
    table: [u8; 256],
}

impl SeedableRng for PermutationTable {
    type Seed = <rand_pcg::Lcg64Xsh32 as rand::SeedableRng>::Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        let mut table = [0; 256];
        table.iter_mut().enumerate().for_each(|(i, v)| *v = i as u8);
        let mut rng = Pcg32::from_seed(seed);
        table.shuffle(&mut rng);
        PermutationTable { table }
    }
}

impl<const D: usize> Euclidean<u8, D> for PermutationTable {}

impl<const D: usize> Generator<[u8; D]> for PermutationTable {
    type Output = u8;

    fn generate(&self, point: [u8; D]) -> Self::Output {
        point.iter().fold(0, |acc, c| acc ^ self.table[*c as usize])
    }
}
