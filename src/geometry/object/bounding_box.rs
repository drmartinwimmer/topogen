#[derive(Clone, Debug)]
pub enum BoundingBox<T: Clone, const D: usize> {
    Unbounded,
    Empty,
    Bounded(Bound<T, D>),
    PartiallyBounded(PartialBound<T, D>),
}

impl<T, const D: usize> BoundingBox<T, D>
where
    T: Clone + Copy,
    Bound<T, D>: BoundT<D, T = T>,
    PartialBound<T, D>: BoundT<D, T = T>,
{
    pub fn within(&self, point: [T; D]) -> bool {
        match self {
            BoundingBox::Unbounded => true,
            BoundingBox::Empty => false,
            BoundingBox::Bounded(bound) => bound.within(point),
            BoundingBox::PartiallyBounded(bound) => bound.within(point),
        }
    }

    pub fn for_union(&self, other: &Self) -> Self {
        match (self, other) {
            (BoundingBox::Unbounded, _) | (_, BoundingBox::Unbounded) => BoundingBox::Unbounded,
            (BoundingBox::Empty, other) | (other, BoundingBox::Empty) => other.clone(),
            (BoundingBox::Bounded(b1), BoundingBox::Bounded(b2)) => {
                BoundingBox::Bounded(b1.for_union(b2))
            }
            (BoundingBox::PartiallyBounded(b1), BoundingBox::PartiallyBounded(b2)) => {
                BoundingBox::PartiallyBounded(b1.for_union(b2))
            }
            (BoundingBox::Bounded(bounded), BoundingBox::PartiallyBounded(partially_bounded))
            | (BoundingBox::PartiallyBounded(partially_bounded), BoundingBox::Bounded(bounded)) => {
                BoundingBox::PartiallyBounded(partially_bounded.for_union(&bounded.into()))
            }
        }
    }

    pub fn for_intersection(&self, other: &Self) -> Self {
        match (self, other) {
            (BoundingBox::Unbounded, other) | (other, BoundingBox::Unbounded) => other.clone(),
            (BoundingBox::Empty, _) | (_, BoundingBox::Empty) => BoundingBox::Empty,
            (BoundingBox::Bounded(b1), BoundingBox::Bounded(b2)) => match b1.for_intersection(b2) {
                Some(b) => BoundingBox::Bounded(b),
                None => BoundingBox::Empty,
            },
            (BoundingBox::PartiallyBounded(b1), BoundingBox::PartiallyBounded(b2)) => {
                match b1.for_intersection(b2) {
                    Some(b) => BoundingBox::PartiallyBounded(b),
                    None => BoundingBox::Empty,
                }
            }
            (BoundingBox::Bounded(bounded), BoundingBox::PartiallyBounded(partially_bounded))
            | (BoundingBox::PartiallyBounded(partially_bounded), BoundingBox::Bounded(bounded)) => {
                match partially_bounded.for_intersection(&bounded.into()) {
                    Some(b) => BoundingBox::PartiallyBounded(b),
                    None => BoundingBox::Empty,
                }
            }
        }
    }

    pub fn remove_lower_bound(self, dimension: usize) -> Self {
        match self {
            BoundingBox::Unbounded => BoundingBox::Unbounded,
            BoundingBox::Empty => BoundingBox::Empty,
            BoundingBox::Bounded(bound) => {
                BoundingBox::PartiallyBounded(bound.remove_lower_bound(dimension))
            }
            BoundingBox::PartiallyBounded(bound) => {
                BoundingBox::PartiallyBounded(bound.remove_lower_bound(dimension))
            }
        }
    }

    pub fn remove_upper_bound(self, dimension: usize) -> Self {
        match self {
            BoundingBox::Unbounded => BoundingBox::Unbounded,
            BoundingBox::Empty => BoundingBox::Empty,
            BoundingBox::Bounded(bound) => {
                BoundingBox::PartiallyBounded(bound.remove_upper_bound(dimension))
            }
            BoundingBox::PartiallyBounded(bound) => {
                BoundingBox::PartiallyBounded(bound.remove_upper_bound(dimension))
            }
        }
    }
}

pub trait BoundT<const D: usize>: Sized {
    type T;

    fn within(&self, point: [Self::T; D]) -> bool;
    fn for_union(&self, other: &Self) -> Self;
    fn for_intersection(&self, other: &Self) -> Option<Self>;
    fn remove_lower_bound(self, dimension: usize) -> PartialBound<Self::T, D>;
    fn remove_upper_bound(self, dimension: usize) -> PartialBound<Self::T, D>;

