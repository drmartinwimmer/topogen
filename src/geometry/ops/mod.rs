use std::marker::PhantomData;

use super::{object::BoundingBox, Cartesian, GeometricObject, WithBoundingBox};
use crate::Generator;

#[derive(Debug, Clone)]
pub struct Bitmap<T, O: GeometricObject<T, D>, const D: usize> {
    t: PhantomData<T>,
    pub object: O,
}

impl<T, O: GeometricObject<T, D>, const D: usize> Bitmap<T, O, D> {
    pub(crate) fn new(object: O) -> Self {
        Self {
            t: PhantomData,
            object,
        }
    }
}

impl<T, O: GeometricObject<T, D>, const D: usize> Generator<[T; D]> for Bitmap<T, O, D>
where
    T: Clone,
{
    type Output = bool;

    fn generate(&self, input: [T; D]) -> Self::Output {
        self.object.within(input)
    }
}

impl<T, O: GeometricObject<T, D>, const D: usize> WithBoundingBox<T, D> for Bitmap<T, O, D>
where
    O: WithBoundingBox<T, D>,
    T: Clone + Copy,
{
    fn bounding_box(&self) -> BoundingBox<T, D> {
        self.object.bounding_box()
    }
}

#[derive(Debug, Clone)]
pub struct Repeat<T, G: Cartesian<T, D>, const D: usize> {
    pub generator: G,
    pub center: [T; D],
    pub repeat: [T; D],
}

impl<T, G: Cartesian<T, D>, const D: usize> WithBoundingBox<T, D> for Repeat<T, G, D>
where
    T: Clone + Copy,
{
    fn bounding_box(&self) -> BoundingBox<T, D> {
        BoundingBox::Unbounded
    }
}

macro_rules! impl_repeat {
    ($($t:ty),+) => {$(
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
