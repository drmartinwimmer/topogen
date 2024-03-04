use crate::Generator;
use num_traits::AsPrimitive;
use std::{iter::Sum, ops::Mul};

pub trait Bitmap<T, const D: usize>: Generator<[T; D], Output = bool> {}

struct UnitSphere;

impl<I, const D: usize> Generator<[I; D]> for UnitSphere
where
    I: Copy + Sum + Mul + Sum<<I as Mul>::Output> + std::cmp::PartialOrd<I> + 'static,
    i32: AsPrimitive<I>,
{
    type Output = bool;

    fn generate(&self, point: [I; D]) -> bool {
        let radius: I = point.into_iter().map(|p| p * p).sum();
        radius <= 1_i32.as_()
    }
}

struct UnitCube;

impl<I, const D: usize> Generator<[I; D]> for UnitCube
where
    I: Copy + std::cmp::PartialOrd<I> + num_traits::Signed + 'static,
    i32: AsPrimitive<I>,
{
    type Output = bool;

    fn generate(&self, point: [I; D]) -> bool {
        point.into_iter().all(|p| p.abs() < 1_i32.as_())
    }
}

#[cfg(test)]
mod tests {
    use super::{UnitCube, UnitSphere};
    use crate::Generator;

    #[test]
    fn generates_unit_sphere() {
        let sphere = UnitSphere;
        assert!(sphere.generate([0.9, 0.0]));
        assert!(sphere.generate([0.7, 0.3]));
        assert!(!sphere.generate([1.0, 0.3]));
        assert!(!sphere.generate([0.9, 0.9]));
    }

    #[test]
    fn unit_sphere_works_for_negatives() {
        let sphere = UnitSphere;
        assert!(sphere.generate([-0.7, -0.3, 0.0]));
        assert!(!sphere.generate([-1.0, 0.3, 0.0]));
    }

    #[test]
    fn unit_sphere_works_across_dimensions() {
        let sphere = UnitSphere;
        assert!(sphere.generate([0.9]));
        assert!(sphere.generate([0.7, 0.3]));
        assert!(sphere.generate([0.5, 0.3, 0.2]));
        assert!(sphere.generate([0.4, 0.3, 0.2, 0.1]));
    }

    #[test]
    fn unit_sphere_works_across_types() {
        let sphere = UnitSphere;
        assert!(sphere.generate([0.9_f32]));
        assert!(sphere.generate([0.9_f64]));
        assert!(sphere.generate([0_i32]));
    }

    #[test]
    fn generates_unit_cube() {
        let cube = UnitCube;
        assert!(cube.generate([0.99, 0.99]));
        assert!(!cube.generate([1, 1]));
    }

    #[test]
    fn unit_cube_works_for_negatives() {
        let cube = UnitCube;
        assert!(cube.generate([-0.9]));
        assert!(!cube.generate([-1]));
    }

    #[test]
    fn unit_cube_works_across_types() {
        let cube = UnitCube;
        assert!(cube.generate([0.9_f32]));
        assert!(cube.generate([0.9_f64]));
        assert!(cube.generate([0_i32]));
    }

    #[test]
    fn unit_cube_works_across_dimensions() {
        let cube = UnitCube;
        assert!(cube.generate([0.9]));
        assert!(cube.generate([0.9, 0.9]));
        assert!(cube.generate([0.9, 0.9, 0.9]));
        assert!(cube.generate([0.9, 0.9, 0.9, 0.9]));
    }
}
