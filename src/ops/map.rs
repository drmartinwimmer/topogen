use crate::traits::{Bounded, Center, Coordinate, Euclidean, Generator, Shape};

#[derive(Debug, Euclidean, Center, Bounded, Shape)]
pub struct Map<G, F> {
    #[wrap]
    generator: G,
    mapper: F,
}

impl<G, F> Map<G, F> {
    pub fn new(generator: G, mapper: F) -> Self {
        Self { generator, mapper }
    }
}

impl<G, F, I, O> Generator<I> for Map<G, F>
where
    G: Generator<I>,
    F: Fn(G::Output) -> O + Clone,
{
    type Output = O;

    fn generate(&self, input: I) -> Self::Output {
        (self.mapper)(self.generator.generate(input))
    }
}

#[derive(Debug)]
pub struct MapInput<G, F> {
    generator: G,
    mapper: F,
}

impl<G, F> MapInput<G, F> {
    pub(crate) fn new(generator: G, mapper: F) -> Self {
        Self { generator, mapper }
    }
}

impl<G, F, I, GI> Generator<I> for MapInput<G, F>
where
    G: Generator<GI>,
    F: Fn(I) -> GI + Clone,
{
    type Output = G::Output;

    fn generate(&self, input: I) -> Self::Output {
        self.generator.generate((self.mapper)(input))
    }
}

impl<G, F, T, const D: usize> Euclidean<T, D> for MapInput<G, F>
where
    T: Coordinate,
    G: Euclidean<T, D>,
{
}

#[cfg(test)]
mod tests {
    use crate::core::Constant;
    use crate::traits::{ByRef, Generator};
    use googletest::prelude::*;

    #[googletest::test]
    fn map_by_value_works() {
        let generator = Constant::new(1);
        let mapped = generator.map(|x| x + 1);
        expect_that!(mapped.generate(0), eq(2));
    }

    #[googletest::test]
    fn map_with_different_output_type_works() {
        let generator = Constant::new(1_i16);
        let mapped = generator.map(|x| f32::from(x) / 2.0);
        expect_that!(mapped.generate(0), eq(0.5));
    }

    #[googletest::test]
    fn map_by_ref_works() {
        let generator = Constant::new(1);
        let mapped = generator.by_ref().map(|x| x + 1);
        expect_that!(mapped.generate(0), eq(2));
        expect_that!(generator.generate(0), eq(1));
    }
}
