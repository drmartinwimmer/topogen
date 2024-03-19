use crate::{
    shapes::BoundingBox,
    traits::{Bounded, Center, Coordinate, Euclidean, Generator, Shape},
};

#[derive(Debug)]
pub struct Map<G, F> {
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

impl<G, F, T, const D: usize> Euclidean<T, D> for Map<G, F>
where
    T: Coordinate,
    G: Euclidean<T, D>,
{
}

impl<G, F, T, const D: usize> Bounded<T, D> for Map<G, F>
where
    T: Coordinate,
    G: Bounded<T, D>,
    F: Clone,
{
    fn within(&self, input: [T; D]) -> bool {
        self.generator.within(input)
    }

    fn bounding_box(&self) -> BoundingBox<T, D> {
        self.generator.bounding_box()
    }
}

impl<G, F, T, const D: usize> Center<T, D> for Map<G, F>
where
    T: Coordinate,
    G: Center<T, D>,
    F: Clone,
{
    fn center(&self) -> &[T; D] {
        self.generator.center()
    }
}

impl<G, F, T, const D: usize> Shape<T, D> for Map<G, F>
where
    T: Coordinate,
    G: Shape<T, D> + Generator<[T; D]>,
    F: Clone,
{
    fn distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float,
    {
        self.generator.distance(point)
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
    use crate::traits::Generator;

    #[test]
    fn map_by_value_works() {
        let gen = Constant::new(1);
        let mapped = gen.map(|x| x + 1);
        assert_eq!(mapped.generate(0), 2);
    }

    #[test]
    fn map_with_different_output_type_works() {
        let gen = Constant::new(1);
        let mapped = gen.map(|x| x as f32 / 2.0);
        assert_eq!(mapped.generate(0), 0.5);
    }

    #[test]
    fn map_by_ref_works() {
        let gen = Constant::new(1);
        let mapped = gen.as_ref().map(|x| x + 1);
        assert_eq!(mapped.generate(0), 2);
        assert_eq!(gen.generate(0), 1);
    }
}
