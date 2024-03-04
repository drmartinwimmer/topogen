use crate::Generator;

pub trait Bitmap<T, const D: usize>: Generator<[T; D], Output = bool> {}

#[derive(Debug)]
pub struct Sphere<T, const D: usize> {
    pub radius: T,
    pub center: [T; D],
}

macro_rules! impl_sphere {
    ($t:ty) => {
        impl<const D: usize> Default for Sphere<$t, D> {
            fn default() -> Self {
                Self {
                    radius: 1 as $t,
                    center: [<$t>::default(); D],
                }
            }
        }

        impl<const D: usize> Generator<[$t; D]> for Sphere<$t, D> {
            type Output = bool;

            fn generate(&self, point: [$t; D]) -> bool {
                let radius: $t = point
                    .into_iter()
                    .zip(self.center)
                    .map(|(p, c)| (p - c) * (p - c))
                    .sum();
                radius <= self.radius * self.radius
            }
        }
    };
}

#[derive(Debug)]
pub struct Cube<T, const D: usize> {
    pub side: T,
    pub center: [T; D],
}

macro_rules! impl_cube {
    ($t:ty) => {
        impl<const D: usize> Default for Cube<$t, D> {
            fn default() -> Self {
                Self {
                    side: 1 as $t,
                    center: [<$t>::default(); D],
                }
            }
        }

        impl<const D: usize> Generator<[$t; D]> for Cube<$t, D> {
            type Output = bool;

            fn generate(&self, point: [$t; D]) -> bool {
                point
                    .into_iter()
                    .zip(self.center)
                    .all(|(p, c)| 2 as $t * (p - c).abs() <= self.side)
            }
        }
    };
}

macro_rules! impl_all {
    ($($t:ty)+) => {
        $(
            impl_sphere!($t);
            impl_cube!($t);
        )+
    }
}

impl_all!(f32 f64 i8 i16 i32 i64);

#[cfg(test)]
mod tests {
    use crate::{
        geometry::bitmap::{Cube, Sphere},
        Generator,
    };

    #[test]
    fn generates_sphere() {
        let sphere: Sphere<f64, 2> = Sphere::default();
        assert!(sphere.generate([0.9, 0.0]));
        assert!(sphere.generate([0.7, 0.3]));
        assert!(!sphere.generate([1.0, 0.3]));
        assert!(!sphere.generate([0.9, 0.9]));
    }

    #[test]
    fn sphere_works_for_negatives() {
        let sphere: Sphere<f64, 3> = Sphere::default();
        assert!(sphere.generate([-0.7, -0.3, 0.0]));
        assert!(!sphere.generate([-1.0, 0.3, 0.0]));
    }

    #[test]
    fn sphere_works_across_dimensions() {
        let sphere: Sphere<f64, 1> = Sphere::default();
        assert!(sphere.generate([0.9]));
        let sphere: Sphere<f64, 4> = Sphere::default();
        assert!(sphere.generate([0.4, 0.3, 0.2, 0.1]));
    }

    #[test]
    fn sphere_works_across_types() {
        let sphere: Sphere<i8, 1> = Sphere::default();
        assert!(sphere.generate([1]));
        assert!(!sphere.generate([2]));
        let sphere: Sphere<i64, 1> = Sphere::default();
        assert!(sphere.generate([1]));
    }

    #[test]
    fn sphere_scales() {
        let mut sphere: Sphere<f64, 3> = Sphere {
            radius: 0.9,
            ..Default::default()
        };
        assert!(!sphere.generate([0.91, 0.0, 0.0]));
        sphere.radius = 3.0;
        assert!(sphere.generate([0.9, 0.0, 0.0]));
        assert!(sphere.generate([1.5, 1.5, 1.5]));
        assert!(!sphere.generate([2.0, 2.0, 2.0]));
    }

    #[test]
    fn sphere_moves() {
        let sphere: Sphere<f64, 2> = Sphere {
            center: [2.0, 2.0],
            ..Default::default()
        };
        assert!(!sphere.generate([0.0, 0.0]));
        assert!(sphere.generate([2.0, 2.0]));
        assert!(sphere.generate([1.0, 2.0]));
        assert!(sphere.generate([2.0, 3.0]));
        assert!(!sphere.generate([3.0, 3.0]));
    }

    #[test]
    fn generates_cube() {
        let cube: Cube<f64, 2> = Cube::default();
        assert!(cube.generate([0.5, 0.5]));
        assert!(!cube.generate([0.5, 0.51]));
    }

    #[test]
    fn cube_works_for_negatives() {
        let cube: Cube<f64, 1> = Cube::default();
        assert!(cube.generate([-0.5]));
        assert!(!cube.generate([-0.51]));
    }

    #[test]
    fn cube_works_across_types() {
        let cube: Cube<i32, 1> = Cube {
            side: 10_i32,
            ..Default::default()
        };
        assert!(cube.generate([5]));
        assert!(!cube.generate([6]));
    }

    #[test]
    fn unit_cube_works_across_dimensions() {
        let cube: Cube<i32, 8> = Cube {
            side: 2_i32,
            ..Default::default()
        };
        assert!(cube.generate([1; 8]));
    }

    #[test]
    fn cube_moves() {
        let cube: Cube<i32, 2> = Cube {
            side: 2,
            center: [2, 2],
        };
        assert!(!cube.generate([0, 0]));
        assert!(cube.generate([1, 1]));
        assert!(cube.generate([2, 2]));
        assert!(cube.generate([3, 3]));
        assert!(cube.generate([1, 3]));
    }
}
