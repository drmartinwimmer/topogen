use crate::{
    shapes::{BoundingBox, BoundingBoxError},
    traits::{Bounded, Coordinate, Generator},
};

#[derive(Debug)]
pub struct Union<G, F, T, const D: usize>
where
    T: Coordinate,
    G: Bounded<T, D>,
{
    bounding_box: BoundingBox<T, D>,
    elements: Vec<G>,
    reducer: F,
}

impl<T, G, F, const D: usize> Union<G, F, T, D>
where
    T: Coordinate,
    G: Bounded<T, D>,
{
    pub fn new(elements: Vec<G>, reducer: F) -> Result<Self, Error<T>> {
        let mut bounding_box = BoundingBox::Empty;
        for element in &elements {
            bounding_box = bounding_box.for_union(&element.bounding_box())?;
        }
        Ok(Self {
            bounding_box,
            elements,
            reducer,
        })
    }
}

impl<T, G, F, const D: usize> Generator<[T; D]> for Union<G, F, T, D>
where
    T: Coordinate,
    G: Bounded<T, D> + Generator<[T; D]>,
    G::Output: Default,
    F: Fn(G::Output, G::Output) -> G::Output + Clone,
{
    type Output = G::Output;

    fn generate(&self, input: [T; D]) -> Self::Output {
        if !self.bounding_box.within(input) {
            return Self::Output::default();
        }
        self.elements
            .iter()
            .filter(|e| e.within(input))
            .map(|e| e.generate(input))
            .reduce(&self.reducer)
            .unwrap_or_default()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error<T: std::fmt::Display> {
    #[error("Bounding box operation failed: {0:?}")]
    UpperBoundFailed(#[from] BoundingBoxError<T>),
}
