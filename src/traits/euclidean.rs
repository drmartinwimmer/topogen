use super::{Bounded, Generator};
use crate::ops::{BoundTo, Fbm, PeriodicCoordinate, Repeat, Rotate2D, Scale, Translate, Warp};
use num_traits::Float;

pub use topogen_derive::Euclidean;

/// Scalar coordinate trait implemented automatically for numeric types supporting Euclidean arithmetic.
pub trait Coordinate:
    Copy
    + std::fmt::Display
    + std::fmt::Debug
    + PartialOrd
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Div<Output = Self>
    + std::ops::DivAssign
    + std::iter::Sum
{
}

impl<T> Coordinate for T where
    T: Copy
        + std::fmt::Display
        + std::fmt::Debug
        + PartialOrd
        + std::ops::Add<Output = T>
        + std::ops::AddAssign
        + std::ops::Sub<Output = T>
        + std::ops::SubAssign
        + std::ops::Mul<Output = T>
        + std::ops::MulAssign
        + std::ops::Div<Output = T>
        + std::ops::DivAssign
        + std::iter::Sum
{
}

/// Element-wise vector and dot-product operations on `D`-dimensional coordinate arrays.
pub trait EuclideanVec<T: Coordinate, const D: usize>: Sized {
    /// Returns `self + rhs` component-wise.
    fn add(self, rhs: Self) -> Self;
    /// Adds `rhs` into `self` component-wise.
    fn add_assign(&mut self, rhs: Self);
    /// Returns `self - rhs` component-wise.
    fn sub(self, rhs: Self) -> Self;
    /// Subtracts `rhs` from `self` component-wise.
    fn sub_assign(&mut self, rhs: Self);
    /// Multiplies every component of `self` by scalar `rhs`.
    fn scalar_mul(self, rhs: T) -> Self;
    /// Multiplies every component of `self` in place by scalar `rhs`.
    fn scalar_mul_assign(&mut self, rhs: T);
    /// Multiplies `self` and `rhs` component-wise.
    fn element_mul(self, rhs: Self) -> Self;
    /// Multiplies `self` in place by `rhs` component-wise.
    fn element_mul_assign(&mut self, rhs: Self);
    /// Divides every component of `self` by scalar `rhs`.
    fn scalar_div(self, rhs: T) -> Self;
    /// Divides every component of `self` in place by scalar `rhs`.
    fn scalar_div_assign(&mut self, rhs: T);
    /// Divides `self` by `rhs` component-wise.
    fn element_div(self, rhs: Self) -> Self;
    /// Divides `self` in place by `rhs` component-wise.
    fn element_div_assign(&mut self, rhs: Self);
    /// Computes the Euclidean dot product of `self` and `rhs`.
    fn dot(self, rhs: Self) -> T;
}

impl<T: Coordinate, const D: usize> EuclideanVec<T, D> for [T; D] {
    fn add(mut self, rhs: Self) -> Self {
        self.add_assign(rhs);
        self
    }

    fn add_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs).for_each(|(a, b)| *a += b);
    }

    fn sub(mut self, rhs: Self) -> Self {
        self.sub_assign(rhs);
        self
    }

    fn sub_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs).for_each(|(a, b)| *a -= b);
    }

    fn scalar_mul(mut self, rhs: T) -> Self {
        self.scalar_mul_assign(rhs);
        self
    }

    fn scalar_mul_assign(&mut self, rhs: T) {
        self.iter_mut().for_each(|i| *i *= rhs);
    }

    fn element_mul(mut self, rhs: Self) -> Self {
        self.element_mul_assign(rhs);
        self
    }

    fn element_mul_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs).for_each(|(a, b)| *a *= b);
    }

    fn scalar_div(mut self, rhs: T) -> Self {
        self.scalar_div_assign(rhs);
        self
    }

    fn scalar_div_assign(&mut self, rhs: T) {
        self.iter_mut().for_each(|i| *i /= rhs);
    }

    fn element_div(mut self, rhs: Self) -> Self {
        self.element_div_assign(rhs);
        self
    }

    fn element_div_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs).for_each(|(a, b)| *a /= b);
    }

    fn dot(self, rhs: Self) -> T {
        self.iter().zip(rhs).map(|(a, b)| *a * b).sum()
    }
}