    #[allow(clippy::type_complexity)]
    fn bounds(&self) -> Result<([Self::T; D], [Self::T; D]), Error>;
}

#[derive(Clone, Debug)]
pub struct Bound<T, const D: usize> {
    min: [T; D],
    max: [T; D],
}

macro_rules! impl_bound_boundt {
    ($t:ty) => {
        impl<const D: usize> BoundT<D> for Bound<$t, D> {
            type T = $t;

            fn within(&self, point: [Self::T; D]) -> bool {
                point
                    .into_iter()
                    .zip(self.min.iter().zip(self.max.iter()))
                    .all(|(p, (min, max))| p >= *min && p < *max)
            }

            fn for_union(&self, other: &Self) -> Self {
                let (mut min, mut max) = (self.min.clone(), self.max.clone());
                for i in 0..D {
                    min[i] = min[i].min(other.min[i]);
                    max[i] = max[i].max(other.max[i]);
                }
                Self { min, max }
            }

            fn for_intersection(&self, other: &Self) -> Option<Self> {
                let (mut min, mut max) = (self.min.clone(), self.max.clone());
                for i in 0..D {
                    min[i] = min[i].max(other.min[i]);
                    max[i] = max[i].min(other.max[i]);
                }
                if min
                    .into_iter()
                    .zip(max.into_iter())
                    .all(|(min, max)| min < max)
                {
                    Some(Self { min, max })
                } else {
                    None
                }
            }

            fn remove_lower_bound(self, dimension: usize) -> PartialBound<Self::T, D> {
                let partial_bound: PartialBound<Self::T, D> = self.into();
                partial_bound.remove_lower_bound(dimension)
            }

            fn remove_upper_bound(self, dimension: usize) -> PartialBound<Self::T, D> {
                let partial_bound: PartialBound<Self::T, D> = self.into();
                partial_bound.remove_upper_bound(dimension)
            }

            fn bounds(&self) -> Result<([Self::T; D], [Self::T; D]), Error> {
                Ok((self.min.clone(), self.max.clone()))
            }
        }
    };
}

macro_rules! impl_bound_float {
    ($t:ty) => {
        impl<const D: usize> Bound<$t, D> {
            pub fn new(mut offset: [$t; D], mut size: [$t; D]) -> Self {
                for i in 0..D {
                    if size[i] < 0 as $t {
                        offset[i] -= size[i];
                        size[i] = -size[i];
                    }
                }
                let (min, mut max) = (offset, size);
                for i in 0..D {
                    max[i] += offset[i];
                }
                Self { min, max }
            }

            pub fn new_centered(center: [$t; D], mut size: [$t; D]) -> Self {
                size.iter_mut().for_each(|s| *s = s.abs());
                let (mut min, mut max) = (center, size);
                for i in 0..D {
                    min[i] -= max[i] / 2.0;
                    max[i] += min[i];
                }
                Self { min, max }
            }
        }
    };
}

macro_rules! impl_bound_int {
    ($t:ty) => {
        impl<const D: usize> Bound<$t, D> {
            pub fn new(mut offset: [$t; D], mut size: [$t; D]) -> Self {
                for i in 0..D {
                    if size[i] < 0 as $t {
                        offset[i] -= size[i];
                        size[i] = -size[i];
                    }
                }
                let (min, mut max) = (offset, size);
                for i in 0..D {
                    max[i] += offset[i];
                }
                Self { min, max }
            }

            pub fn new_centered(center: [$t; D], mut size: [$t; D]) -> Self {
                size.iter_mut().for_each(|s| {
                    *s = s.abs();
                    // Round up if uneven
                    *s += *s % 2;
                });
                let (mut min, mut max) = (center, size);
                for i in 0..D {
                    min[i] -= max[i] / 2;
                    max[i] += min[i];
                }
                Self { min, max }
            }
        }
    };
}

macro_rules! impl_bound_uint {
    ($t:ty) => {
        impl<const D: usize> Bound<$t, D> {
            pub fn new(offset: [$t; D], size: [$t; D]) -> Self {
                let (min, mut max) = (offset, size);
                for i in 0..D {
                    max[i] += offset[i];
                }
                Self { min, max }
            }

            pub fn new_centered(center: [$t; D], mut size: [$t; D]) -> Self {
                size.iter_mut().for_each(|s| {
                    // Round up if uneven
                    *s += *s % 2;
                });
                let (mut min, mut max) = (center, size);
                for i in 0..D {
                    min[i] -= max[i] / 2;
                    max[i] += min[i];
                }
                Self { min, max }
            }
        }
    };
}

