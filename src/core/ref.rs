use crate::shapes::BoundingBox;
use crate::traits::{Bounded, Center, Coordinate, Euclidean, Generator, Shape};

/// A lightweight reference adapter forwarding spatial and generator traits without consuming the
/// underlying value.
#[derive(Debug, PartialEq, Eq)]
pub struct Ref<'a, F> {
    referenced: &'a F,
}

impl<F> Clone for Ref<'_, F> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<F> Copy for Ref<'_, F> {}

impl<'a, F> Ref<'a, F> {
    /// Wraps a shared reference so combinators can be chained without moving `referenced`.
    pub fn new(referenced: &'a F) -> Self {
        Self { referenced }
    }
}

impl<G: Generator<I>, I> Generator<I> for Ref<'_, G> {
    type Output = G::Output;

    fn generate(&self, input: I) -> G::Output {
        self.referenced.generate(input)
    }
}

impl<E, T: Coordinate, const D: usize> Euclidean<T, D> for Ref<'_, E> where E: Euclidean<T, D> {}

impl<B, T, const D: usize> Bounded<T, D> for Ref<'_, B>
where
    T: Coordinate,
    B: Bounded<T, D>,
{
    fn within(&self, input: [T; D]) -> bool {
        self.referenced.within(input)
    }

    fn bounding_box(&self) -> BoundingBox<T, D> {
        self.referenced.bounding_box()
    }
}

impl<C, T, const D: usize> Center<T, D> for Ref<'_, C>
where
    T: Coordinate,
    C: Center<T, D>,
{
    fn center(&self) -> [T; D] {
        self.referenced.center()
    }
}

impl<S, T, const D: usize> Shape<T, D> for Ref<'_, S>
where
    T: Coordinate,
    S: Shape<T, D>,
{
    fn distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float,
    {
        self.referenced.distance(point)
    }

    fn signed_distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float,
    {
        self.referenced.signed_distance(point)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shapes::Sphere,
        traits::{Bounded, ByRef, Center, Generator, Shape},
    };
    use googletest::prelude::*;

    #[googletest::test]
    fn ref_forwards_spatial_and_generator_traits() {
        let sphere = Sphere {
            radius: 3.0_f64,
            center: [1.0, -1.0],
        };
        let borrowed = sphere.by_ref();
        expect_that!(borrowed.center(), eq([1.0, -1.0]));
        expect_that!(borrowed.within([2.0, -1.0]), is_true());
        expect_that!(borrowed.distance([5.0, -1.0]), eq(1.0));
        expect_that!(borrowed.signed_distance([1.0, -1.0]), eq(-3.0));
        let bitmap = sphere.by_ref().bitmap();
        expect_that!(bitmap.generate([2.0, -1.0]), is_true());
        expect_that!(sphere.within([2.0, -1.0]), is_true());
    }
}
