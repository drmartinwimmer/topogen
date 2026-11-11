use rand::{SeedableRng, seq::SliceRandom};
use rand_pcg::{Lcg64Xsh32, Pcg32};

use crate::traits::{Euclidean, Generator};

// 256-byte lookup table intentionally omits Copy to avoid implicit 256-byte stack copies.
#[expect(
    missing_copy_implementations,
    reason = "256-byte permutation table should not be copied implicitly"
)]
pub struct PermutationTable {
    table: [u8; 256],
}

impl SeedableRng for PermutationTable {
    type Seed = <Lcg64Xsh32 as SeedableRng>::Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        let mut table: [u8; 256] = std::array::from_fn(|i| u8::try_from(i).unwrap_or(0));
        let mut rng = Pcg32::from_seed(seed);
        table.shuffle(&mut rng);
        PermutationTable { table }
    }
}

impl<const D: usize> Euclidean<u8, D> for PermutationTable {}

impl<const D: usize> Generator<[u8; D]> for PermutationTable {
    type Output = u8;

    fn generate(&self, point: [u8; D]) -> Self::Output {
        point.iter().fold(0, |acc, c| {
            self.table
                .get(usize::from(acc.wrapping_add(*c)))
                .copied()
                .unwrap_or(0)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::PermutationTable;
    use crate::traits::Generator;
    use rand::SeedableRng;

    #[test]
    fn permutation_table_distributes_evenly_for_slice() {
        let pt = PermutationTable::seed_from_u64(12);
        let mut output = [0; 256];
        for i in 0..=255 {
            output[i as usize] = pt.generate([i, 13]);
        }
        output.sort();
        for (i, o) in output.iter().enumerate() {
            assert_eq!(*o, i as u8);
        }
    }

    #[test]
    fn permutation_table_dimensions_are_independent() {
        let pt = PermutationTable::seed_from_u64(12);
        assert_ne!(pt.generate([255, 254]), pt.generate([254, 255]));
        assert_ne!(pt.generate([3, 8]), pt.generate([8, 3]));
    }
}
