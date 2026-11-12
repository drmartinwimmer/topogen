use crate::traits::Coordinate;
use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BoundingBox<T: Copy + std::fmt::Debug, const D: usize> {
    Unbounded,
    #[default]
    Empty,
    Bounded([DimensionBound<T>; D]),
}

impl<T, const D: usize> BoundingBox<T, D>
where
    T: Coordinate,
{
    pub fn new_bounded(b: [DimensionBound<T>; D]) -> Result<Self, BoundingBoxError<T>> {
        Self::validated(Self::Bounded(b))
    }

    pub fn new_offset(offset: [T; D], size: [T; D]) -> Self {
        let mut bounding_box = [DimensionBound::Unbounded; D];
        for (bound, (o, s)) in bounding_box.iter_mut().zip(offset.into_iter().zip(size)) {
            *bound = DimensionBound::Range(o, o + s);
        }
        Self::Bounded(bounding_box)
    }

    pub fn new_centered(center: [T; D], extents: [T; D]) -> Self {
        let mut bounding_box = [DimensionBound::Unbounded; D];
        for (bound, (c, e)) in bounding_box.iter_mut().zip(center.into_iter().zip(extents)) {
            *bound = DimensionBound::Range(c - e, c + e);
        }
        Self::Bounded(bounding_box)
    }

    fn validated(self) -> Result<Self, BoundingBoxError<T>> {
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

    pub fn for_union(&self, other: &Self) -> Result<Self, BoundingBoxError<T>> {
        match (self, other) {
            (BoundingBox::Unbounded, _) | (_, BoundingBox::Unbounded) => Ok(BoundingBox::Unbounded),
            (BoundingBox::Empty, other) | (other, BoundingBox::Empty) => Ok(*other),
            (BoundingBox::Bounded(b1), BoundingBox::Bounded(b2)) => {
                let mut union = [DimensionBound::Empty; D];
                for (out, (d1, d2)) in union.iter_mut().zip(b1.iter().zip(b2.iter())) {
                    *out = d1.for_union(*d2)?;
                }
                BoundingBox::new_bounded(union)
            }
        }
    }

    pub fn for_intersect(&self, other: &Self) -> Result<Self, BoundingBoxError<T>> {
        match (self, other) {
            (BoundingBox::Unbounded, other) | (other, BoundingBox::Unbounded) => Ok(*other),
            (BoundingBox::Empty, _) | (_, BoundingBox::Empty) => Ok(BoundingBox::Empty),
            (BoundingBox::Bounded(b1), BoundingBox::Bounded(b2)) => {
                let mut intersect = [DimensionBound::Unbounded; D];
                for (out, (d1, d2)) in intersect.iter_mut().zip(b1.iter().zip(b2.iter())) {
                    *out = d1.for_intersect(*d2)?;
                }
                BoundingBox::new_bounded(intersect)
            }
        }
    }

    pub fn scale(self, factor: [T; D]) -> Self {
        let zero: T = std::iter::empty().sum();
        let is_non_invertible = |f: T| matches!(f.partial_cmp(&zero), Some(Ordering::Equal) | None);
        match self {
            BoundingBox::Unbounded => {
                if factor.into_iter().any(is_non_invertible) {
                    BoundingBox::Empty
                } else {
                    BoundingBox::Unbounded
                }
            }
            BoundingBox::Empty => BoundingBox::Empty,
            BoundingBox::Bounded(mut b) => {
                b.iter_mut().zip(factor).for_each(|(b, f)| {
                    *b = b.scale(f);
                });
                if b.iter().all(|b| matches!(b, DimensionBound::Unbounded)) {
                    BoundingBox::Unbounded
                } else if b.iter().any(|b| matches!(b, DimensionBound::Empty)) {
                    BoundingBox::Empty
                } else {
                    BoundingBox::Bounded(b)
                }
            }
        }
    }

    pub fn translate(self, offset: [T; D]) -> Self {
        match self {
            BoundingBox::Unbounded => BoundingBox::Unbounded,
            BoundingBox::Empty => BoundingBox::Empty,
            BoundingBox::Bounded(mut b) => {
                b.iter_mut().zip(offset).for_each(|(b, o)| {
                    *b = b.translate(o);
                });
                BoundingBox::Bounded(b)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DimensionBound<T> {
    #[default]
    Empty,
    Unbounded,
    Min(T),
    Max(T),
    Range(T, T),
}

fn min<T>(a: T, b: T) -> Result<T, BoundingBoxError<T>>
where
    T: Coordinate,
{
    match a.partial_cmp(&b) {
        Some(Ordering::Less) | Some(Ordering::Equal) => Ok(a),
        Some(Ordering::Greater) => Ok(b),
        None => Err(BoundingBoxError::PartialCmpFailed(a, b)),
    }
}

fn max<T>(a: T, b: T) -> Result<T, BoundingBoxError<T>>
where
    T: Coordinate,
{
    match a.partial_cmp(&b) {
        Some(Ordering::Greater) | Some(Ordering::Equal) => Ok(a),
        Some(Ordering::Less) => Ok(b),
        None => Err(BoundingBoxError::PartialCmpFailed(a, b)),
    }
}

fn range<T>(min: T, max: T) -> Result<DimensionBound<T>, BoundingBoxError<T>>
where
    T: Coordinate,
{
    match min.partial_cmp(&max) {
        Some(Ordering::Less) => Ok(DimensionBound::Range(min, max)),
        Some(Ordering::Greater) | Some(Ordering::Equal) => Ok(DimensionBound::Empty),
        None => Err(BoundingBoxError::PartialCmpFailed(min, max)),
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

    fn for_union(self, other: Self) -> Result<Self, BoundingBoxError<T>> {
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

    fn for_intersect(self, other: Self) -> Result<Self, BoundingBoxError<T>> {
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

    fn scale(self, factor: T) -> Self {
        let zero: T = std::iter::empty().sum();
        match factor.partial_cmp(&zero) {
            Some(Ordering::Greater) => match self {
                DimensionBound::Empty => DimensionBound::Empty,
                DimensionBound::Unbounded => DimensionBound::Unbounded,
                DimensionBound::Min(min) => DimensionBound::Min(min * factor),
                DimensionBound::Max(max) => DimensionBound::Max(max * factor),
                DimensionBound::Range(min, max) => {
                    DimensionBound::Range(min * factor, max * factor)
                }
            },
            Some(Ordering::Less) => match self {
                DimensionBound::Empty => DimensionBound::Empty,
                DimensionBound::Unbounded => DimensionBound::Unbounded,
                DimensionBound::Min(min) => DimensionBound::Max(min * factor),
                DimensionBound::Max(max) => DimensionBound::Min(max * factor),
                DimensionBound::Range(min, max) => {
                    DimensionBound::Range(max * factor, min * factor)
                }
            },
            Some(Ordering::Equal) | None => DimensionBound::Empty,
        }
    }

    fn translate(self, offset: T) -> Self {
        match self {
            DimensionBound::Empty => DimensionBound::Empty,
            DimensionBound::Unbounded => DimensionBound::Unbounded,
            DimensionBound::Min(min) => DimensionBound::Min(min + offset),
            DimensionBound::Max(max) => DimensionBound::Max(max + offset),
            DimensionBound::Range(min, max) => DimensionBound::Range(min + offset, max + offset),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BoundingBoxError<T: Display> {
    #[error("Partial comparison failed: {0}, {1}")]
    PartialCmpFailed(T, T),
}

#[cfg(test)]
mod tests {
    use super::{BoundingBox, DimensionBound};
    use googletest::prelude::*;

    #[googletest::test]
    fn new_bound_works() {
        let bound = BoundingBox::new_offset([15, 15], [10, 10]);
        expect_that!(bound.within([16, 16]), is_true());
        expect_that!(bound.within([24, 24]), is_true());
        expect_that!(bound.within([25, 16]), is_false());
        expect_that!(bound.within([15, 25]), is_false());
        expect_that!(bound.within([15, 16]), is_false());
    }

    #[googletest::test]
    fn for_union_works() -> Result<()> {
        let bound = BoundingBox::new_offset([15, 15], [10, 10]);
        let bound2 = BoundingBox::new_offset([10, 10], [10, 10]);
        let union = bound.for_union(&bound2)?;
        expect_that!(union.within([11, 11]), is_true());
        expect_that!(union.within([24, 24]), is_true());
        expect_that!(union.within([25, 15]), is_false());
        expect_that!(union.within([10, 15]), is_false());
        Ok(())
    }

    #[googletest::test]
    fn for_intersection_works() -> Result<()> {
        let bound = BoundingBox::new_offset([15, 15], [10, 10]);
        let bound2 = BoundingBox::new_offset([10, 10], [10, 10]);
        let inter = bound.for_intersect(&bound2)?;
        expect_that!(inter.within([11, 11]), is_false());
        expect_that!(inter.within([16, 16]), is_true());
        expect_that!(inter.within([20, 19]), is_false());
        Ok(())
    }

    #[googletest::test]
    fn scales_bounds_with_negative_factor() -> Result<()> {
        let bound =
            BoundingBox::new_bounded([DimensionBound::Range(2.0, 5.0), DimensionBound::Min(1.0)])?;
        let mirrored = bound.scale([-2.0, -3.0]);
        expect_that!(mirrored.within([-6.0, -4.0]), is_true());
        expect_that!(mirrored.within([-3.0, -4.0]), is_false());
        expect_that!(mirrored.within([-11.0, -4.0]), is_false());
        expect_that!(mirrored.within([-6.0, -2.0]), is_false());
        Ok(())
    }

    #[googletest::test]
    fn scales_bounds_with_zero_factor_to_empty() {
        let bound = BoundingBox::new_offset([1.0, 1.0], [4.0, 4.0]);
        let collapsed = bound.scale([0.0, 2.0]);
        expect_that!(collapsed.within([0.0, 4.0]), is_false());
        expect_that!(matches!(collapsed, BoundingBox::Empty), is_true());
    }
}
