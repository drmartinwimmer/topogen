use crate::ops;

pub use topogen_derive::Generator;

pub trait Generator<I>: Sized {
    type Output;

    fn generate(&self, input: I) -> Self::Output;

    fn map<O, F>(self, f: F) -> ops::Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> O,
    {
        ops::Map::new(self, f)
    }

    fn map_input<O, F>(self, f: F) -> ops::MapInput<Self, F>
    where
        Self: Sized,
        F: Fn(I) -> O,
    {
        ops::MapInput::new(self, f)
    }

    fn clamp(self, min: Self::Output, max: Self::Output) -> ops::Clamp<Self, I> {
        ops::Clamp::new(self, min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::Generator;
    use crate::{
        shapes::{Point, Sphere},
        traits::{Bounded, Center},
    };
    use std::f32::consts::FRAC_PI_2;

    #[test]
    fn map_works() {
        let sphere = Sphere::default().bitmap().map(|b: bool| !b);
        assert!(!sphere.generate([0]));
        assert!(sphere.generate([2]));
    }

    #[test]
    fn clamp_works() {
        let gen = Point::default().center_sin();
        assert_eq!(gen.generate([0.0]), 0.0);
        assert_eq!(gen.generate([FRAC_PI_2]), 1.0);
        assert_eq!(gen.generate([3.0 * FRAC_PI_2]), -1.0);
        let clamped = gen.clamp(-0.5, 0.5);
        assert_eq!(clamped.generate([0.0]), 0.0);
        assert_eq!(clamped.generate([FRAC_PI_2]), 0.5);
        assert_eq!(clamped.generate([3.0 * FRAC_PI_2]), -0.5);
    }
}
