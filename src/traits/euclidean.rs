use super::{Bounded, Generator};
use crate::ops::{BoundTo, Fbm, Scale, Translate};
use num_traits::Float;

pub use generative_map_derive::Euclidean;

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

pub trait EuclideanVec<T: Coordinate, const D: usize>: Sized {
    fn add(self, rhs: Self) -> Self;
    fn add_assign(&mut self, rhs: Self);
    fn sub(self, rhs: Self) -> Self;
    fn sub_assign(&mut self, rhs: Self);
    fn scalar_mul(self, rhs: T) -> Self;
    fn scalar_mul_assign(&mut self, rhs: T);
    fn element_mul(self, rhs: Self) -> Self;
    fn element_mul_assign(&mut self, rhs: Self);
    fn scalar_div(self, rhs: T) -> Self;
    fn scalar_div_assign(&mut self, rhs: T);
    fn element_div(self, rhs: Self) -> Self;
    fn element_div_assign(&mut self, rhs: Self);
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

pub trait WrapToU8 {
    fn wrap_to_u8(self) -> u8;
}

impl WrapToU8 for f32 {
    fn wrap_to_u8(self) -> u8 {
        self.rem_euclid(256.0) as u8
    }
}

impl WrapToU8 for f64 {
    fn wrap_to_u8(self) -> u8 {
        self.rem_euclid(256.0) as u8
    }
}

pub trait EuclideanVecFloat<T: Coordinate + Float, const D: usize>: EuclideanVec<T, D> {
    fn floor(self) -> Self;
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

pub trait Euclidean<T: Coordinate, const D: usize>: Sized {
    fn bound_to<B: Bounded<T, D>>(self, bound_to: B) -> BoundTo<Self, B, T, D> {
        BoundTo::new(self, bound_to)
    }

    fn fbm<O>(
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

    fn scale(self, factor: [T; D]) -> Scale<Self, T, D> {
        Scale::new(self, factor)
    }

    fn translate(self, translation: [T; D]) -> Translate<Self, T, D> {
        Translate::new(self, translation)
    }
}

#[cfg(test)]
mod tests {
    use super::WrapToU8;

    #[test]
    fn wrap_to_u8_works_for_negative() {
        assert_eq!((-2.0).wrap_to_u8(), 254);
        assert_eq!((-1.0).wrap_to_u8(), 255);
        assert_eq!((-0.5).wrap_to_u8(), 255);
        assert_eq!((0.0).wrap_to_u8(), 0);
    }
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
