use crate::traits::{Bounded, Center, Coordinate, Euclidean, Generator};

/// Clamps values produced by an inner [`Generator`] into `[min, max]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Euclidean, Center, Bounded)]
pub struct Clamp<G, I>
where
    G: Generator<I>,
{
    #[wrap]
    generator: G,
    min: G::Output,
    max: G::Output,
}

impl<G, I> Clamp<G, I>
where
    G: Generator<I>,
{
    /// Creates a clamping adapter bounding `generator` outputs to `[min, max]`.
    pub fn new(generator: G, min: G::Output, max: G::Output) -> Self {
        Self {
            generator,
            min,
            max,
        }
    }
}

impl<G, I> Generator<I> for Clamp<G, I>
where
    G: Generator<I>,
    G::Output: std::cmp::PartialOrd + Copy,
{
    type Output = G::Output;

    fn generate(&self, input: I) -> Self::Output {
        let out = self.generator.generate(input);
        if out < self.min {
            self.min
        } else if out > self.max {
            self.max
        } else {
            out
        }
    }
}
