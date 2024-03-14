use crate::geometry::{object::BoundingBox, GeometricObject, WithCenter};
use crate::ops::{Bitmap, Union, UnionError};
use crate::traits::{Bounded, Coordinate};

#[derive(Debug, Clone)]
pub struct Cuboid<T, const D: usize> {
    pub center: [T; D],
    pub half_edges: [T; D],
}

impl<T: Clone + Copy, const D: usize> WithCenter<T, D> for Cuboid<T, D> {
    fn center(&self) -> &[T; D] {
        &self.center
    }
}

impl<T, const D: usize> Bounded<T, D> for Cuboid<T, D>
where
    T: Coordinate,
{
    type Bitmap = Bitmap<T, Self, D>;

    fn bitmap(self) -> Self::Bitmap {
        Bitmap::new(self)
    }

    fn within(&self, point: [T; D]) -> bool {
        point
            .into_iter()
            .zip(self.half_edges.iter().zip(self.center.iter()))
            .all(|(p, (e, c))| p > *c - *e && p < *c + *e)
    }

    fn union<F>(
        self,
        mut other: Vec<Self>,
        reducer: F,
    ) -> Result<Union<T, Self, F, D>, UnionError<T>> {
        other.push(self);
        Union::new(other, reducer)
    }

    fn bounding_box(&self) -> BoundingBox<T, D> {
        BoundingBox::new_centered(self.center, self.half_edges)
    }
}

macro_rules! impl_cuboid_default {
    ($t:ty) => {
        impl<const D: usize> Default for Cuboid<$t, D> {
            fn default() -> Self {
                Self {
                    half_edges: [1 as $t; D],
                    center: [<$t>::default(); D],
                }
            }
        }
    };
}

macro_rules! impl_geometric_object {
    ($t:ty, $sqrtt:ty) => {
        impl<const D: usize> GeometricObject<$t, D> for Cuboid<$t, D> {
            fn distance(&self, point: [$t; D]) -> $t {
                let mut closest_point = point.clone();
                closest_point
                    .iter_mut()
                    .zip(self.half_edges.iter().zip(self.center.iter()))
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

                let distance: $t = point
                    .into_iter()
                    .zip(closest_point.iter())
                    .map(|(p, c)| (p - c) * (p - c))
                    .sum();
                (distance as $sqrtt).sqrt() as $t
            }
        }
    };
}

macro_rules! impl_cuboid_float {
    ($($t:ty),+) => {$(
        impl_cuboid_default!($t);
        impl_geometric_object!($t, $t);
    )+};
}

macro_rules! impl_cuboid_int {
    ($($t:ty),+) => {$(
        impl_cuboid_default!($t);
        impl_geometric_object!($t, f64);
    )+};
}

macro_rules! impl_cuboid_uint {
    ($($t:ty),+) => {$(
        impl_cuboid_default!($t);
        impl_geometric_object!($t, f64);
    )+};
}

