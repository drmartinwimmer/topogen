use crate::{
    shapes::BoundingBox,
    traits::{Bounded, Coordinate, Euclidean, Generator},
};

/// Evaluates two generators at the same input and combines their outputs with a closure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Zip<A, B, F> {
    a: A,
    b: B,
    combiner: F,
}

impl<A, B, F> Zip<A, B, F> {
    /// Combines generators `a` and `b` point-wise using `combiner`.
    pub fn new(a: A, b: B, combiner: F) -> Self {
        Self { a, b, combiner }
    }
}

impl<I, A, B, O, F> Generator<I> for Zip<A, B, F>
where
    I: Copy,
    A: Generator<I>,
    B: Generator<I>,
    F: Fn(A::Output, B::Output) -> O,
{
    type Output = O;

    fn generate(&self, input: I) -> Self::Output {
        (self.combiner)(self.a.generate(input), self.b.generate(input))
    }
}

impl<A, B, F, T, const D: usize> Euclidean<T, D> for Zip<A, B, F>
where
    T: Coordinate,
    A: Euclidean<T, D>,
    B: Euclidean<T, D>,
{
}

impl<A, B, F, T, const D: usize> Bounded<T, D> for Zip<A, B, F>
where
    T: Coordinate,
    A: Bounded<T, D>,
    B: Bounded<T, D>,
{
    fn within(&self, input: [T; D]) -> bool {
        self.a.within(input) || self.b.within(input)
    }

    fn bounding_box(&self) -> BoundingBox<T, D> {
        self.a
            .bounding_box()
            .for_union(&self.b.bounding_box())
            .unwrap_or(BoundingBox::Unbounded)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shapes::{Cuboid, Sphere},
        traits::{Bounded, Generator, Shape},
    };
    use googletest::prelude::*;

    #[googletest::test]
    fn zip_combines_heterogeneous_generators() {
        let mask = Cuboid {
            center: [0.0_f64, 0.0],
            extents: [2.0, 2.0],
        }
        .bitmap();
        let sdf = Sphere {
            center: [0.0_f64, 0.0],
            radius: 1.0,
        }
        .signed_distance_map();

        let masked_sdf = mask.zip(sdf, |inside, dist| if inside { dist } else { 99.0 });
        expect_that!(masked_sdf.generate([0.0, 0.0]), eq(-1.0));
        expect_that!(masked_sdf.generate([1.5, 0.0]), eq(0.5));
        expect_that!(masked_sdf.generate([3.0, 0.0]), eq(99.0));
    }

    #[googletest::test]
    fn zip_bounds_cover_union_of_inputs() {
        let left = Cuboid {
            center: [-2, 0],
            extents: [1, 1],
        }
        .bitmap();
        let right = Cuboid {
            center: [2, 0],
            extents: [1, 1],
        }
        .bitmap();
        let zipped = left.zip(right, |a, b| (a, b));
        expect_that!(zipped.within([-2, 0]), is_true());
        expect_that!(zipped.within([2, 0]), is_true());
        expect_that!(zipped.within([0, 0]), is_false());
    }
}
