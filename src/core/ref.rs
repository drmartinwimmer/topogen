use crate::traits::{Bounded, Center, Coordinate, Euclidean, Generator, Shape};

#[derive(Clone)]
pub struct Ref<'a, F> {
    referenced: &'a F,
}

impl<'a, F> Ref<'a, F> {
    pub fn new(referenced: &'a F) -> Self {
        Self { referenced }
    }
}

impl<'a, G: Generator<I>, I> Generator<I> for Ref<'a, G> {
    type Output = G::Output;

    fn generate(&self, input: I) -> G::Output {
        self.referenced.generate(input)
    }
}

impl<'a, E, T: Coordinate, const D: usize> Euclidean<T, D> for Ref<'a, E> where E: Euclidean<T, D> {}

impl<'a, B, T, const D: usize> Bounded<T, D> for Ref<'a, B>
where
    T: Coordinate,
    B: Bounded<T, D>,
{
    fn within(&self, input: [T; D]) -> bool {
        self.referenced.within(input)
    }

    fn bounding_box(&self) -> crate::shapes::BoundingBox<T, D> {
        self.referenced.bounding_box()
    }
}

impl<'a, C, T, const D: usize> Center<T, D> for Ref<'a, C>
where
    T: Coordinate,
    C: Center<T, D>,
{
    fn center(&self) -> &[T; D] {
        self.referenced.center()
    }
}

impl<'a, S, T, const D: usize> Shape<T, D> for Ref<'a, S>
where
    T: Coordinate,
    S: Shape<T, D>,
{
    fn distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float,
    {
        self.referenced.distance(point)
    }
}
