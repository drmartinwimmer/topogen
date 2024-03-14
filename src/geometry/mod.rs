pub mod object;

//use crate::ops::Repeat;
use crate::traits::{Bounded, Coordinate};

pub trait GeometricObject<T, const D: usize>: Sized + Clone + Bounded<T, D>
where
    T: Coordinate + num_traits::Float,
{
    fn distance(&self, point: [T; D]) -> T;
}

// pub trait Cartesian<T, const D: usize>: Generator<[T; D]> {
//     fn repeat(self, center: [T; D], repeat: [T; D]) -> Repeat<T, Self, D> {
//         Repeat {
//             generator: self,
//             center,
//             repeat,
//         }
//     }
// }
// impl<G, T, const D: usize> Cartesian<T, D> for G where G: Generator<[T; D]> {}

// #[cfg(test)]
// mod tests {
//     use super::Cartesian;
//     use crate::{geometry::object::Sphere, Generator};

//     #[test]
//     fn repeats_sphere() {
//         let sphere: Sphere<i32, 2> = Sphere {
//             radius: 10,
//             ..Default::default()
//         };
//         let repeat = sphere.bitmap().repeat([0; 2], [100, 50]);
//         assert!(repeat.generate([9, 0]));
//         assert!(repeat.generate([-9, 0]));
//         assert!(!repeat.generate([10, 0]));
//         assert!(repeat.generate([91, 0]));
//         assert!(repeat.generate([209, 300]));
//     }

//     #[test]
//     fn repeats_quarter_sphere_without_center() {
//         let sphere: Sphere<i32, 2> = Sphere {
//             radius: 10,
//             ..Default::default()
//         };
//         let repeat = sphere.bitmap().repeat([50; 2], [100, 100]);
//         assert!(repeat.generate([0, 0]));
//         assert!(repeat.generate([9, 0]));
//         assert!(repeat.generate([0, 9]));
//         assert!(!repeat.generate([-1, 0]));
//     }
// }
