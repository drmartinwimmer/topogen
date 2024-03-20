use crate::traits::{Bounded, Center, Coordinate, Euclidean, Generator};

#[derive(Debug)]
pub struct Translate<E, T, const D: usize> {
    wrapped: E,
    offset: [T; D],
}

impl<E, T, const D: usize> Translate<E, T, D>
where
    E: Euclidean<T, D>,
    T: Coordinate,
{
    pub fn new(wrapped: E, offset: [T; D]) -> Self {
        Self {
            wrapped,
            offset,
        }
    }
}

impl<E, T, const D: usize> Euclidean<T, D> for Translate<E, T, D>
where
    E: Euclidean<T, D>,
    T: Coordinate,
{
}

impl<B, T, const D: usize> Bounded<T, D> for Translate<B, T, D>
where
    B: Bounded<T, D>,
    T: Coordinate,
{
    fn within(&self, mut input: [T; D]) -> bool {
        input
            .iter_mut()
            .zip(self.offset.iter())
            .for_each(|(i, s)| *i -= *s);
        self.wrapped.within(input)
    }

    fn bounding_box(&self) -> crate::shapes::BoundingBox<T, D> {
        self.wrapped.bounding_box().translate(self.offset)
    }
}

impl<C, T, const D: usize> Center<T, D> for Translate<C, T, D>
where
    C: Center<T, D>,
    T: Coordinate,
{
    fn center(&self) -> [T; D] {
        let mut center = self.wrapped.center();
        center
            .iter_mut()
            .zip(self.offset.iter())
            .for_each(|(i, s)| *i += *s);
        center
    }
}

impl<G, T, const D: usize> Generator<[T; D]> for Translate<G, T, D>
where
    G: Generator<[T; D]> + Euclidean<T, D>,
    T: Coordinate,
{
    type Output = G::Output;

    fn generate(&self, mut input: [T; D]) -> Self::Output {
        input
            .iter_mut()
            .zip(self.offset.iter())
            .for_each(|(i, s)| *i -= *s);
        self.wrapped.generate(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shapes::Cuboid,
        traits::{Bounded, Euclidean, Generator},
    };

    #[test]
    fn translates_generator() {
        let cube_map = Cuboid {
            center: [0, 0],
            extents: [2, 2],
        }
        .bitmap();
        let scaled = cube_map.translate([3, 3]);
        assert!(scaled.generate([2, 2]));
        assert!(scaled.generate([4, 4]));
        assert!(!scaled.generate([1, 1]));
    }
}
