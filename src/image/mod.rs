#[cfg(feature = "palette")]
pub mod palette;

use num_traits::AsPrimitive;

use crate::shapes::{BoundingBox, DimensionBound};
use crate::traits::{Bounded, Coordinate, Generator};
use std::fmt::Display;

pub trait Image<T: Display>: Generator<[T; 2]> {
    fn save_image(self, path: &str, step: [T; 2]) -> Result<(), Error<T>>;
}

pub trait IntoPixel {
    type Pixel;
    fn into_pixel(self) -> Self::Pixel;
}

impl<T> IntoPixel for image::Rgb<T> {
    type Pixel = image::Rgb<T>;
    fn into_pixel(self) -> Self::Pixel {
        self
    }
}

impl<T> IntoPixel for image::Rgba<T> {
    type Pixel = image::Rgba<T>;
    fn into_pixel(self) -> Self::Pixel {
        self
    }
}

impl<T> IntoPixel for image::Luma<T> {
    type Pixel = image::Luma<T>;
    fn into_pixel(self) -> Self::Pixel {
        self
    }
}

impl<T> IntoPixel for image::LumaA<T> {
    type Pixel = image::LumaA<T>;
    fn into_pixel(self) -> Self::Pixel {
        self
    }
}

impl<T, C, G> Image<T> for G
where
    T: Coordinate + Default + num_traits::One + AsPrimitive<u32>,
    u32: AsPrimitive<T>,
    G: Generator<[T; 2], Output = C> + Bounded<T, 2>,
    C: IntoPixel,
    C::Pixel: image::PixelWithColorType,
    [<C::Pixel as image::Pixel>::Subpixel]: image::EncodableLayout,
{
    fn save_image(self, path: &str, step: [T; 2]) -> Result<(), Error<T>> {
        let bounds = match self.bounding_box() {
            BoundingBox::Empty => return Err(Error::EmptyBoundingBox),
            BoundingBox::Unbounded => return Err(Error::UnboundedBoundingBox),
            BoundingBox::Bounded(bounds) => bounds,
        };
        let (mut min, mut max) = ([T::default(); 2], [T::default(); 2]);
        for i in 0..2 {
            (min[i], max[i]) = match bounds[i] {
                DimensionBound::Range(min, max) => (min, max),
                DimensionBound::Empty => return Err(Error::EmptyBoundingBox),
                _ => return Err(Error::UnboundedBoundingBox),
            };
        }
        let steps = [((max[0] - min[0]) / step[0]), ((max[1] - min[1]) / step[1])];
        let size = [steps[0] as T * step[0], steps[1] as T * step[1]];
        let two: T = T::one() + T::one();
        min = [
            min[0] + (max[0] - min[0] - size[0]) / two,
            min[1] + (max[1] - min[1] - size[1]) / two,
        ];
        let mut image =
            image::ImageBuffer::<C::Pixel, Vec<<C::Pixel as image::Pixel>::Subpixel>>::new(
                steps[0].as_(),
                steps[1].as_(),
            );
        for ix in 0..steps[0].as_() {
            let gx = min[0] + ix.as_() * step[0];
            for iy in 0..steps[1].as_() {
                let gy = min[1] + iy.as_() * step[1];
                image.put_pixel(ix, iy, self.generate([gx, gy]).into_pixel());
            }
        }
        image.save(path)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error<T: Display> {
    #[error("Image Error {0:?}")]
    ImageError(#[from] image::ImageError),
    #[error("Computing bounds failed")]
    BoundingBoxError(#[from] crate::shapes::BoundingBoxError<T>),
    #[error("Bounding box is empty")]
    EmptyBoundingBox,
    #[error("Bounding box is unbounded in at least one direction")]
    UnboundedBoundingBox,
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;

    use super::Image;
    use crate::{
        shapes::Cuboid,
        traits::{Bounded, Generator},
    };

    #[test]
    fn saves_image() {
        let cuboid = Cuboid {
            extents: [2, 2],
            center: [0, 0],
        };
        let image = cuboid.bitmap().map(|b| match b {
            true => image::Rgb([255_u8, 255, 255]),
            false => image::Rgb([0_u8, 0, 0]),
        });
        let output_dir = TempDir::new("generative_map_saves_image_test").unwrap();
        let path = output_dir.path().join("test.png");
        image.save_image(path.to_str().unwrap(), [1, 1]).unwrap();

        let saved_image = image::open(path).unwrap();
        assert_eq!(saved_image.width(), 4);
        assert_eq!(saved_image.height(), 4);
    }
}
