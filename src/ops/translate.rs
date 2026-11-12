use crate::shapes::BoundingBox;
use crate::traits::{Bounded, Center, Coordinate, Euclidean, EuclideanVec, Generator, Shape};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Euclidean)]
pub struct Translate<E, T, const D: usize> {
    #[wrap]
    wrapped: E,
    offset: [T; D],
}

impl<E, T, const D: usize> Translate<E, T, D>
where
    E: Euclidean<T, D>,
    T: Coordinate,
{
    pub fn new(wrapped: E, offset: [T; D]) -> Self {
        Self { wrapped, offset }
    }
}

impl<B, T, const D: usize> Bounded<T, D> for Translate<B, T, D>
where
    B: Bounded<T, D>,
    T: Coordinate,
{
    fn within(&self, input: [T; D]) -> bool {
        self.wrapped.within(input.sub(self.offset))
    }

    fn bounding_box(&self) -> BoundingBox<T, D> {
        self.wrapped.bounding_box().translate(self.offset)
    }
}

impl<C, T, const D: usize> Center<T, D> for Translate<C, T, D>
where
    C: Center<T, D>,
    T: Coordinate,
{
    fn center(&self) -> [T; D] {
        self.wrapped.center().add(self.offset)
    }
}

impl<S, T, const D: usize> Shape<T, D> for Translate<S, T, D>
where
    S: Shape<T, D>,
    T: Coordinate,
{
    fn distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float,
    {
        self.wrapped.distance(point.sub(self.offset))
    }

    fn signed_distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float,
    {
        self.wrapped.signed_distance(point.sub(self.offset))
    }
}

impl<G, T, const D: usize> Generator<[T; D]> for Translate<G, T, D>
where
    G: Generator<[T; D]> + Euclidean<T, D>,
    T: Coordinate,
{
    type Output = G::Output;

    fn generate(&self, input: [T; D]) -> Self::Output {
        self.wrapped.generate(input.sub(self.offset))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shapes::{Cuboid, Sphere},
        traits::{Bounded, Euclidean, Generator, Shape},
    };
    use googletest::prelude::*;

    #[googletest::test]
    fn translates_cuboid_generator() {
        let cube_map = Cuboid {
            center: [0, 0],
            extents: [2, 2],
        }
        .bitmap();
        let translated = cube_map.translate([3, 3]);
        expect_that!(translated.generate([2, 2]), is_true());
        expect_that!(translated.generate([4, 4]), is_true());
        expect_that!(translated.generate([1, 1]), is_false());
    }

    #[googletest::test]
    fn translates_shape_distances() {
        let sphere = Sphere {
            center: [0.0_f64, 0.0],
            radius: 2.0,
        }
        .translate([5.0, -3.0]);
        expect_that!(sphere.distance([5.0, -3.0]), eq(0.0));
        expect_that!(sphere.signed_distance([5.0, -3.0]), eq(-2.0));
        expect_that!(sphere.signed_distance([8.0, 1.0]), eq(3.0));
        expect_that!(sphere.distance([8.0, 1.0]), eq(3.0));
    }
}
