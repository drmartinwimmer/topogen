use num_traits::Float;

use crate::traits::{Center, Coordinate, Euclidean, Generator};

pub struct Sin<T, const D: usize>
where
    T: Coordinate,
{
    center: [T; D],
}

impl<T, const D: usize> Sin<T, D>
where
    T: Coordinate,
{
    pub fn new(center: [T; D]) -> Self {
        Self { center }
    }
}

impl<T, const D: usize> Euclidean<T, D> for Sin<T, D> where T: Coordinate {}

impl<T, const D: usize> Center<T, D> for Sin<T, D>
where
    T: Coordinate,
{
    fn center(&self) -> [T; D] {
        self.center
    }
}

impl<T, const D: usize> Generator<[T; D]> for Sin<T, D>
where
    T: Coordinate + Float,
{
    type Output = T;

    fn generate(&self, point: [T; D]) -> Self::Output {
        self.center_distance(point).sin()
    }
}

#[cfg(test)]
mod tests {
    use super::Sin;
    use crate::traits::{Euclidean, Generator};
    use std::f64::consts::{FRAC_2_PI, SQRT_2};

    #[test]
    fn generates_sin() {
        let sin = Sin::new([0.0, 0.0]).scale([FRAC_2_PI, FRAC_2_PI]);
        assert_eq!(sin.generate([0.0, 0.0]), 0.0);
        assert_eq!(sin.generate([1.0, 0.0]), 1.0);
        assert_eq!(sin.generate([SQRT_2 / 2.0, SQRT_2 / 2.0]), 1.0);
    }
}
