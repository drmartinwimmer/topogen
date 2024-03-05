pub mod geometry;
pub mod ops;

pub trait Generator<I>: Sized {
    type Output;

    fn generate(&self, input: I) -> Self::Output;

    fn map<O, F>(self, f: F) -> ops::MapOutput<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> O,
    {
        ops::MapOutput::new(self, f)
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
    use crate::geometry::object::Sphere;

    #[test]
    fn map_works() {
        let sphere = Sphere::default().map(|b: bool| !b);
        assert!(!sphere.generate([0]));
        assert!(sphere.generate([2]));
    }
}
