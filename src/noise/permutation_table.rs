use rand::{SeedableRng, seq::SliceRandom};
use rand_pcg::{Lcg64Xsh32, Pcg32};

use crate::traits::{Euclidean, Generator};

/// Deterministic 256-entry permutation table mapping `D`-dimensional `u8` lattice coordinates to hashed indices.
#[derive(Clone, Debug, PartialEq, Eq)]
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
    use googletest::prelude::*;
    use rand::SeedableRng;

    #[googletest::test]
    fn permutation_table_distributes_evenly_for_slice() {
        let pt = PermutationTable::seed_from_u64(12);
        let mut output: [u8; 256] =
            std::array::from_fn(|i| pt.generate([u8::try_from(i).unwrap_or(0), 13]));
        output.sort_unstable();
        for (expected, actual) in (0_u8..=255).zip(output) {
            expect_that!(actual, eq(expected));
        }
    }

    #[googletest::test]
    fn permutation_table_dimensions_are_independent() {
        let pt = PermutationTable::seed_from_u64(12);
        expect_that!(pt.generate([255, 254]), not(eq(pt.generate([254, 255]))));
        expect_that!(pt.generate([3, 8]), not(eq(pt.generate([8, 3]))));
    }
}
