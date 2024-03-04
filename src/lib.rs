pub mod geometry;

pub trait Generator<I> {
    type Output;

    fn generate(&self, input: I) -> Self::Output;
}
