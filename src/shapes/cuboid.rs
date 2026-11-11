use crate::shapes::BoundingBox;
use crate::traits::{Bounded, Center, Coordinate, Euclidean, Shape};

#[derive(Debug, Clone)]
pub struct Cuboid<T, const D: usize> {
    pub center: [T; D],
    pub extents: [T; D],
}

impl<T: Coordinate, const D: usize> Center<T, D> for Cuboid<T, D> {
    fn center(&self) -> [T; D] {
        self.center
    }
}

impl<T: Coordinate, const D: usize> Euclidean<T, D> for Cuboid<T, D> {}

impl<T, const D: usize> Bounded<T, D> for Cuboid<T, D>
where
    T: Coordinate,
{
    fn within(&self, point: [T; D]) -> bool {
        point
            .into_iter()
            .zip(self.extents.iter().zip(self.center.iter()))
            .all(|(p, (e, c))| p > *c - *e && p < *c + *e)
    }

    fn bounding_box(&self) -> BoundingBox<T, D> {
        BoundingBox::new_centered(self.center, self.extents)
    }
}

impl<T, const D: usize> Default for Cuboid<T, D>
where
    T: Coordinate + Default + num_traits::One,
{
    fn default() -> Self {
        Self {
            center: [T::default(); D],
            extents: [T::one(); D],
        }
    }
}

impl<T, const D: usize> Shape<T, D> for Cuboid<T, D>
where
    T: Coordinate,
{
    fn distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float,
    {
        let mut closest_point = point;
        closest_point
            .iter_mut()
            .zip(self.extents.iter().zip(self.center.iter()))
            .for_each(|(p, (e, c))| {
                let (left, right) = (*c - *e, *c + *e);
                *p = if *p < left {
                    left
                } else if *p > right {
                    right
                } else {
                    *p
                }
            });

        let distance: T = point
            .into_iter()
            .zip(closest_point.iter())
            .map(|(p, c)| (p - *c) * (p - *c))
            .sum();
        distance.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::Cuboid;
    use crate::traits::{Bounded, Center, Generator, Shape};
    use googletest::prelude::*;

    #[googletest::test]
    fn generates_cuboid_bitmap() {
        let cuboid: Cuboid<f64, 2> = Cuboid::default();
        let cuboid = cuboid.bitmap();
        expect_that!(cuboid.generate([0.99, 0.99]), is_true());
        expect_that!(cuboid.generate([0.99, 1.0]), is_false());
    }

    #[googletest::test]
    fn cuboid_works_for_negatives() {
        let cuboid: Cuboid<f64, 1> = Cuboid::default();
        expect_that!(cuboid.within([-0.99]), is_true());
        expect_that!(cuboid.within([-1.0]), is_false());
    }

    #[googletest::test]
    fn cuboid_works_across_types() {
        let cuboid: Cuboid<i32, 1> = Cuboid {
            extents: [5_i32],
            ..Default::default()
        };
        expect_that!(cuboid.within([4]), is_true());
        expect_that!(cuboid.within([5]), is_false());
    }

    #[googletest::test]
    fn unit_cuboid_works_across_dimensions() {
        let cuboid: Cuboid<i32, 8> = Cuboid {
            extents: [2_i32; 8],
            ..Default::default()
        };
        expect_that!(cuboid.within([1; 8]), is_true());
    }

    #[googletest::test]
    fn moves_cuboid_bounds() {
        let cuboid: Cuboid<i32, 2> = Cuboid {
            extents: [2, 2],
            center: [2, 2],
        };
        expect_that!(cuboid.within([0, 0]), is_false());
        expect_that!(cuboid.within([1, 1]), is_true());
        expect_that!(cuboid.within([2, 2]), is_true());
        expect_that!(cuboid.within([3, 3]), is_true());
        expect_that!(cuboid.within([1, 3]), is_true());
    }

    #[googletest::test]
    fn cuboid_works_with_distinct_sides() {
        let cuboid: Cuboid<i32, 3> = Cuboid {
            extents: [2, 3, 4],
            ..Default::default()
        };
        expect_that!(cuboid.within([1, 2, 3]), is_true());
        expect_that!(cuboid.within([-1, -2, -3]), is_true());
        expect_that!(cuboid.within([-1, 2, -3]), is_true());
        expect_that!(cuboid.within([1, 2, 4]), is_false());
        expect_that!(cuboid.within([1, 3, 3]), is_false());
        expect_that!(cuboid.within([-2, -2, -3]), is_false());
    }

    #[googletest::test]
    fn returns_cuboid_center() {
        let center = [2.0, 2.0];
        let cuboid: Cuboid<f64, 2> = Cuboid {
            center,
            ..Default::default()
        };
        expect_that!(cuboid.center(), eq(center));
    }

    #[googletest::test]
    fn returns_center_distance() {
        let cuboid: Cuboid<f64, 2> = Cuboid::default();
        expect_that!(cuboid.center_distance([0.0, 0.0]), eq(0.0));
        expect_that!(cuboid.center_distance([2.0, 0.0]), eq(2.0));
        expect_that!(cuboid.center_distance([0.0, 2.0]), eq(2.0));
        expect_that!(cuboid.center_distance([3.0, 4.0]), eq(5.0));
    }

    #[googletest::test]
    fn returns_cuboid_distance() {
        let cuboid: Cuboid<f64, 2> = Cuboid::default();
        expect_that!(cuboid.distance([0.0, 0.0]), eq(0.0));
        expect_that!(cuboid.distance([2.0, 0.0]), eq(1.0));
        expect_that!(cuboid.distance([2.0, -0.2]), eq(1.0));
        expect_that!(cuboid.distance([0.1, 2.0]), eq(1.0));
        expect_that!(cuboid.distance([4.0, 5.0]), eq(5.0));
    }

    #[googletest::test]
    fn returns_distance_for_other_sides() {
        let cuboid: Cuboid<f32, 3> = Cuboid {
            extents: [5.0, 10.0, 15.0],
            ..Default::default()
        };
        expect_that!(cuboid.distance([0.0, 0.0, 0.0]), eq(0.0));
        expect_that!(cuboid.distance([1.0, 10.0, -15.0]), eq(0.0));
        expect_that!(cuboid.distance([2.0, -10.0, 16.0]), eq(1.0));
    }

    #[googletest::test]
    fn returns_distance_for_other_center() {
        let cuboid: Cuboid<f64, 2> = Cuboid {
            center: [4.0, 5.0],
            ..Default::default()
        };
        expect_that!(cuboid.distance([0.0, 0.0]), eq(5.0));
        expect_that!(cuboid.distance([4.0, 5.0]), eq(0.0));
        expect_that!(cuboid.distance([5.0, 7.0]), eq(1.0));
    }
}
