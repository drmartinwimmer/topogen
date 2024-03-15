use crate::core::GeneratorRef;
use crate::ops;

pub trait Generator<I>: Sized {
    type Output;

    fn generate(&self, input: I) -> Self::Output;

    fn as_ref(&self) -> GeneratorRef<Self> {
        GeneratorRef::new(self)
    }

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
}

#[cfg(test)]
mod tests {
    use super::Generator;
    use crate::{shapes::Sphere, traits::Bounded};

    #[test]
    fn map_works() {
        let sphere = Sphere::default().bitmap().map(|b: bool| !b);
        assert!(!sphere.generate([0]));
        assert!(sphere.generate([2]));
    }
}
