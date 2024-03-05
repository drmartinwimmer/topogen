use crate::Generator;

pub trait Bitmap<T, const D: usize>: Generator<[T; D], Output = bool> {}

#[derive(Debug)]
pub struct Cuboid<T, const D: usize> {
    pub sides: [T; D],
    pub center: [T; D],
}

macro_rules! impl_cuboid_default {
    ($t:ty) => {
        impl<const D: usize> Default for Cuboid<$t, D> {
            fn default() -> Self {
                Self {
                    sides: [1 as $t; D],
                    center: [<$t>::default(); D],
                }
            }
        }
    };
}

macro_rules! impl_cuboid_generator_signed {
    ($t:ty) => {
        impl<const D: usize> Generator<[$t; D]> for Cuboid<$t, D> {
            type Output = bool;

            fn generate(&self, point: [$t; D]) -> bool {
                point
                    .into_iter()
                    .zip(self.sides.iter().zip(self.center.iter()))
                    .all(|(p, (s, c))| 2 as $t * (p - c).abs() <= *s)
            }
        }
    };
}

macro_rules! impl_cuboid_generator_unsigned {
    ($t:ty) => {
        impl<const D: usize> Generator<[$t; D]> for Cuboid<$t, D> {
            type Output = bool;

            fn generate(&self, point: [$t; D]) -> bool {
                point
                    .into_iter()
                    .zip(self.sides.iter().zip(self.center.iter()))
                    .all(|(p, (s, c))| 2 as $t * p - c <= *s)
            }
        }
    };
}

macro_rules! impl_cuboid_signed {
    ($($t:ty),+) => {$(
        impl_cuboid_default!($t);
        impl_cuboid_generator_signed!($t);
    )+};
}

macro_rules! impl_cuboid_unsigned {
    ($($t:ty),+) => {$(
        impl_cuboid_default!($t);
        impl_cuboid_generator_unsigned!($t);
    )+};
}

impl_cuboid_signed!(i8, i16, i32, i64, i128, isize, f32, f64);
impl_cuboid_unsigned!(u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use super::Cuboid;
    use crate::Generator;

    #[test]
    fn generates_cuboid() {
        let cuboid: Cuboid<f64, 2> = Cuboid::default();
        assert!(cuboid.generate([0.5, 0.5]));
        assert!(!cuboid.generate([0.5, 0.51]));
    }

    #[test]
    fn cuboid_works_for_negatives() {
        let cuboid: Cuboid<f64, 1> = Cuboid::default();
        assert!(cuboid.generate([-0.5]));
        assert!(!cuboid.generate([-0.51]));
    }

    #[test]
    fn cuboid_works_across_types() {
        let cuboid: Cuboid<i32, 1> = Cuboid {
            sides: [10_i32],
            ..Default::default()
        };
        assert!(cuboid.generate([5]));
        assert!(!cuboid.generate([6]));
    }

    #[test]
    fn unit_cuboid_works_across_dimensions() {
        let cuboid: Cuboid<i32, 8> = Cuboid {
            sides: [2_i32; 8],
            ..Default::default()
        };
        assert!(cuboid.generate([1; 8]));
    }

    #[test]
    fn cuboid_moves() {
        let cuboid: Cuboid<i32, 2> = Cuboid {
            sides: [2, 2],
            center: [2, 2],
        };
        assert!(!cuboid.generate([0, 0]));
        assert!(cuboid.generate([1, 1]));
        assert!(cuboid.generate([2, 2]));
        assert!(cuboid.generate([3, 3]));
        assert!(cuboid.generate([1, 3]));
    }

    #[test]
    fn cuboid_works_with_distinct_sides() {
        let cuboid: Cuboid<i32, 3> = Cuboid {
            sides: [2, 4, 6],
            ..Default::default()
        };
        assert!(cuboid.generate([1, 2, 3]));
        assert!(cuboid.generate([-1, -2, -3]));
        assert!(cuboid.generate([-1, 2, -3]));
        assert!(!cuboid.generate([1, 2, 4]));
        assert!(!cuboid.generate([1, 3, 3]));
        assert!(!cuboid.generate([-2, -2, -3]));
    }
}
