use crate::{
    shapes::{BoundingBox, DimensionBound},
    traits::{Bounded, Coordinate, Euclidean, Generator},
};
use std::cmp::Ordering;

pub trait PeriodicCoordinate: Coordinate {
    fn wrap_periodic(self, center: Self, period: Self) -> Self;
}

macro_rules! impl_signed_periodic {
    ($($t:ty),+) => {$(
        impl PeriodicCoordinate for $t {
            fn wrap_periodic(self, center: Self, period: Self) -> Self {
                if period == 0 {
                    return self;
                }
                let period = period.abs();
                let d = (self - center).rem_euclid(period);
                let half = period / 2 + (period % 2);
                if d < half {
                    center + d
                } else {
                    center - (period - d)
                }
            }
        }
    )+};
}

macro_rules! impl_unsigned_periodic {
    ($($t:ty),+) => {$(
        impl PeriodicCoordinate for $t {
            fn wrap_periodic(self, center: Self, period: Self) -> Self {
                if period == 0 {
                    return self;
                }
                let d = (self % period + period - center % period) % period;
                let half = period.div_ceil(2);
                if d < half {
                    center + d
                } else {
                    let neg_offset = period - d;
                    if center >= neg_offset {
                        center - neg_offset
                    } else {
                        center + d
                    }
                }
            }
        }
    )+};
}

macro_rules! impl_float_periodic {
    ($($t:ty),+) => {$(
        impl PeriodicCoordinate for $t {
            fn wrap_periodic(self, center: Self, period: Self) -> Self {
                let period = period.abs();
                if period == 0.0 {
                    return self;
                }
                let half = period / 2.0;
                (self - center + half).rem_euclid(period) - half + center
            }
        }
    )+};
}

impl_signed_periodic!(i8, i16, i32, i64, i128, isize);
impl_unsigned_periodic!(u8, u16, u32, u64, u128, usize);
impl_float_periodic!(f32, f64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Repeat<E, T, const D: usize>
where
    T: PeriodicCoordinate,
{
    wrapped: E,
    center: [T; D],
    period: [T; D],
}

impl<E, T, const D: usize> Repeat<E, T, D>
where
    T: PeriodicCoordinate,
{
    pub fn new(wrapped: E, center: [T; D], period: [T; D]) -> Self {
        Self {
            wrapped,
            center,
            period,
        }
    }

    fn wrap_point(&self, mut point: [T; D]) -> [T; D] {
        for ((p, c), r) in point.iter_mut().zip(self.center).zip(self.period) {
            *p = p.wrap_periodic(c, r);
        }
        point
    }
}

impl<E, T, const D: usize> Euclidean<T, D> for Repeat<E, T, D>
where
    T: PeriodicCoordinate,
    E: Euclidean<T, D>,
{
}

impl<E, T, const D: usize> Bounded<T, D> for Repeat<E, T, D>
where
    T: PeriodicCoordinate,
    E: Bounded<T, D>,
{
    fn within(&self, point: [T; D]) -> bool {
        self.wrapped.within(self.wrap_point(point))
    }

    fn bounding_box(&self) -> BoundingBox<T, D> {
        let zero: T = std::iter::empty().sum();
        match self.wrapped.bounding_box() {
            BoundingBox::Empty => BoundingBox::Empty,
            BoundingBox::Unbounded => BoundingBox::Unbounded,
            BoundingBox::Bounded(mut bounds) => {
                for (bound, period) in bounds.iter_mut().zip(self.period) {
                    if !matches!(period.partial_cmp(&zero), Some(Ordering::Equal) | None) {
                        *bound = DimensionBound::Unbounded;
                    }
                }
                if bounds
                    .iter()
                    .all(|b| matches!(b, DimensionBound::Unbounded))
                {
                    BoundingBox::Unbounded
                } else {
                    BoundingBox::Bounded(bounds)
                }
            }
        }
    }
}

impl<E, T, const D: usize> Generator<[T; D]> for Repeat<E, T, D>
where
    T: PeriodicCoordinate,
    E: Generator<[T; D]>,
{
    type Output = E::Output;

    fn generate(&self, point: [T; D]) -> Self::Output {
        self.wrapped.generate(self.wrap_point(point))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shapes::{BoundingBox, Cuboid, DimensionBound, Sphere},
        traits::{Bounded, Euclidean, Generator},
    };
    use googletest::prelude::*;

    #[googletest::test]
    fn repeats_sphere_periodically() {
        let sphere: Sphere<i32, 2> = Sphere {
            radius: 10,
            ..Default::default()
        };
        let repeat = sphere.bitmap().repeat([0; 2], [100, 50]);
        expect_that!(repeat.generate([9, 0]), is_true());
        expect_that!(repeat.generate([-9, 0]), is_true());
        expect_that!(repeat.generate([10, 0]), is_false());
        expect_that!(repeat.generate([91, 0]), is_true());
        expect_that!(repeat.generate([209, 300]), is_true());
        expect_that!(repeat.within([91, 0]), is_true());
        expect_that!(repeat.within([50, 0]), is_false());
        expect_that!(
            repeat.bounding_box(),
            matches_pattern!(BoundingBox::Unbounded)
        );
    }

    #[googletest::test]
    fn repeats_quarter_sphere_without_center() {
        let sphere: Sphere<i32, 2> = Sphere {
            radius: 10,
            ..Default::default()
        };
        let repeat = sphere.bitmap().repeat([50; 2], [100, 100]);
        expect_that!(repeat.generate([0, 0]), is_true());
        expect_that!(repeat.generate([9, 0]), is_true());
        expect_that!(repeat.generate([0, 9]), is_true());
        expect_that!(repeat.generate([-1, 0]), is_false());
    }

    #[googletest::test]
    fn repeats_float_and_unsigned_coordinates_and_preserves_zero_period_axis() {
        let sphere_f64 = Sphere {
            center: [0.0_f64, 0.0],
            radius: 2.5,
        };
        let repeated_f64 = sphere_f64.bitmap().repeat([0.0, 0.0], [10.0, 0.0]);
        expect_that!(repeated_f64.generate([9.0, 0.0]), is_true());
        expect_that!(repeated_f64.generate([9.0, 10.0]), is_false());
        expect_that!(
            repeated_f64.bounding_box(),
            matches_pattern!(BoundingBox::Bounded(elements_are![
                matches_pattern!(DimensionBound::Unbounded),
                matches_pattern!(DimensionBound::Range(eq(&-2.5), eq(&2.5))),
            ]))
        );

        let box_u32 = Cuboid {
            center: [50_u32, 50],
            extents: [10, 10],
        };
        let repeated_u32 = box_u32.bitmap().repeat([50, 50], [100, 100]);
        expect_that!(repeated_u32.generate([154, 246]), is_true());
        expect_that!(repeated_u32.generate([180, 250]), is_false());
    }
}
