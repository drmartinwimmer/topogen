use crate::{
    geometry::{object::BoundingBox, WithBoundingBox},
    Generator,
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

impl<'a, G: Clone + Generator<T>, T: Copy + Clone> Generator<T> for GeneratorRef<'a, G> {
    type Output = G::Output;

    fn generate(&self, input: T) -> G::Output {
        self.generator.generate(input)
    }
}

impl<'a, G, T, const D: usize> WithBoundingBox<T, D> for GeneratorRef<'a, G>
where
    G: WithBoundingBox<T, D>,
    T: Clone + Copy,
{
    fn bounding_box(&self) -> BoundingBox<T, D> {
        self.generator.bounding_box()
    }
}
