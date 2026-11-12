use crate::{
    shapes::BoundingBox,
    traits::{Bounded, Center, Coordinate, Euclidean, Generator, Shape},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Euclidean, Center)]
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
        self.bounding_box
    }

    fn within(&self, point: [T; D]) -> bool {
        self.bounding_box.within(point) && self.bounded.within(point)
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

    fn signed_distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float,
    {
        self.bounded.signed_distance(point)
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

#[cfg(test)]
mod tests {
    use crate::{
        shapes::{BoundingBox, Cuboid},
        traits::{Bounded, Generator},
    };
    use googletest::prelude::*;

    #[googletest::test]
    fn with_bounding_box_restricts_generator_domain() {
        let cube = Cuboid {
            center: [0, 0],
            extents: [5, 5],
        }
        .bitmap()
        .with_bounding_box(BoundingBox::new_centered([0, 0], [2, 2]));
        expect_that!(cube.within([1, 1]), is_true());
        expect_that!(cube.within([3, 0]), is_false());
        expect_that!(cube.generate([1, -1]), is_true());
        expect_that!(cube.generate([3, 0]), is_false());
    }
}
