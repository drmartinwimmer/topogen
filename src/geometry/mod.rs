pub mod bitmap;
pub mod object;
pub mod ops;

use self::object::Cuboid;
use crate::Generator;
use ops::Repeat;

pub trait GeometricObject<T, const D: usize>: Sized + Clone {
    fn center(&self) -> [T; D];
    fn bounding_box(&self) -> Cuboid<T, D>;
    fn within(&self, point: [T; D]) -> bool;
    fn distance(&self, point: [T; D]) -> T;
    fn center_distance(&self, point: [T; D]) -> T;

    fn bitmap(self) -> ops::Bitmap<T, Self, D> {
        ops::Bitmap::new(self)
    }
}

pub trait Cartesian<T, const D: usize>: Generator<[T; D]> {
    fn repeat(self, center: [T; D], repeat: [T; D]) -> Repeat<T, Self, D> {
        Repeat {
            generator: self,
            center,
            repeat,
        }
    }
}

impl<G, T, const D: usize> Cartesian<T, D> for G where G: Generator<[T; D]> {}

#[cfg(test)]
mod tests {
    use super::{Cartesian, GeometricObject};
    use crate::{geometry::object::Sphere, Generator};

    #[test]
    fn repeats_sphere() {
        let sphere: Sphere<i32, 2> = Sphere {
            radius: 10,
            ..Default::default()
        };
        let repeat = sphere.bitmap().repeat([0; 2], [100, 50]);
        assert!(repeat.generate([10, 0]));
        assert!(repeat.generate([-9, 0]));
        assert!(!repeat.generate([11, 0]));
        assert!(repeat.generate([90, 0]));
        assert!(repeat.generate([210, 300]));
    }

    #[test]
    fn repeats_quarter_sphere_without_center() {
        let sphere: Sphere<i32, 2> = Sphere {
            radius: 10,
            ..Default::default()
        };
        let repeat = sphere.bitmap().repeat([50; 2], [100, 100]);
        assert!(repeat.generate([0, 0]));
        assert!(repeat.generate([10, 0]));
        assert!(repeat.generate([0, 10]));
        assert!(!repeat.generate([-1, 0]));
    }
}
