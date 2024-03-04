use super::Generator;

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
    F: Fn(G::Output) -> O,
{
    type Output = O;

    fn generate(&self, input: I) -> Self::Output {
        (self.mapper)(self.generator.generate(input))
    }
}

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
    F: Fn(I) -> GI,
{
    type Output = G::Output;

    fn generate(&self, input: I) -> Self::Output {
        self.generator.generate((self.mapper)(input))
    }
}
