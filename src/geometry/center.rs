pub trait WithCenter<T, const D: usize>: Sized + Clone {
    fn center(&self) -> &[T; D];
}

pub trait WithCenterDistance<T, const D: usize>: WithCenter<T, D> {
    fn center_distance(&self, point: [T; D]) -> T;
}

macro_rules! impl_with_center_distance {
    ($t:ty, $sqrtt:ty) => {
        impl<C, const D: usize> WithCenterDistance<$t, D> for C
        where
            C: WithCenter<$t, D>,
        {
            fn center_distance(&self, point: [$t; D]) -> $t {
                let distance: $t = point
                    .into_iter()
                    .zip(self.center())
                    .map(|(p, c)| (p - c) * (p - c))
                    .sum();
                (distance as $sqrtt).sqrt() as $t
            }
        }
    };
}

macro_rules! impl_with_center_distance_int {
    ($($t:ty),+) => {$(
        impl_with_center_distance!($t, f64);
    )+};
}

macro_rules! impl_with_center_distance_float {
    ($($t:ty),+) => {$(
        impl_with_center_distance!($t, $t);
    )+};
}

impl_with_center_distance_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_with_center_distance_float!(f32, f64);
