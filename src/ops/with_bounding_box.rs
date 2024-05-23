use crate::{
    shapes::BoundingBox,
    traits::{Bounded, Center, Coordinate, Euclidean, Generator, Shape},
};

#[derive(Euclidean)]
pub struct WithBoundingBox<B, T, const D: usize>
where
    B: Bounded<T, D>,
    T: Coordinate,
{
    #[wrap]
    bounded: B,
    bounding_box: BoundingBox<T, D>,
}

impl<B, T, const D: usize> WithBoundingBox<B, T, D>
where
    B: Bounded<T, D>,
    T: Coordinate,
{
    pub fn new(bounded: B, bounding_box: BoundingBox<T, D>) -> Self {
        Self {
            bounded,
            bounding_box,
        }
    }
}

impl<B, T, const D: usize> Bounded<T, D> for WithBoundingBox<B, T, D>
where
    B: Bounded<T, D>,
    T: Coordinate,
{
    fn bounding_box(&self) -> BoundingBox<T, D> {
        self.bounding_box.clone()
    }

    fn within(&self, point: [T; D]) -> bool {
        self.bounding_box.within(point) && self.bounded.within(point)
    }
}

impl<C, T, const D: usize> Center<T, D> for WithBoundingBox<C, T, D>
where
    C: Center<T, D> + Bounded<T, D>,
    T: Coordinate,
{
    fn center(&self) -> [T; D] {
        self.bounded.center()
    }
}

impl<S, T, const D: usize> Shape<T, D> for WithBoundingBox<S, T, D>
where
    S: Shape<T, D>,
    T: Coordinate,
{
    fn distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float,
    {
        self.bounded.distance(point)
    }
}

impl<G, T, const D: usize> Generator<[T; D]> for WithBoundingBox<G, T, D>
where
    G: Generator<[T; D]> + Bounded<T, D>,
    G::Output: Default,
    T: Coordinate,
{
    type Output = G::Output;

    fn generate(&self, input: [T; D]) -> Self::Output {
        match self.bounding_box.within(input) {
            true => self.bounded.generate(input),
            false => Default::default(),
        }
    }
}
