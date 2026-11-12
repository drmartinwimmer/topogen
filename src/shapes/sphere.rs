use crate::shapes::BoundingBox;
use crate::traits::{Bounded, Center, Coordinate, Euclidean, Shape};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sphere<T, const D: usize> {
    pub radius: T,
    pub center: [T; D],
}

impl<T: Coordinate, const D: usize> Center<T, D> for Sphere<T, D> {
    fn center(&self) -> [T; D] {
        self.center
    }
}

impl<T: Coordinate, const D: usize> Euclidean<T, D> for Sphere<T, D> {}

impl<T, const D: usize> Bounded<T, D> for Sphere<T, D>
where
    T: Coordinate,
{
    fn within(&self, point: [T; D]) -> bool {
        let radius: T = point
            .into_iter()
            .zip(self.center)
            .map(|(p, c)| (p - c) * (p - c))
            .sum();
        radius < self.radius * self.radius
    }

    fn bounding_box(&self) -> BoundingBox<T, D> {
        BoundingBox::new_centered(self.center, [self.radius; D])
    }
}

impl<T, const D: usize> Shape<T, D> for Sphere<T, D>
where
    T: Coordinate,
{
    fn distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float,
    {
        T::max(self.signed_distance(point), T::zero())
    }

    fn signed_distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float,
    {
        self.center_distance(point) - self.radius
    }
}

impl<T, const D: usize> Default for Sphere<T, D>
where
    T: Coordinate + num_traits::One + Default,
{
    fn default() -> Self {
        Self {
            radius: T::one(),
            center: [T::default(); D],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Sphere;
    use crate::traits::{Bounded, Center, Generator, Shape};
    use googletest::prelude::*;

    #[googletest::test]
    fn generates_sphere_bitmap() {
        let sphere: Sphere<f64, 2> = Sphere::default();
        let sphere = sphere.bitmap();
        expect_that!(sphere.generate([0.9, 0.0]), is_true());
        expect_that!(sphere.generate([0.7, 0.3]), is_true());
        expect_that!(sphere.generate([1.0, 0.3]), is_false());
        expect_that!(sphere.generate([0.9, 0.9]), is_false());
    }

    #[googletest::test]
    fn sphere_works_for_negatives() {
        let sphere: Sphere<f64, 3> = Sphere::default();
        expect_that!(sphere.within([-0.7, -0.3, 0.0]), is_true());
        expect_that!(sphere.within([-1.0, 0.3, 0.0]), is_false());
    }

    #[googletest::test]
    fn sphere_works_across_dimensions() {
        let sphere: Sphere<f64, 1> = Sphere::default();
        expect_that!(sphere.within([0.9]), is_true());
        let sphere: Sphere<f64, 4> = Sphere::default();
        expect_that!(sphere.within([0.4, 0.3, 0.2, 0.1]), is_true());
    }

    #[googletest::test]
    fn sphere_works_across_types() {
        let sphere: Sphere<i8, 1> = Sphere {
            radius: 2,
            ..Default::default()
        };
        expect_that!(sphere.within([1]), is_true());
        expect_that!(sphere.within([2]), is_false());
        let sphere: Sphere<i64, 1> = Sphere::default();
        expect_that!(sphere.within([0]), is_true());
    }

    #[googletest::test]
    fn scales_sphere_radius() {
        let mut sphere: Sphere<f64, 3> = Sphere {
            radius: 0.9,
            ..Default::default()
        };
        expect_that!(sphere.within([0.91, 0.0, 0.0]), is_false());
        sphere.radius = 3.0;
        expect_that!(sphere.within([0.9, 0.0, 0.0]), is_true());
        expect_that!(sphere.within([1.5, 1.5, 1.5]), is_true());
        expect_that!(sphere.within([2.0, 2.0, 2.0]), is_false());
    }

    #[googletest::test]
    fn moves_sphere_bounds() {
        let sphere: Sphere<f64, 2> = Sphere {
            center: [2.0, 2.0],
            ..Default::default()
        };
        expect_that!(sphere.within([0.0, 0.0]), is_false());
        expect_that!(sphere.within([2.0, 2.0]), is_true());
        expect_that!(sphere.within([1.01, 2.0]), is_true());
        expect_that!(sphere.within([2.0, 2.99]), is_true());
        expect_that!(sphere.within([3.0, 3.0]), is_false());
    }

    #[googletest::test]
    fn returns_sphere_center() {
        let center = [2.0, 2.0];
        let sphere: Sphere<f64, 2> = Sphere {
            center,
            ..Default::default()
        };
        expect_that!(sphere.center(), eq(center));
    }

    #[googletest::test]
    fn returns_center_distance() {
        let sphere: Sphere<f64, 2> = Sphere::default();
        expect_that!(sphere.center_distance([0.0, 0.0]), eq(0.0));
        expect_that!(sphere.center_distance([2.0, 0.0]), eq(2.0));
        expect_that!(sphere.center_distance([0.0, 2.0]), eq(2.0));
        expect_that!(sphere.center_distance([3.0, 4.0]), eq(5.0));
    }

    #[googletest::test]
    fn returns_sphere_distance() {
        let sphere: Sphere<f64, 2> = Sphere::default();
        expect_that!(sphere.distance([0.0, 0.0]), eq(0.0));
        expect_that!(sphere.distance([2.0, 0.0]), eq(1.0));
        expect_that!(sphere.distance([0.0, 2.0]), eq(1.0));
        expect_that!(sphere.distance([3.0, 4.0]), eq(4.0));
    }

    #[googletest::test]
    fn returns_distance_for_other_radius() {
        let sphere: Sphere<f64, 2> = Sphere {
            radius: 2.0,
            ..Default::default()
        };
        expect_that!(sphere.distance([0.0, 0.0]), eq(0.0));
        expect_that!(sphere.distance([2.0, 0.0]), eq(0.0));
        expect_that!(sphere.distance([0.0, 3.0]), eq(1.0));
        expect_that!(sphere.distance([3.0, 4.0]), eq(3.0));
    }

    #[googletest::test]
    fn returns_center_distance_for_other_center() {
        let sphere: Sphere<f64, 2> = Sphere {
            center: [3.0, 4.0],
            ..Default::default()
        };
        expect_that!(sphere.center_distance([0.0, 0.0]), eq(5.0));
        expect_that!(sphere.center_distance([3.0, 4.0]), eq(0.0));
        expect_that!(sphere.center_distance([4.0, 4.0]), eq(1.0));
    }

    #[googletest::test]
    fn returns_sphere_signed_distance() {
        let sphere = Sphere {
            radius: 2.0_f64,
            center: [0.0, 0.0],
        };
        expect_that!(sphere.signed_distance([0.0, 0.0]), eq(-2.0));
        expect_that!(sphere.signed_distance([2.0, 0.0]), eq(0.0));
        expect_that!(sphere.signed_distance([3.0, 4.0]), eq(3.0));
    }
}