/// Wraps floating-point lattice coordinates modulo 256 into `u8` table indices.
pub trait WrapToU8 {
    /// Reduces `self` modulo 256 using Euclidean remainder and converts to `u8`.
    fn wrap_to_u8(self) -> u8;
}

impl WrapToU8 for f32 {
    fn wrap_to_u8(self) -> u8 {
        // rem_euclid(256.0) bounds finite values into 0.0..256.0 prior to integer conversion.
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "rem_euclid(256.0) bounds the value to 0.0..256.0 before integer conversion"
        )]
        {
            self.rem_euclid(256.0) as u8
        }
    }
}

impl WrapToU8 for f64 {
    fn wrap_to_u8(self) -> u8 {
        // rem_euclid(256.0) bounds finite values into 0.0..256.0 prior to integer conversion.
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "rem_euclid(256.0) bounds the value to 0.0..256.0 before integer conversion"
        )]
        {
            self.rem_euclid(256.0) as u8
        }
    }
}

/// Floating-point vector helpers for lattice noise evaluation.
pub trait EuclideanVecFloat<T: Coordinate + Float, const D: usize>: EuclideanVec<T, D> {
    /// Applies `floor()` component-wise.
    fn floor(self) -> Self;
    /// Wraps each coordinate modulo 256 into `[u8; D]`.
    fn wrap_to_u8(self) -> [u8; D];
}

impl<T: Coordinate + Float + WrapToU8, const D: usize> EuclideanVecFloat<T, D> for [T; D] {
    fn floor(mut self) -> Self {
        self.iter_mut().for_each(|i| *i = i.floor());
        self
    }

    fn wrap_to_u8(self) -> [u8; D] {
        let mut out = [0u8; D];
        out.iter_mut()
            .zip(self)
            .for_each(|(o, i)| *o = i.wrap_to_u8());
        out
    }
}

/// Marker and combinator trait for objects embedded in `D`-dimensional Euclidean space.
pub trait Euclidean<T: Coordinate, const D: usize>: Sized {
    /// Bounds `self` to `bound_to`, returning `Default::default()` outside `bound_to`.
    fn bound_to<B: Bounded<T, D>>(self, bound_to: B) -> BoundTo<Self, B, T, D> {
        BoundTo::new(self, bound_to)
    }

    /// Layers `octaves` of fractional Brownian motion over `self`.
    fn fbm(
        self,
        octaves: usize,
        lacunarity: T,
        gain: <Self as Generator<[T; D]>>::Output,
    ) -> Fbm<Self, T, D>
    where
        Self: Generator<[T; D]>,
    {
        Fbm::new(self, octaves, lacunarity, gain)
    }

    /// Tiles `self` periodically around `center` with per-axis `period` (`0` disables repetition on an axis).
    fn repeat(self, center: [T; D], period: [T; D]) -> Repeat<Self, T, D>
    where
        T: PeriodicCoordinate,
    {
        Repeat::new(self, center, period)
    }

    /// Rotates a 2D spatial object counter-clockwise by `angle` radians around the origin.
    fn rotate(self, angle: T) -> Rotate2D<Self, T>
    where
        Self: Euclidean<T, 2>,
        T: Float,
    {
        Rotate2D::new(self, angle)
    }

    /// Scales `self` along each coordinate axis by `factor`.
    fn scale(self, factor: [T; D]) -> Scale<Self, T, D> {
        Scale::new(self, factor)
    }

    /// Translates `self` by `translation`.
    fn translate(self, translation: [T; D]) -> Translate<Self, T, D> {
        Translate::new(self, translation)
    }

    /// Applies domain warping by displacing input coordinates using `warp.generate(point)`.
    fn warp<W>(self, warp: W) -> Warp<Self, W>
    where
        W: Generator<[T; D], Output = [T; D]>,
    {
        Warp::new(self, warp)
    }
}

#[cfg(test)]
mod tests {
    use super::WrapToU8;
    use googletest::prelude::*;

    #[googletest::test]
    fn wrap_to_u8_works_for_negative() {
        expect_that!((-2.0).wrap_to_u8(), eq(254));
        expect_that!((-1.0).wrap_to_u8(), eq(255));
        expect_that!((-0.5).wrap_to_u8(), eq(255));
        expect_that!((0.0).wrap_to_u8(), eq(0));
    }
}
