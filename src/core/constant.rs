use crate::{
    shapes::BoundingBox,
    traits::{Bounded, Coordinate, Euclidean, Generator},
};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Constant<V: Copy, I = V> {
    pub value: V,
    input: PhantomData<fn(I)>,
}

impl<V: Copy, I> Clone for Constant<V, I> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<V: Copy, I> Copy for Constant<V, I> {}

impl<V: Copy + PartialEq, I> PartialEq for Constant<V, I> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<V: Copy + Eq, I> Eq for Constant<V, I> {}

impl<V: Copy, I> Constant<V, I> {
    pub fn new(value: V) -> Self {
        Self {
            value,
            input: PhantomData,
        }
    }
}

impl<V: Copy, I> Generator<I> for Constant<V, I> {
    type Output = V;

    fn generate(&self, _input: I) -> V {
        self.value
    }
}

impl<V: Copy, T: Coordinate, const D: usize> Euclidean<T, D> for Constant<V, [T; D]> {}

impl<V: Copy, T: Coordinate, const D: usize> Bounded<T, D> for Constant<V, [T; D]> {
    fn within(&self, _input: [T; D]) -> bool {
        true
    }

    fn bounding_box(&self) -> BoundingBox<T, D> {
        BoundingBox::Unbounded
    }
}

#[cfg(test)]
mod tests {
    use super::Constant;
    use crate::{
        shapes::Cuboid,
        traits::{Bounded, Euclidean, Generator},
    };
    use googletest::prelude::*;

    #[googletest::test]
    fn constant_generates_value_for_spatial_coordinates() {
        let field = Constant::new(42_u32).bound_to(Cuboid {
            center: [0, 0],
            extents: [2, 2],
        });
        expect_that!(field.generate([0, 0]), eq(42));
        expect_that!(field.generate([3, 0]), eq(0));
        expect_that!(Constant::new(7_i32).within([100, -100]), is_true());
    }
}
