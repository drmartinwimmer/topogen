use crate::{
    ops::{Union, UnionError},
    traits::{Bounded, Coordinate, Euclidean, Generator},
};

#[derive(Clone)]
pub struct GeneratorRef<'a, G> {
    generator: &'a G,
}

impl<'a, G> GeneratorRef<'a, G> {
    pub fn new(generator: &'a G) -> Self {
        Self { generator }
    }
}

impl<'a, G: Generator<I>, I> Generator<I> for GeneratorRef<'a, G> {
    type Output = G::Output;

    fn generate(&self, input: I) -> G::Output {
        self.generator.generate(input)
    }
}

impl<'a, G, T: Coordinate, const D: usize> Euclidean<T, D> for GeneratorRef<'a, G> where
    G: Euclidean<T, D>
{
}

impl<'a, G, T, const D: usize> Bounded<T, D> for GeneratorRef<'a, G>
where
    T: Coordinate,
    G: Bounded<T, D>,
{
    fn within(&self, input: [T; D]) -> bool {
        self.generator.within(input)
    }

    fn union<F>(
        self,
        mut other: Vec<Self>,
        reducer: F,
    ) -> Result<Union<Self, F, T, D>, UnionError<T>> {
        other.push(self);
        Union::new(other, reducer)
    }

    fn bounding_box(&self) -> crate::shapes::BoundingBox<T, D> {
        self.generator.bounding_box()
    }
}
