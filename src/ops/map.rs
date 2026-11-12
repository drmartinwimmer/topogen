use crate::traits::{Bounded, Center, Coordinate, Euclidean, Generator, Shape};

/// Transforms values produced by an inner [`Generator`] using a mapping closure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Euclidean, Center, Bounded, Shape)]
pub struct Map<G, F> {
    #[wrap]
    generator: G,
    mapper: F,
}

impl<G, F> Map<G, F> {
    /// Wraps `generator` so each output is transformed by `mapper`.
    pub fn new(generator: G, mapper: F) -> Self {
        Self { generator, mapper }
    }
}

impl<G, F, I, O> Generator<I> for Map<G, F>
where
    G: Generator<I>,
    F: Fn(G::Output) -> O,
{
    type Output = O;

    fn generate(&self, input: I) -> Self::Output {
        (self.mapper)(self.generator.generate(input))
    }
}

/// Adapts the input type of a [`Generator`] by mapping caller inputs before evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapInput<G, F> {
    generator: G,
    mapper: F,
}

impl<G, F> MapInput<G, F> {
    /// Wraps `generator` so caller inputs are converted via `mapper`.
    pub fn new(generator: G, mapper: F) -> Self {
        Self { generator, mapper }
    }
}

impl<G, F, I, GI> Generator<I> for MapInput<G, F>
where
    G: Generator<GI>,
    F: Fn(I) -> GI,
{
    type Output = G::Output;

    fn generate(&self, input: I) -> Self::Output {
        self.generator.generate((self.mapper)(input))
    }
}

impl<G, F, T, const D: usize> Euclidean<T, D> for MapInput<G, F>
where
    T: Coordinate,
    G: Euclidean<T, D>,
{
}

#[cfg(test)]
mod tests {
    use crate::core::Constant;
    use crate::shapes::Sphere;
    use crate::traits::{Bounded, ByRef, Generator};
    use googletest::prelude::*;
    use std::cell::Cell;

    #[googletest::test]
    fn map_by_value_works() {
        let generator = Constant::new(1);
        let mapped = generator.map(|x| x + 1);
        expect_that!(mapped.generate(0), eq(2));
    }

    #[googletest::test]
    fn map_with_different_output_type_works() {
        let generator = Constant::new(1_i16);
        let mapped = generator.map(|x| f32::from(x) / 2.0);
        expect_that!(mapped.generate(0), eq(0.5));
    }

    #[googletest::test]
    fn map_by_ref_works() {
        let generator = Constant::new(1);
        let mapped = generator.by_ref().map(|x| x + 1);
        expect_that!(mapped.generate(0), eq(2));
        expect_that!(generator.generate(0), eq(1));
    }

    #[googletest::test]
    fn map_supports_non_clone_closure() {
        struct NonCloneOffset(i32);
        let offset = NonCloneOffset(5);
        let counter = Cell::new(0_usize);
        let mapped = Constant::new(10).map(|x| {
            counter.set(counter.get() + 1);
            x + offset.0
        });
        expect_that!(mapped.generate(0), eq(15));
        expect_that!(counter.get(), eq(1));
    }

    #[googletest::test]
    fn map_input_adapts_coordinate_type_and_dimension() {
        let circle = Sphere {
            radius: 5.0_f64,
            center: [0.0, 0.0],
        }
        .bitmap();
        let grid_slice = circle.map_input(|[x, y, _z]: [i32; 3]| [f64::from(x), f64::from(y)]);
        expect_that!(grid_slice.generate([3, 3, 99]), is_true());
        expect_that!(grid_slice.generate([4, 4, 99]), is_false());
    }
}