impl_cuboid_float!(f32, f64);
impl_cuboid_int!(i8, i16, i32, i64, i128, isize);
impl_cuboid_uint!(u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use super::Cuboid;
    use crate::geometry::{GeometricObject, WithCenter, WithCenterDistance};
    use crate::traits::Bounded;
    use crate::Generator;

    #[test]
    fn generates_cuboid() {
        let cuboid: Cuboid<f64, 2> = Cuboid::default();
        let cuboid = cuboid.bitmap();
        assert!(cuboid.generate([0.99, 0.99]));
        assert!(!cuboid.generate([0.99, 1.0]));
    }

    #[test]
    fn cuboid_works_for_negatives() {
        let cuboid: Cuboid<f64, 1> = Cuboid::default();
        assert!(cuboid.within([-0.99]));
        assert!(!cuboid.within([-1.0]));
    }

    #[test]
    fn cuboid_works_across_types() {
        let cuboid: Cuboid<i32, 1> = Cuboid {
            half_edges: [5_i32],
            ..Default::default()
        };
        assert!(cuboid.within([4]));
        assert!(!cuboid.within([5]));
    }

    #[test]
    fn unit_cuboid_works_across_dimensions() {
        let cuboid: Cuboid<i32, 8> = Cuboid {
            half_edges: [2_i32; 8],
            ..Default::default()
        };
        assert!(cuboid.within([1; 8]));
    }

    #[test]
    fn cuboid_moves() {
        let cuboid: Cuboid<i32, 2> = Cuboid {
            half_edges: [2, 2],
            center: [2, 2],
        };
        assert!(!cuboid.within([0, 0]));
        assert!(cuboid.within([1, 1]));
        assert!(cuboid.within([2, 2]));
        assert!(cuboid.within([3, 3]));
        assert!(cuboid.within([1, 3]));
    }

    #[test]
    fn cuboid_works_with_distinct_sides() {
        let cuboid: Cuboid<i32, 3> = Cuboid {
            half_edges: [2, 3, 4],
            ..Default::default()
        };
        assert!(cuboid.within([1, 2, 3]));
        assert!(cuboid.within([-1, -2, -3]));
        assert!(cuboid.within([-1, 2, -3]));
        assert!(!cuboid.within([1, 2, 4]));
        assert!(!cuboid.within([1, 3, 3]));
        assert!(!cuboid.within([-2, -2, -3]));
    }

    #[test]
    fn returns_center() {
        let center = [2.0, 2.0];
        let cuboid: Cuboid<f64, 2> = Cuboid {
            center,
            ..Default::default()
        };
        assert_eq!(&center, cuboid.center());
    }

    #[test]
    fn returns_center_distance() {
        let cuboid: Cuboid<f64, 2> = Cuboid::default();
        assert_eq!(0.0, cuboid.center_distance([0.0, 0.0]));
        assert_eq!(2.0, cuboid.center_distance([2.0, 0.0]));
        assert_eq!(2.0, cuboid.center_distance([0.0, 2.0]));
        assert_eq!(5.0, cuboid.center_distance([3.0, 4.0]));
    }

    #[test]
    fn returns_center_distance_for_int() {
        let cuboid: Cuboid<i8, 2> = Cuboid::default();
        assert_eq!(0, cuboid.center_distance([0, 0]));
        assert_eq!(2, cuboid.center_distance([2, 0]));
        assert_eq!(2, cuboid.center_distance([0, 2]));
        assert_eq!(5, cuboid.center_distance([3, 4]));
    }

    #[test]
    fn returns_center_distance_for_uint() {
        let cuboid: Cuboid<u8, 2> = Cuboid::default();
        assert_eq!(0, cuboid.center_distance([0, 0]));
        assert_eq!(2, cuboid.center_distance([2, 0]));
        assert_eq!(2, cuboid.center_distance([0, 2]));
        assert_eq!(5, cuboid.center_distance([3, 4]));
    }

    #[test]
    fn returns_distance() {
        let cuboid: Cuboid<f64, 2> = Cuboid::default();
        assert_eq!(0.0, cuboid.distance([0.0, 0.0]));
        assert_eq!(1.0, cuboid.distance([2.0, 0.0]));
        assert_eq!(1.0, cuboid.distance([2.0, -0.2]));
        assert_eq!(1.0, cuboid.distance([0.1, 2.0]));
        assert_eq!(5.0, cuboid.distance([4.0, 5.0]));
    }

    #[test]
    fn returns_distance_for_other_sides() {
        let cuboid: Cuboid<i32, 3> = Cuboid {
            half_edges: [5, 10, 15],
            ..Default::default()
        };
        assert_eq!(0, cuboid.distance([0, 0, 0]));
        assert_eq!(0, cuboid.distance([1, 10, -15]));
        assert_eq!(1, cuboid.distance([2, -10, 16]));
    }

    #[test]
    fn returns_distance_for_other_center() {
        let cuboid: Cuboid<f64, 2> = Cuboid {
            center: [4.0, 5.0],
            ..Default::default()
        };
        assert_eq!(5.0, cuboid.distance([0.0, 0.0]));
        assert_eq!(0.0, cuboid.distance([4.0, 5.0]));
        assert_eq!(1.0, cuboid.distance([5.0, 7.0]));
    }
}
