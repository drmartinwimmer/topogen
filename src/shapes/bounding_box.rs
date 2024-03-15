use crate::ops::{Bitmap, Union, UnionError};
use crate::traits::{Bounded, Center, Coordinate, Euclidean, Generator, Shape};
use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum BoundingBox<T: Copy + std::fmt::Debug, const D: usize> {
    Unbounded,
    Empty,
    Bounded([DimensionBound<T>; D]),
}

impl<T, const D: usize> BoundingBox<T, D>
where
    T: Coordinate,
{
    pub fn new_bounded(b: [DimensionBound<T>; D]) -> Result<Self, Error<T>> {
        Self::validated(Self::Bounded(b))
    }

    pub fn new_offset(offset: [T; D], size: [T; D]) -> Self {
        let mut bounding_box = [DimensionBound::Unbounded; D];
        for i in 0..D {
            bounding_box[i] = DimensionBound::Range(offset[i], offset[i] + size[i]);
        }
        Self::Bounded(bounding_box)
    }

    pub fn new_centered(center: [T; D], half_edge: [T; D]) -> Self {
        let mut bounding_box = [DimensionBound::Unbounded; D];
        for i in 0..D {
            bounding_box[i] =
                DimensionBound::Range(center[i] - half_edge[i], center[i] + half_edge[i]);
        }
        Self::Bounded(bounding_box)
    }

    fn validated(self) -> Result<Self, Error<T>> {
        match self {
            BoundingBox::Unbounded => Ok(BoundingBox::Unbounded),
            BoundingBox::Empty => Ok(BoundingBox::Empty),
            BoundingBox::Bounded(mut b) => {
                for bound in b.iter_mut() {
                    if let DimensionBound::Range(min, max) = bound {
                        *bound = range(*min, *max)?
                    }
                }
                Ok(
                    if b.iter().all(|b| matches!(b, DimensionBound::Unbounded)) {
                        BoundingBox::Unbounded
                    } else if b.iter().any(|b| matches!(b, DimensionBound::Empty)) {
                        BoundingBox::Empty
                    } else {
                        BoundingBox::Bounded(b)
                    },
                )
            }
        }
    }

    pub fn within(&self, point: [T; D]) -> bool {
        match self {
            BoundingBox::Unbounded => true,
            BoundingBox::Empty => false,
            BoundingBox::Bounded(bounds) => bounds.iter().zip(point).all(|(b, p)| b.within(p)),
        }
    }

    pub fn for_union(&self, other: &Self) -> Result<Self, Error<T>> {
        match (self, other) {
            (BoundingBox::Unbounded, _) | (_, BoundingBox::Unbounded) => Ok(BoundingBox::Unbounded),
            (BoundingBox::Empty, other) | (other, BoundingBox::Empty) => Ok(other.clone()),
            (BoundingBox::Bounded(b1), BoundingBox::Bounded(b2)) => {
                let mut union = [DimensionBound::Empty; D];
                for i in 0..D {
                    union[i] = b1[i].for_union(b2[i])?;
                }
                BoundingBox::new_bounded(union)
            }
        }
    }

    pub fn for_intersect(&self, other: &Self) -> Result<Self, Error<T>> {
        match (self, other) {
            (BoundingBox::Unbounded, other) | (other, BoundingBox::Unbounded) => Ok(other.clone()),
            (BoundingBox::Empty, _) | (_, BoundingBox::Empty) => Ok(BoundingBox::Empty),
            (BoundingBox::Bounded(b1), BoundingBox::Bounded(b2)) => {
                let mut intersect = [DimensionBound::Unbounded; D];
                for i in 0..D {
                    intersect[i] = b1[i].for_intersect(b2[i])?;
                }
                BoundingBox::new_bounded(intersect)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DimensionBound<T> {
    Empty,
    Unbounded,
    Min(T),
    Max(T),
    Range(T, T),
}

impl<T> Default for DimensionBound<T> {
    fn default() -> Self {
        Self::Empty
    }
}

fn min<T>(a: T, b: T) -> Result<T, Error<T>>
where
    T: Coordinate,
{
    match a.partial_cmp(&b) {
        Some(Ordering::Less) | Some(Ordering::Equal) => Ok(a),
        Some(Ordering::Greater) => Ok(b),
        None => Err(Error::PartialCmpFailed(a, b)),
    }
}

fn max<T>(a: T, b: T) -> Result<T, Error<T>>
where
    T: Coordinate,
{
    match a.partial_cmp(&b) {
        Some(Ordering::Greater) | Some(Ordering::Equal) => Ok(a),
        Some(Ordering::Less) => Ok(b),
        None => Err(Error::PartialCmpFailed(a, b)),
    }
}

fn range<T>(min: T, max: T) -> Result<DimensionBound<T>, Error<T>>
where
    T: Coordinate,
{
    match min.partial_cmp(&max) {
        Some(Ordering::Less) => Ok(DimensionBound::Range(min, max)),
        Some(Ordering::Greater) | Some(Ordering::Equal) => Ok(DimensionBound::Empty),
        None => Err(Error::PartialCmpFailed(min, max)),
    }
}

impl<T> DimensionBound<T>
where
    T: Coordinate,
{
    fn within(&self, point: T) -> bool {
        match self {
            DimensionBound::Empty => false,
            DimensionBound::Unbounded => true,
            DimensionBound::Min(min) => point > *min,
            DimensionBound::Max(max) => point < *max,
            DimensionBound::Range(min, max) => point > *min && point < *max,
        }
    }

    fn for_union(self, other: Self) -> Result<Self, Error<T>> {
        match (self, other) {
            (DimensionBound::Empty, other) | (other, DimensionBound::Empty) => Ok(other),
            (DimensionBound::Unbounded, _)
            | (_, DimensionBound::Unbounded)
            | (DimensionBound::Min(_), DimensionBound::Max(_))
            | (DimensionBound::Max(_), DimensionBound::Min(_)) => Ok(DimensionBound::Unbounded),
            (DimensionBound::Min(min1), DimensionBound::Min(min2))
            | (DimensionBound::Min(min1), DimensionBound::Range(min2, _))
            | (DimensionBound::Range(min1, _), DimensionBound::Min(min2)) => {
                Ok(DimensionBound::Min(min(min1, min2)?))
            }
            (DimensionBound::Max(max1), DimensionBound::Max(max2))
            | (DimensionBound::Max(max1), DimensionBound::Range(_, max2))
            | (DimensionBound::Range(_, max1), DimensionBound::Max(max2)) => {
                Ok(DimensionBound::Max(max(max1, max2)?))
            }
            (DimensionBound::Range(min1, max1), DimensionBound::Range(min2, max2)) => {
                range(min(min1, min2)?, max(max1, max2)?)
            }
        }
    }

    fn for_intersect(self, other: Self) -> Result<Self, Error<T>> {
        match (self, other) {
            (DimensionBound::Empty, _) | (_, DimensionBound::Empty) => Ok(DimensionBound::Empty),
            (DimensionBound::Unbounded, other) | (other, DimensionBound::Unbounded) => Ok(other),
            (DimensionBound::Min(min1), DimensionBound::Min(min2)) => {
                Ok(DimensionBound::Min(max(min1, min2)?))
            }
            (DimensionBound::Max(max1), DimensionBound::Max(max2)) => {
                Ok(DimensionBound::Max(min(max1, max2)?))
            }
            (DimensionBound::Min(min), DimensionBound::Max(max))
            | (DimensionBound::Max(max), DimensionBound::Min(min)) => range(min, max),
            (DimensionBound::Min(min1), DimensionBound::Range(min2, max1))
            | (DimensionBound::Range(min1, max1), DimensionBound::Min(min2)) => {
                range(max(min1, min2)?, max1)
            }
            (DimensionBound::Max(max1), DimensionBound::Range(min1, max2))
            | (DimensionBound::Range(min1, max1), DimensionBound::Max(max2)) => {
                range(min1, min(max1, max2)?)
            }
            (DimensionBound::Range(min1, max1), DimensionBound::Range(min2, max2)) => {
                range(max(min1, min2)?, min(max1, max2)?)
            }
        }
    }
}

pub struct WithBoundingBox<B, T, const D: usize>
where
    B: Bounded<T, D>,
    T: Coordinate,
{
    bounded: B,
    bounding_box: BoundingBox<T, D>,
}

impl<B, T, const D: usize> WithBoundingBox<B, T, D>
where
    B: Bounded<T, D>,
    T: Coordinate,
{
    pub fn new(bounded: B, bounding_box: BoundingBox<T, D>) -> Self {
        Self {
            bounded,
            bounding_box,
        }
    }
}

impl<B, T, const D: usize> Euclidean<T, D> for WithBoundingBox<B, T, D>
where
    B: Bounded<T, D>,
    T: Coordinate,
{
}

impl<B, T, const D: usize> Bounded<T, D> for WithBoundingBox<B, T, D>
where
    B: Bounded<T, D>,
    T: Coordinate,
{
    type Bitmap = Bitmap<T, Self, D>;

    fn bounding_box(&self) -> BoundingBox<T, D> {
        self.bounding_box.clone()
    }

    fn within(&self, point: [T; D]) -> bool {
        self.bounding_box.within(point) && self.bounded.within(point)
    }

    fn union<F>(
        self,
        mut others: Vec<Self>,
        reducer: F,
    ) -> Result<Union<T, Self, F, D>, UnionError<T>> {
        others.push(self);
        Union::new(others, reducer)
    }

    fn bitmap(self) -> Self::Bitmap {
        Bitmap::new(self)
    }
}

impl<C, T, const D: usize> Center<T, D> for WithBoundingBox<C, T, D>
where
    C: Center<T, D> + Bounded<T, D>,
    T: Coordinate,
{
    fn center(&self) -> &[T; D] {
        self.bounded.center()
    }
}

impl<S, T, const D: usize> Shape<T, D> for WithBoundingBox<S, T, D>
where
    S: Shape<T, D>,
    T: Coordinate,
{
    fn distance(&self, point: [T; D]) -> T
    where
        T: num_traits::Float,
    {
        self.bounded.distance(point)
    }
}

impl<G, T, const D: usize> Generator<[T; D]> for WithBoundingBox<G, T, D>
where
    G: Generator<[T; D]> + Bounded<T, D>,
    G::Output: Default,
    T: Coordinate,
{
    type Output = G::Output;

    fn generate(&self, input: [T; D]) -> Self::Output {
        match self.bounding_box.within(input) {
            true => self.bounded.generate(input),
            false => Default::default(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error<T: Display> {
    #[error("Partial comparison failed: {0}, {1}")]
    PartialCmpFailed(T, T),
}

#[cfg(test)]
mod tests {
    use super::BoundingBox;

    #[test]
    fn new_bound_works() {
        let bound = BoundingBox::new_offset([15, 15], [10, 10]);
        assert!(bound.within([16, 16]));
        assert!(bound.within([24, 24]));
        assert!(!bound.within([25, 16]));
        assert!(!bound.within([15, 25]));
        assert!(!bound.within([15, 16]));
    }

    #[test]
    fn for_union_works() {
        let bound = BoundingBox::new_offset([15, 15], [10, 10]);
        let bound2 = BoundingBox::new_offset([10, 10], [10, 10]);
        let union = bound.for_union(&bound2).unwrap();
        assert!(union.within([11, 11]));
        assert!(union.within([24, 24]));
        assert!(!union.within([25, 15]));
        assert!(!union.within([10, 15]));
    }

    #[test]
    fn for_intersection_works() {
        let bound = BoundingBox::new_offset([15, 15], [10, 10]);
        let bound2 = BoundingBox::new_offset([10, 10], [10, 10]);
        let inter = bound.for_intersect(&bound2).unwrap();
        assert!(!inter.within([11, 11]));
        assert!(inter.within([16, 16]));
        assert!(!inter.within([20, 19]));
    }
}