macro_rules! impl_bound_float_all {
    ($($t:ty),+) => {$(
        impl_bound_boundt!($t);
        impl_bound_float!($t);
    )+};
}

macro_rules! impl_bound_int_all {
    ($($t:ty),+) => {$(
        impl_bound_boundt!($t);
        impl_bound_int!($t);
    )+};
}

macro_rules! impl_bound_uint_all {
    ($($t:ty),+) => {$(
        impl_bound_boundt!($t);
        impl_bound_uint!($t);
    )+};
}

impl_bound_float_all!(f32, f64);
impl_bound_int_all!(i8, i16, i32, i64, i128, isize);
impl_bound_uint_all!(u8, u16, u32, u64, u128, usize);

#[derive(Clone, Debug)]
pub struct PartialBound<T, const D: usize> {
    min: [Option<T>; D],
    max: [Option<T>; D],
}

impl<T: Copy, const D: usize> From<Bound<T, D>> for PartialBound<T, D> {
    fn from(value: Bound<T, D>) -> Self {
        let (mut min, mut max) = ([None; D], [None; D]);
        for i in 0..D {
            min[i] = Some(value.min[i]);
            max[i] = Some(value.max[i]);
        }
        Self { min, max }
    }
}

impl<T: Copy, const D: usize> From<&Bound<T, D>> for PartialBound<T, D> {
    fn from(value: &Bound<T, D>) -> Self {
        let (mut min, mut max) = ([None; D], [None; D]);
        for i in 0..D {
            min[i] = Some(value.min[i]);
            max[i] = Some(value.max[i]);
        }
        Self { min, max }
    }
}

impl<T, const D: usize> TryFrom<PartialBound<T, D>> for Bound<T, D>
where
    [T; D]: Default,
    T: Clone + Copy,
{
    type Error = Error;

    fn try_from(value: PartialBound<T, D>) -> Result<Self, Error> {
        let (mut new_min, mut new_max) = (<[T; D]>::default(), <[T; D]>::default());
        for i in 0..D {
            match (&value.min[i], &value.max[i]) {
                (Some(min), Some(max)) => {
                    new_min[i] = *min;
                    new_max[i] = *max;
                }
                _ => return Err(Error::Unbounded),
            }
        }
        Ok(Self {
            min: new_min,
            max: new_max,
        })
    }
}

impl<T, const D: usize> TryFrom<&PartialBound<T, D>> for Bound<T, D>
where
    T: Default + Clone + Copy,
{
    type Error = Error;

    fn try_from(value: &PartialBound<T, D>) -> Result<Self, Error> {
        let (mut new_min, mut new_max) = ([<T>::default(); D], [<T>::default(); D]);
        for i in 0..D {
            match (&value.min[i], &value.max[i]) {
                (Some(min), Some(max)) => {
                    new_min[i] = *min;
                    new_max[i] = *max;
                }
                _ => return Err(Error::Unbounded),
            }
        }
        Ok(Self {
            min: new_min,
            max: new_max,
        })
    }
}

