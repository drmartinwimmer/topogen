use crate::{
    shapes::{BoundingBox, BoundingBoxError},
    traits::{Bounded, Coordinate, Euclidean, Generator, Shape},
};
use num_traits::Float;

#[derive(Debug)]
pub struct Intersect<G, F, T, const D: usize>
where
    T: Coordinate,
    G: Bounded<T, D>,
{
    bounding_box: BoundingBox<T, D>,
    elements: Vec<G>,
    reducer: F,
}

impl<T, G, F, const D: usize> Intersect<G, F, T, D>
where
    T: Coordinate,
    G: Bounded<T, D>,
{
    pub fn new(elements: Vec<G>, reducer: F) -> Result<Self, IntersectError<T>> {
        if elements.is_empty() {
            return Ok(Self {
                bounding_box: BoundingBox::Empty,
                elements,
                reducer,
            });
        }
        let mut bounding_box = BoundingBox::Unbounded;
        for element in &elements {
            bounding_box = bounding_box.for_intersect(&element.bounding_box())?;
        }
        Ok(Self {
            bounding_box,
            elements,
            reducer,
        })
    }
}

impl<T, G, F, const D: usize> Generator<[T; D]> for Intersect<G, F, T, D>
where
    T: Coordinate,
    G: Bounded<T, D> + Generator<[T; D]>,
    G::Output: Default,
    F: Fn(G::Output, G::Output) -> G::Output,
{
    type Output = G::Output;

    fn generate(&self, input: [T; D]) -> Self::Output {
        if !self.within(input) {
            return Self::Output::default();
        }
        self.elements
            .iter()
            .map(|e| e.generate(input))
            .reduce(&self.reducer)
            .unwrap_or_default()
    }
}

impl<G, F, T, const D: usize> Euclidean<T, D> for Intersect<G, F, T, D>
where
    T: Coordinate,
    G: Bounded<T, D>,
{
}

impl<G, F, T, const D: usize> Bounded<T, D> for Intersect<G, F, T, D>
where
    T: Coordinate,
    G: Bounded<T, D>,
{
    fn within(&self, input: [T; D]) -> bool {
        !self.elements.is_empty()
            && self.bounding_box.within(input)
            && self.elements.iter().all(|e| e.within(input))
    }

    fn bounding_box(&self) -> BoundingBox<T, D> {
        self.bounding_box
    }
}

impl<G, F, T, const D: usize> Shape<T, D> for Intersect<G, F, T, D>
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
            .reduce(Float::max)
            .unwrap_or(T::infinity())
    }

    fn signed_distance(&self, point: [T; D]) -> T
    where
        T: Float,
    {
        self.elements
            .iter()
            .map(|e| e.signed_distance(point))
            .reduce(Float::max)
            .unwrap_or(T::infinity())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum IntersectError<T: std::fmt::Display> {
    #[error("Bounding box operation failed: {0:?}")]
    UpperBoundFailed(#[from] BoundingBoxError<T>),
}

#[cfg(test)]
mod tests {
    use super::Intersect;
    use crate::{
        shapes::{BoundingBox, Cuboid, DimensionBound},
        traits::{Bounded, Generator, Shape},
    };
    use googletest::prelude::*;

    #[googletest::test]
    fn generator_intersect_matches_overlap_region() -> Result<()> {
        let left_cube = Cuboid {
            center: [-1, 0],
            extents: [3, 3],
        };
        let right_cube = Cuboid {
            center: [1, 0],
            extents: [3, 3],
        };
        let overlap = left_cube
            .bitmap()
            .intersect(vec![right_cube.bitmap()], |a, b| a & b)?;

        expect_that!(overlap.within([0, 0]), is_true());
        expect_that!(overlap.generate([0, 0]), is_true());
        expect_that!(overlap.within([-2, 0]), is_false());
        expect_that!(overlap.generate([-2, 0]), is_false());
        expect_that!(overlap.within([2, 0]), is_false());
        expect_that!(
            overlap.bounding_box(),
            eq(BoundingBox::Bounded([
                DimensionBound::Range(-2, 2),
                DimensionBound::Range(-3, 3),
            ]))
        );
        Ok(())
    }

    #[googletest::test]
    fn empty_or_disjoint_intersection_has_empty_bounds() -> Result<()> {
        let empty = Intersect::<Cuboid<f64, 2>, _, f64, 2>::new(vec![], |a: (), ()| a)?;
        expect_that!(empty.bounding_box(), matches_pattern!(BoundingBox::Empty));
        expect_that!(empty.within([0.0, 0.0]), is_false());
        expect_that!(empty.distance([0.0, 0.0]).is_infinite(), is_true());

        let a = Cuboid {
            center: [-5.0_f64, 0.0],
            extents: [2.0, 2.0],
        };
        let b = Cuboid {
            center: [5.0_f64, 0.0],
            extents: [2.0, 2.0],
        };
        let disjoint = a.intersect(vec![b], |x: (), ()| x)?;
        expect_that!(
            disjoint.bounding_box(),
            matches_pattern!(BoundingBox::Empty)
        );
        expect_that!(disjoint.within([0.0, 0.0]), is_false());
        expect_that!(disjoint.distance([0.0, 0.0]), near(3.0, 1e-12));
        expect_that!(disjoint.signed_distance([0.0, 0.0]), near(3.0, 1e-12));

        let c1 = Cuboid {
            center: [-1.0_f64, 0.0],
            extents: [3.0, 3.0],
        };
        let c2 = Cuboid {
            center: [1.0_f64, 0.0],
            extents: [3.0, 3.0],
        };
        let overlap = c1.intersect(vec![c2], |x: (), ()| x)?;
        expect_that!(overlap.signed_distance([0.0, 0.0]), near(-2.0, 1e-12));
        expect_that!(overlap.signed_distance([2.0, 0.0]), near(0.0, 1e-12));
        Ok(())
    }
}
