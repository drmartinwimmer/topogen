use crate::traits::{Center, Coordinate, Euclidean, Generator, Shape};
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Euclidean, Center)]
pub struct DistanceMap<S, T, const D: usize> {
    i: PhantomData<[T; D]>,
    #[wrap]
    pub shape: S,
}

impl<S, T, const D: usize> DistanceMap<S, T, D> {
    pub(crate) fn new(shape: S) -> Self {
        Self {
            i: PhantomData,
            shape,
        }
    }
}

impl<S, T, const D: usize> Generator<[T; D]> for DistanceMap<S, T, D>
where
    T: Coordinate + num_traits::Float,
    S: Shape<T, D>,
{
    type Output = T;

    fn generate(&self, input: [T; D]) -> Self::Output {
        self.shape.distance(input)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Euclidean, Center)]
pub struct SignedDistanceMap<S, T, const D: usize> {
    i: PhantomData<[T; D]>,
    #[wrap]
    pub shape: S,
}

impl<S, T, const D: usize> SignedDistanceMap<S, T, D> {
    pub(crate) fn new(shape: S) -> Self {
        Self {
            i: PhantomData,
            shape,
        }
    }
}

impl<S, T, const D: usize> Generator<[T; D]> for SignedDistanceMap<S, T, D>
where
    T: Coordinate + num_traits::Float,
    S: Shape<T, D>,
{
    type Output = T;

    fn generate(&self, input: [T; D]) -> Self::Output {
        self.shape.signed_distance(input)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CenterDistanceMap<C, T, const D: usize> {
    i: PhantomData<[T; D]>,
    pub center: C,
}

impl<C, T, const D: usize> CenterDistanceMap<C, T, D> {
    pub(crate) fn new(center: C) -> Self {
        Self {
            i: PhantomData,
            center,
        }
    }
}

impl<C, T, const D: usize> Generator<[T; D]> for CenterDistanceMap<C, T, D>
where
    T: Coordinate + num_traits::Float,
    C: Center<T, D>,
{
    type Output = T;

    fn generate(&self, input: [T; D]) -> Self::Output {
        self.center.center_distance(input)
    }
}

impl<E, T: Coordinate, const D: usize> Euclidean<T, D> for CenterDistanceMap<E, T, D> where
    E: Euclidean<T, D>
{
}

impl<C, T, const D: usize> Center<T, D> for CenterDistanceMap<C, T, D>
where
    T: Coordinate,
    C: Center<T, D>,
{
    fn center(&self) -> [T; D] {
        self.center.center()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shapes::{Point, Sphere},
        traits::{Center, Generator, Shape},
    };
    use googletest::prelude::*;

    #[googletest::test]
    fn distance_map_evaluates_shape_distance() {
        let map = Sphere {
            radius: 2.0_f64,
            center: [1.0, 2.0],
        }
        .distance_map();
        expect_that!(map.generate([1.0, 2.0]), eq(0.0));
        expect_that!(map.generate([4.0, 6.0]), eq(3.0));
    }

    #[googletest::test]
    fn signed_distance_map_evaluates_interior_boundary_and_exterior() {
        let map = Sphere {
            radius: 2.0_f64,
            center: [1.0, 2.0],
        }
        .signed_distance_map();
        expect_that!(map.generate([1.0, 2.0]), eq(-2.0));
        expect_that!(map.generate([3.0, 2.0]), eq(0.0));
        expect_that!(map.generate([4.0, 6.0]), eq(3.0));
    }

    #[googletest::test]
    fn center_distance_map_evaluates_euclidean_distance() {
        let map = Point::new([1.0_f64, 2.0]).center_distance_map();
        expect_that!(map.center(), eq([1.0, 2.0]));
        expect_that!(map.generate([4.0, 6.0]), eq(5.0));
    }
}
