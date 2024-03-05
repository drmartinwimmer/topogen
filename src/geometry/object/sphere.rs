use crate::geometry::object::Cuboid;
use crate::geometry::GeometricObject;

#[derive(Debug, Clone)]
pub struct Sphere<T, const D: usize> {
    pub radius: T,
    pub center: [T; D],
}

macro_rules! impl_sphere {
    ($t:ty, $sqrtt:ty) => {
        impl<const D: usize> GeometricObject<$t, D> for Sphere<$t, D> {
            fn center(&self) -> [$t; D] {
                self.center
            }

            fn bounding_box(&self) -> Cuboid<$t, D> {
                Cuboid {
                    center: self.center,
                    sides: [(2 as $t) * self.radius; D],
                }
            }

            fn within(&self, point: [$t; D]) -> bool {
                let radius: $t = point
                    .into_iter()
                    .zip(self.center)
                    .map(|(p, c)| (p - c) * (p - c))
                    .sum();
                radius <= self.radius * self.radius
            }

            fn center_distance(&self, point: [$t; D]) -> $t {
                let distance: $t = point
                    .into_iter()
                    .zip(self.center)
                    .map(|(p, c)| (p - c) * (p - c))
                    .sum();
                (distance as $sqrtt).sqrt() as $t
            }

            fn distance(&self, point: [$t; D]) -> $t {
                <$t>::max(self.center_distance(point) - self.radius, 0 as $t)
            }
        }

        impl<const D: usize> Default for Sphere<$t, D> {
            fn default() -> Self {
                Self {
                    radius: 1 as $t,
                    center: [<$t>::default(); D],
                }
            }
        }
    };
}

macro_rules! impl_sphere_int {
    ($($t:ty),+) => {$(
        impl_sphere!($t, f64);
    )+};
}

macro_rules! impl_sphere_float {
    ($($t:ty),+) => {$(
        impl_sphere!($t, $t);
    )+};
}

impl_sphere_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_sphere_float!(f32, f64);

#[cfg(test)]
mod tests {
    use super::Sphere;
    use crate::geometry::GeometricObject;
    use crate::Generator;

    #[test]
    fn generates_sphere() {
        let sphere: Sphere<f64, 2> = Sphere::default();
        let sphere = sphere.bitmap();
        assert!(sphere.generate([0.9, 0.0]));
        assert!(sphere.generate([0.7, 0.3]));
        assert!(!sphere.generate([1.0, 0.3]));
        assert!(!sphere.generate([0.9, 0.9]));
    }

    #[test]
    fn sphere_works_for_negatives() {
        let sphere: Sphere<f64, 3> = Sphere::default();
        assert!(sphere.within([-0.7, -0.3, 0.0]));
        assert!(!sphere.within([-1.0, 0.3, 0.0]));
    }

    #[test]
    fn sphere_works_across_dimensions() {
        let sphere: Sphere<f64, 1> = Sphere::default();
        assert!(sphere.within([0.9]));
        let sphere: Sphere<f64, 4> = Sphere::default();
        assert!(sphere.within([0.4, 0.3, 0.2, 0.1]));
    }

    #[test]
    fn sphere_works_across_types() {
        let sphere: Sphere<i8, 1> = Sphere::default();
        assert!(sphere.within([1]));
        assert!(!sphere.within([2]));
        let sphere: Sphere<i64, 1> = Sphere::default();
        assert!(sphere.within([1]));
    }

    #[test]
    fn sphere_scales() {
        let mut sphere: Sphere<f64, 3> = Sphere {
            radius: 0.9,
            ..Default::default()
        };
        assert!(!sphere.within([0.91, 0.0, 0.0]));
        sphere.radius = 3.0;
        assert!(sphere.within([0.9, 0.0, 0.0]));
        assert!(sphere.within([1.5, 1.5, 1.5]));
        assert!(!sphere.within([2.0, 2.0, 2.0]));
    }

    #[test]
    fn sphere_moves() {
        let sphere: Sphere<f64, 2> = Sphere {
            center: [2.0, 2.0],
            ..Default::default()
        };
        assert!(!sphere.within([0.0, 0.0]));
        assert!(sphere.within([2.0, 2.0]));
        assert!(sphere.within([1.0, 2.0]));
        assert!(sphere.within([2.0, 3.0]));
        assert!(!sphere.within([3.0, 3.0]));
    }

    #[test]
    fn returns_sphere_center() {
        let center = [2.0, 2.0];
        let sphere: Sphere<f64, 2> = Sphere {
            center,
            ..Default::default()
        };
        assert_eq!(center, sphere.center());
    }

    #[test]
    fn returns_center_distance() {
        let sphere: Sphere<f64, 2> = Sphere::default();
        assert_eq!(0.0, sphere.center_distance([0.0, 0.0]));
        assert_eq!(2.0, sphere.center_distance([2.0, 0.0]));
        assert_eq!(2.0, sphere.center_distance([0.0, 2.0]));
        assert_eq!(5.0, sphere.center_distance([3.0, 4.0]));
    }

    #[test]
    fn returns_center_distance_for_int() {
        let sphere: Sphere<i8, 2> = Sphere::default();
        assert_eq!(0, sphere.center_distance([0, 0]));
        assert_eq!(2, sphere.center_distance([2, 0]));
        assert_eq!(2, sphere.center_distance([0, 2]));
        assert_eq!(5, sphere.center_distance([3, 4]));
    }

    #[test]
    fn returns_distance() {
        let sphere: Sphere<f64, 2> = Sphere::default();
        assert_eq!(0.0, sphere.distance([0.0, 0.0]));
        assert_eq!(1.0, sphere.distance([2.0, 0.0]));
        assert_eq!(1.0, sphere.distance([0.0, 2.0]));
        assert_eq!(4.0, sphere.distance([3.0, 4.0]));
    }

    #[test]
    fn returns_distance_for_other_radius() {
        let sphere: Sphere<f64, 2> = Sphere {
            radius: 2.0,
            ..Default::default()
        };
        assert_eq!(0.0, sphere.distance([0.0, 0.0]));
        assert_eq!(0.0, sphere.distance([2.0, 0.0]));
        assert_eq!(1.0, sphere.distance([0.0, 3.0]));
        assert_eq!(3.0, sphere.distance([3.0, 4.0]));
    }

    #[test]
    fn returns_center_distance_for_other_center() {
        let sphere: Sphere<f64, 2> = Sphere {
            center: [3.0, 4.0],
            ..Default::default()
        };
        assert_eq!(5.0, sphere.center_distance([0.0, 0.0]));
        assert_eq!(0.0, sphere.center_distance([3.0, 4.0]));
        assert_eq!(1.0, sphere.center_distance([4.0, 4.0]));
    }
}
