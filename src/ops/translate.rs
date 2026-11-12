use crate::shapes::BoundingBox;
use crate::traits::{Bounded, Center, Coordinate, Euclidean, EuclideanVec, Generator};

#[derive(Debug, Euclidean)]
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
        shapes::Cuboid,
        traits::{Bounded, Euclidean, Generator},
    };
    use googletest::prelude::*;

    #[googletest::test]
    fn translates_cuboid_generator() {
        let cube_map = Cuboid {
            center: [0, 0],
            extents: [2, 2],
        }
        .bitmap();
        let scaled = cube_map.translate([3, 3]);
        expect_that!(scaled.generate([2, 2]), is_true());
        expect_that!(scaled.generate([4, 4]), is_true());
        expect_that!(scaled.generate([1, 1]), is_false());
    }
}
