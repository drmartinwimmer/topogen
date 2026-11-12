use crate::{
    shapes::{BoundingBox, DimensionBound},
    traits::{Bounded, Center, Coordinate, Euclidean, Generator, Shape},
};
use num_traits::Float;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rotate2D<E, T> {
    wrapped: E,
    sin: T,
    cos: T,
}

impl<E, T> Rotate2D<E, T>
where
    T: Coordinate + Float,
{
    pub fn new(wrapped: E, angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self { wrapped, sin, cos }
    }

    fn forward_rotate(&self, [x, y]: [T; 2]) -> [T; 2] {
        [x * self.cos - y * self.sin, x * self.sin + y * self.cos]
    }

    fn inverse_rotate(&self, [x, y]: [T; 2]) -> [T; 2] {
        [x * self.cos + y * self.sin, -x * self.sin + y * self.cos]
    }
}

impl<E, T> Euclidean<T, 2> for Rotate2D<E, T>
where
    T: Coordinate + Float,
    E: Euclidean<T, 2>,
{
}

impl<C, T> Center<T, 2> for Rotate2D<C, T>
where
    T: Coordinate + Float,
    C: Center<T, 2>,
{
    fn center(&self) -> [T; 2] {
        self.forward_rotate(self.wrapped.center())
    }
}

impl<B, T> Bounded<T, 2> for Rotate2D<B, T>
where
    T: Coordinate + Float,
    B: Bounded<T, 2>,
{
    fn within(&self, input: [T; 2]) -> bool {
        self.wrapped.within(self.inverse_rotate(input))
    }

    fn bounding_box(&self) -> BoundingBox<T, 2> {
        match self.wrapped.bounding_box() {
            BoundingBox::Empty => BoundingBox::Empty,
            BoundingBox::Bounded(
                [
                    DimensionBound::Range(min_x, max_x),
                    DimensionBound::Range(min_y, max_y),
                ],
            ) => {
                let corners = [
                    self.forward_rotate([min_x, min_y]),
                    self.forward_rotate([min_x, max_y]),
                    self.forward_rotate([max_x, min_y]),
                    self.forward_rotate([max_x, max_y]),
                ];
                let min_rx = corners.iter().map(|c| c[0]).fold(T::infinity(), T::min);
                let max_rx = corners.iter().map(|c| c[0]).fold(T::neg_infinity(), T::max);
                let min_ry = corners.iter().map(|c| c[1]).fold(T::infinity(), T::min);
                let max_ry = corners.iter().map(|c| c[1]).fold(T::neg_infinity(), T::max);
                BoundingBox::new_bounded([
                    DimensionBound::Range(min_rx, max_rx),
                    DimensionBound::Range(min_ry, max_ry),
                ])
                .unwrap_or(BoundingBox::Unbounded)
            }
            BoundingBox::Unbounded | BoundingBox::Bounded(_) => BoundingBox::Unbounded,
        }
    }
}

impl<S, T> Shape<T, 2> for Rotate2D<S, T>
where
    T: Coordinate + Float,
    S: Shape<T, 2>,
{
    fn distance(&self, point: [T; 2]) -> T
    where
        T: Float,
    {
        self.wrapped.distance(self.inverse_rotate(point))
    }

    fn signed_distance(&self, point: [T; 2]) -> T
    where
        T: Float,
    {
        self.wrapped.signed_distance(self.inverse_rotate(point))
    }
}

impl<G, T> Generator<[T; 2]> for Rotate2D<G, T>
where
    T: Coordinate + Float,
    G: Generator<[T; 2]> + Euclidean<T, 2>,
{
    type Output = G::Output;

    fn generate(&self, input: [T; 2]) -> Self::Output {
        self.wrapped.generate(self.inverse_rotate(input))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shapes::{BoundingBox, Cuboid, DimensionBound},
        traits::{Bounded, Center, Euclidean, Generator, Shape},
    };
    use googletest::prelude::*;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2};

    #[googletest::test]
    fn rotates_2d_cuboid_by_quarter_turn() {
        let bar = Cuboid {
            center: [2.0_f64, 0.0],
            extents: [3.0, 1.0],
        };
        let rotated = bar.rotate(FRAC_PI_2);
        let [cx, cy] = rotated.center();
        expect_that!(cx, near(0.0, 1e-12));
        expect_that!(cy, near(2.0, 1e-12));
        expect_that!(rotated.within([0.0, 4.5]), is_true());
        expect_that!(rotated.within([4.5, 0.0]), is_false());
        expect_that!(rotated.signed_distance([0.0, 2.0]), near(-1.0, 1e-12));
        expect_that!(rotated.distance([2.0, 2.0]), near(1.0, 1e-12));
        expect_that!(rotated.bitmap().generate([0.0, 4.5]), is_true());
    }

    #[googletest::test]
    fn rotates_2d_bounding_box_corners() -> Result<()> {
        let square = Cuboid {
            center: [0.0_f64, 0.0],
            extents: [1.0, 1.0],
        }
        .rotate(FRAC_PI_4);
        let BoundingBox::Bounded(
            [
                DimensionBound::Range(min_x, max_x),
                DimensionBound::Range(min_y, max_y),
            ],
        ) = square.bounding_box()
        else {
            return fail!("expected finite rotated bounding box");
        };
        expect_that!(min_x, near(-SQRT_2, 1e-12));
        expect_that!(max_x, near(SQRT_2, 1e-12));
        expect_that!(min_y, near(-SQRT_2, 1e-12));
        expect_that!(max_y, near(SQRT_2, 1e-12));
        Ok(())
    }
}
