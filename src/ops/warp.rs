use crate::{
    shapes::BoundingBox,
    traits::{Bounded, Coordinate, Euclidean, EuclideanVec, Generator},
};

/// Domain-warping operator displacing input coordinates by a vector-valued [`Generator`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Warp<E, W> {
    wrapped: E,
    warp: W,
}

impl<E, W> Warp<E, W> {
    /// Evaluates `wrapped` at `point + warp.generate(point)`.
    pub fn new(wrapped: E, warp: W) -> Self {
        Self { wrapped, warp }
    }
}

impl<E, W, T, const D: usize> Euclidean<T, D> for Warp<E, W>
where
    T: Coordinate,
    E: Euclidean<T, D>,
    W: Generator<[T; D], Output = [T; D]>,
{
}

impl<E, W, T, const D: usize> Bounded<T, D> for Warp<E, W>
where
    T: Coordinate,
    E: Bounded<T, D>,
    W: Generator<[T; D], Output = [T; D]>,
{
    fn within(&self, input: [T; D]) -> bool {
        self.wrapped.within(input.add(self.warp.generate(input)))
    }

    fn bounding_box(&self) -> BoundingBox<T, D> {
        match self.wrapped.bounding_box() {
            BoundingBox::Empty => BoundingBox::Empty,
            BoundingBox::Unbounded | BoundingBox::Bounded(_) => BoundingBox::Unbounded,
        }
    }
}

impl<E, W, T, const D: usize> Generator<[T; D]> for Warp<E, W>
where
    T: Coordinate,
    E: Generator<[T; D]> + Euclidean<T, D>,
    W: Generator<[T; D], Output = [T; D]>,
{
    type Output = E::Output;

    fn generate(&self, input: [T; D]) -> Self::Output {
        self.wrapped.generate(input.add(self.warp.generate(input)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::Constant,
        shapes::Sphere,
        traits::{Bounded, Euclidean, Generator},
    };
    use googletest::prelude::*;

    #[googletest::test]
    fn warp_displaces_evaluation_coordinates() {
        let sphere = Sphere {
            center: [5.0_f64, -2.0],
            radius: 1.5,
        };
        let warped = sphere.warp(Constant::new([5.0_f64, -2.0])).bitmap();
        expect_that!(warped.generate([0.0, 0.0]), is_true());
        expect_that!(warped.generate([5.0, -2.0]), is_false());
    }
}
