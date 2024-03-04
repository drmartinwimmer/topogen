use crate::Generator;

pub mod bitmap;

pub trait Remainder {
    fn remainder(&self, rhs: Self) -> Self;
}

macro_rules! impl_euclid {
    ($($t:ty),+) => {$(
        impl Remainder for $t {
            fn remainder(&self, rhs: Self) -> Self {
                self.rem_euclid(rhs)
            }
        }
    )+}
}
impl_euclid!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

pub trait Cartesian<T, const D: usize>: Generator<[T; D]>
where
    Self: Sized,
    T: Remainder + Copy + 'static,
{
    fn repeat(self, repeat: [T; D]) -> impl Generator<[T; D]> {
        self.map_input(move |mut point| {
            point
                .iter_mut()
                .zip(repeat.iter())
                .for_each(|(p, r)| *p = p.remainder(*r));
            point
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::Cartesian;
//     use crate::geometry::bitmap::Sphere;

//     #[test]
//     fn repeats_sphere() {
//         let sphere: Sphere<i32, 2> = Sphere {
//             radius: 10,
//             ..Default::default()
//         };
//         let repeat = sphere.repeat([100, 50]);
//         assert!(sphere.generate([0.9, 0.0]));
//         assert!(sphere.generate([0.7, 0.3]));
//         assert!(!sphere.generate([1.0, 0.3]));
//         assert!(!sphere.generate([0.9, 0.9]));
//     }
// }
