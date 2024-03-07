use crate::geometry::{object::BoundingBox, WithBoundingBox};

use super::Generator;

#[derive(Debug, Clone)]
pub struct MapOutput<G, F> {
    generator: G,
    mapper: F,
}

impl<G, F> MapOutput<G, F> {
    pub(crate) fn new(generator: G, mapper: F) -> Self {
        Self { generator, mapper }
    }
}

impl<G, F, I, O> Generator<I> for MapOutput<G, F>
where
    G: Generator<I>,
    F: Fn(G::Output) -> O + Clone,
{
    type Output = O;

    fn generate(&self, input: I) -> Self::Output {
        (self.mapper)(self.generator.generate(input))
    }
}

impl<G, F, T, const D: usize> WithBoundingBox<T, D> for MapOutput<G, F>
where
    G: WithBoundingBox<T, D>,
    T: Clone + Copy,
{
    fn bounding_box(&self) -> BoundingBox<T, D> {
        self.generator.bounding_box()
    }
}

#[derive(Debug, Clone)]
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

impl<G, F, T, const D: usize> WithBoundingBox<T, D> for MapInput<G, F>
where
    G: WithBoundingBox<T, D>,
    T: Clone + Copy,
{
    fn bounding_box(&self) -> BoundingBox<T, D> {
        self.generator.bounding_box()
    }
}
