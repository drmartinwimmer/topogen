use crate::{
    shapes::{BoundingBox, BoundingBoxError},
    traits::{Bounded, Coordinate, Euclidean, Generator, Shape},
};
use num_traits::Float;

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

impl<G, F, T, const D: usize> Euclidean<T, D> for Union<G, F, T, D>
where
    T: Coordinate,
    G: Bounded<T, D>,
{
}

impl<G, F, T, const D: usize> Bounded<T, D> for Union<G, F, T, D>
where
    T: Coordinate,
    G: Bounded<T, D>,
{
    fn within(&self, input: [T; D]) -> bool {
        self.bounding_box.within(input) && self.elements.iter().all(|e| e.within(input))
    }

    fn bounding_box(&self) -> BoundingBox<T, D> {
        self.bounding_box.clone()
    }
}

impl<G, F, T, const D: usize> Shape<T, D> for Union<G, F, T, D>
where
    T: Coordinate,
    G: Shape<T, D>,
{
    fn distance(&self, point: [T; D]) -> T
    where
        T: Float,
    {
        self.elements
            .iter()
            .map(|e| e.distance(point))
            .reduce(Float::min)
            .unwrap_or(T::infinity())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error<T: std::fmt::Display> {
    #[error("Bounding box operation failed: {0:?}")]
    UpperBoundFailed(#[from] BoundingBoxError<T>),
}

#[cfg(test)]
mod tests {
    use super::Union;
    use crate::{
        shapes::Cuboid,
        traits::{Bounded, Generator},
    };
    use googletest::prelude::*;

    #[googletest::test]
    fn generator_union_works() -> Result<()> {
        let left_cube = Cuboid {
            center: [-2, 0, 0],
            ..Default::default()
        };
        let right_cube = Cuboid {
            center: [2, 0, 0],
            ..Default::default()
        };
        let combined_bitmap =
            Union::new(vec![left_cube.bitmap(), right_cube.bitmap()], |a, b| a | b)?;
        expect_that!(combined_bitmap.generate([-1, 0, 0]), is_false());
        expect_that!(combined_bitmap.generate([0, 0, 0]), is_false());
        expect_that!(combined_bitmap.generate([1, 0, 0]), is_false());
        Ok(())
    }
}
