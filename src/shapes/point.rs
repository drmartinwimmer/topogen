use crate::traits::{Center, Coordinate, Euclidean};

pub struct Point<T, const D: usize> {
    pub coordinates: [T; D],
}

impl<T, const D: usize> Point<T, D> {
    pub fn new(coordinates: [T; D]) -> Point<T, D> {
        Point { coordinates }
    }
}

impl<T, const D: usize> Center<T, D> for Point<T, D>
where
    T: Coordinate,
{
    fn center(&self) -> [T; D] {
        self.coordinates
    }
}

impl<T, const D: usize> Euclidean<T, D> for Point<T, D> where T: Coordinate {}
