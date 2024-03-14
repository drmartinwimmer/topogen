use crate::geometry::{object::BoundingBox, Cartesian};
use crate::traits::Bounded;
use crate::Generator;

#[derive(Debug)]
pub struct Repeat<T, C: Cartesian<T, D>, const D: usize> {
    pub cartesian: C,
    pub center: [T; D],
    pub repeat: [T; D],
}

macro_rules! impl_repeat {
    ($($t:ty),+) => {$(
        impl<C: Cartesian<$t, D>, const D: usize> Bounded<[$t; D]> for Repeat<$t, C, D>
        where
            C: Bounded<[$t; D]>,
        {
            type UpperBound = BoundingBox<$t, D>;
            type Bitmap = C::Bitmap;

            fn within(&self, point: [$t; D]) -> bool {
                self.cartesian.within(point)
            }

            fn upper_bound(&self) -> BoundingBox<$t, D> {
                BoundingBox::Unbounded
            }
        }

        impl<G: Cartesian<$t, D>, const D: usize> Generator<[$t; D]> for Repeat<$t, G, D> {
            type Output = G::Output;

            fn generate(&self, mut point: [$t; D]) -> Self::Output {
                point
                    .iter_mut()
                    .zip(self.center.iter().zip(self.repeat.iter()))
                    .for_each(|(p, (c,r))| {
                        let centered = *p - *c;
                        // Double to avoid rounding errors for ints
                        let offset = (2 as $t) * centered  + *r;
                        let rem = offset.rem_euclid(2 as $t * *r);
                        // Reverse centering and offset and store result
                        *p = (rem - *r) / (2 as $t) + *c;
                    });
                self.generator.generate(point)
            }
        }
    )+}
}

impl_repeat!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
