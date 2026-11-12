/// Adapter wrapping `noise::NoiseFn` implementations as `topogen` generators.
#[cfg(feature = "noise_fn")]
pub mod noise_fn;
mod perlin;
mod permutation_table;
mod worley;

pub use perlin::{FixedGradientTable, GradientTable, ImprovedPerlin, Interpolation};
pub use permutation_table::PermutationTable;
pub use worley::Worley;
