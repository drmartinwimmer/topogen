use crate::geometry::{object::BoundingBox, GeometricObject, WithBoundingBox};
use crate::Generator;
use std::marker::PhantomData;

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