macro_rules! impl_partial_bound {
    ($t:ty) => {
        impl<const D: usize> BoundT<D> for PartialBound<$t, D> {
            type T = $t;

            fn within(&self, point: [Self::T; D]) -> bool {
                point
                    .into_iter()
                    .zip(self.min.iter().zip(self.max.iter()))
                    .all(|(p, (min, max))| match (*min, *max) {
                        (Some(min), Some(max)) => p >= min && p < max,
                        (Some(min), None) => p >= min,
                        (None, Some(max)) => p < max,
                        (None, None) => true,
                    })
            }

            fn for_union(&self, other: &Self) -> Self {
                let (mut min, mut max) = ([None; D], [None; D]);
                for i in 0..D {
                    min[i] = match (self.min[i], other.min[i]) {
                        (None, _) | (_, None) => None,
                        (Some(m1), Some(m2)) => Some(m1.min(m2)),
                    };
                    max[i] = match (self.max[i], other.max[i]) {
                        (None, _) | (_, None) => None,
                        (Some(m1), Some(m2)) => Some(m1.max(m2)),
                    };
                }
                Self { min, max }
            }

            fn for_intersection(&self, other: &Self) -> Option<Self> {
                let (mut min, mut max) = ([None; D], [None; D]);
                for i in 0..D {
                    min[i] = match (self.min[i], other.min[i]) {
                        (Some(m1), Some(m2)) => Some(m1.max(m2)),
                        (None, m) | (m, None) => m,
                    };
                    max[i] = match (self.max[i], other.max[i]) {
                        (Some(m1), Some(m2)) => Some(m1.min(m2)),
                        (None, m) | (m, None) => m,
                    };
                }
                if min
                    .into_iter()
                    .zip(max.into_iter())
                    .filter_map(|m| match m {
                        (Some(min), Some(max)) => Some((min, max)),
                        _ => None,
                    })
                    .all(|(min, max)| min < max)
                {
                    Some(Self { min, max })
                } else {
                    None
                }
            }

            fn remove_lower_bound(mut self, dimension: usize) -> Self {
                self.min[dimension] = None;
                self
            }

            fn remove_upper_bound(mut self, dimension: usize) -> Self {
                self.max[dimension] = None;
                self
            }

            fn bounds(&self) -> Result<([Self::T; D], [Self::T; D]), Error> {
                let bound = Bound::<$t, D>::try_from(self)?;
                bound.bounds()
            }
        }

        impl<const D: usize> PartialBound<$t, D> {
            pub fn is_unbounded(&self) -> bool {
                self.min
                    .into_iter()
                    .zip(self.max.into_iter())
                    .all(|(min, max)| min == None && max == None)
            }
        }
    };
}

macro_rules! impl_partial_bound_all {
    ($($t:ty),+) => {$(
        impl_partial_bound!($t);
    )+};
}

impl_partial_bound_all!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("An unbounded bound was used in an operation that requires a bounded bound")]
    Unbounded,
}

#[cfg(test)]
mod tests {
    use super::{Bound, BoundT, BoundingBox, Error, PartialBound};

    #[test]
    fn new_bound_works() {
        let bound = Bound::<i32, 2>::new([15, 15], [10, 10]);
        assert!(bound.within([15, 15]));
        assert!(bound.within([24, 24]));
        assert!(!bound.within([25, 15]));
        assert!(!bound.within([15, 25]));
        assert!(!bound.within([14, 15]));
    }

    #[test]
    fn partial_bound_works() {
        let partial_bound: PartialBound<i32, 2> = Bound::<i32, 2>::new([15, 15], [10, 10]).into();
        assert!(partial_bound.within([15, 15]));
        assert!(!partial_bound.within([25, 15]));
    }

    #[test]
    fn try_from_partial_bound_works() {
        let partial_bound: PartialBound<i32, 2> = Bound::<i32, 2>::new([15, 15], [10, 10]).into();
        let _: Bound<i32, 2> = partial_bound.try_into().unwrap();
    }

    #[test]
    fn try_from_partial_bound_fails_on_unbounded() {
        let partial_bound: PartialBound<i32, 2> = Bound::<i32, 2>::new([15, 15], [10, 10]).into();
        let partial_bound = partial_bound.remove_lower_bound(0);
        assert!(matches!(
            Bound::<i32, 2>::try_from(&partial_bound),
            Err(Error::Unbounded)
        ));
    }

    #[test]
    fn for_union_works_for_bounds() {
        let bound = Bound::<i32, 2>::new([15, 15], [10, 10]);
        let bound2 = Bound::<i32, 2>::new([10, 10], [10, 10]);
        let union = bound.for_union(&bound2);
        assert!(union.within([10, 10]));
        assert!(union.within([24, 24]));
        assert!(!union.within([25, 15]));
        assert!(!union.within([9, 15]));
    }

    #[test]
    fn for_union_works_for_partial_bounds() {
        let bound: PartialBound<i32, 2> = Bound::<i32, 2>::new([15, 15], [10, 10]).into();
        let bound2: PartialBound<i32, 2> = Bound::<i32, 2>::new([10, 10], [10, 10]).into();
        let union = bound.for_union(&bound2);
        assert!(union.within([10, 10]));
        assert!(union.within([24, 24]));
        assert!(!union.within([25, 15]));
        assert!(!union.within([9, 15]));
    }

    #[test]
    fn bounding_box_intersection_works() {
        let bound = BoundingBox::Bounded(Bound::<i32, 2>::new([15, 15], [10, 10]));
        let bound2 = BoundingBox::Bounded(Bound::<i32, 2>::new([10, 10], [10, 10]));
        let inter = bound.for_intersection(&bound2);
        assert!(!inter.within([10, 10]));
        assert!(inter.within([15, 15]));
        assert!(!inter.within([20, 19]));
    }
}
