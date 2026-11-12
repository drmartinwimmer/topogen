use crate::ops;

pub use topogen_derive::Generator;

/// Evaluates a procedural value of type [`Self::Output`] for any input `I`.
pub trait Generator<I>: Sized {
    /// Output value produced at each evaluated input.
    type Output;

    /// Evaluates the generator at `input`.
    fn generate(&self, input: I) -> Self::Output;

    /// Transforms each generated output value using `f`.
    fn map<O, F>(self, f: F) -> ops::Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> O,
    {
        ops::Map::new(self, f)
    }

    /// Adapts the generator to accept inputs of type `NI` mapped to `I` via `f`.
    fn map_input<NI, F>(self, f: F) -> ops::MapInput<Self, F>
    where
        Self: Sized,
        F: Fn(NI) -> I,
    {
        ops::MapInput::new(self, f)
    }

    /// Clamps generated output values into the closed interval `[min, max]`.
    fn clamp(self, min: Self::Output, max: Self::Output) -> ops::Clamp<Self, I> {
        ops::Clamp::new(self, min, max)
    }

    /// Evaluates `self` and `other` at the same `Copy` input and combines their outputs with `combiner`.
    fn zip<B, O, F>(self, other: B, combiner: F) -> ops::Zip<Self, B, F>
    where
        I: Copy,
        B: Generator<I>,
        F: Fn(Self::Output, B::Output) -> O,
    {
        ops::Zip::new(self, other, combiner)
    }
}

#[cfg(test)]
mod tests {
    use super::Generator;
    use crate::{
        shapes::{Point, Sphere},
        traits::{Bounded, Center},
    };
    use googletest::prelude::*;
    use std::f32::consts::FRAC_PI_2;

    #[googletest::test]
    fn maps_generator_output() {
        let sphere = Sphere::default().bitmap().map(|b: bool| !b);
        expect_that!(sphere.generate([0]), is_false());
        expect_that!(sphere.generate([2]), is_true());
    }

    #[googletest::test]
    fn clamps_generator_output() {
        let generator = Point::default().center_sin();
        expect_that!(generator.generate([0.0]), eq(0.0));
        expect_that!(generator.generate([FRAC_PI_2]), eq(1.0));
        expect_that!(generator.generate([3.0 * FRAC_PI_2]), eq(-1.0));
        let clamped = generator.clamp(-0.5, 0.5);
        expect_that!(clamped.generate([0.0]), eq(0.0));
        expect_that!(clamped.generate([FRAC_PI_2]), eq(0.5));
        expect_that!(clamped.generate([3.0 * FRAC_PI_2]), eq(-0.5));
    }
}
