use crate::traits::{Bounded, Center, Coordinate, Euclidean, Generator};

#[derive(Debug, Euclidean, Center)]
pub struct Clamp<G, I>
where
    G: Generator<I>,
{
    #[wrap]
    gen: G,
    min: G::Output,
    max: G::Output,
}

impl<G, I> Clamp<G, I>
where
    G: Generator<I>,
{
    pub fn new(gen: G, min: G::Output, max: G::Output) -> Self {
        Self { gen, min, max }
    }
}

impl<G, I> Generator<I> for Clamp<G, I>
where
    G: Generator<I>,
    G::Output: std::cmp::PartialOrd + Copy,
{
    type Output = G::Output;

    fn generate(&self, input: I) -> Self::Output {
        let out = self.gen.generate(input);
        if out < self.min {
            self.min
        } else if out > self.max {
            self.max
        } else {
            out
        }
    }
}

impl<G, T, const D: usize> Bounded<T, D> for Clamp<G, [T; D]>
where
    G: Bounded<T, D> + Generator<[T; D]>,
    T: Coordinate,
{
    fn within(&self, input: [T; D]) -> bool {
        self.gen.within(input)
    }

    fn bounding_box(&self) -> crate::shapes::BoundingBox<T, D> {
        self.gen.bounding_box()
    }
}
