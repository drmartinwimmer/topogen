#[cfg(feature = "palette")]
pub mod palette;

use image::{EncodableLayout, ImageBuffer, ImageError, Pixel, PixelWithColorType};
use num_traits::AsPrimitive;

use crate::shapes::{BoundingBox, BoundingBoxError, DimensionBound};
use crate::traits::{Bounded, Coordinate, Generator};
use std::fmt::Display;
use std::path::Path;

pub trait Image<T: Display>: Generator<[T; 2]> {
    fn save_image(self, path: &Path, step: [T; 2]) -> Result<(), ImageRenderError<T>>;
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
    C::Pixel: PixelWithColorType,
    [<C::Pixel as Pixel>::Subpixel]: EncodableLayout,
{
    fn save_image(self, path: &Path, step: [T; 2]) -> Result<(), ImageRenderError<T>> {
        let bounds = match self.bounding_box() {
            BoundingBox::Empty => return Err(ImageRenderError::EmptyBoundingBox),
            BoundingBox::Unbounded => return Err(ImageRenderError::UnboundedBoundingBox),
            BoundingBox::Bounded(bounds) => bounds,
        };
        let extract_range = |bound: DimensionBound<T>| match bound {
            DimensionBound::Range(min, max) => Ok((min, max)),
            DimensionBound::Empty => Err(ImageRenderError::EmptyBoundingBox),
            DimensionBound::Unbounded | DimensionBound::Min(_) | DimensionBound::Max(_) => {
                Err(ImageRenderError::UnboundedBoundingBox)
            }
        };
        let [bound_x, bound_y] = bounds;
        let (min_x, max_x) = extract_range(bound_x)?;
        let (min_y, max_y) = extract_range(bound_y)?;
        let mut min = [min_x, min_y];
        let max = [max_x, max_y];
        let steps = [((max[0] - min[0]) / step[0]), ((max[1] - min[1]) / step[1])];
        let size = [steps[0] as T * step[0], steps[1] as T * step[1]];
        let two: T = T::one() + T::one();
        min = [
            min[0] + (max[0] - min[0] - size[0]) / two,
            min[1] + (max[1] - min[1] - size[1]) / two,
        ];
        let mut image = ImageBuffer::<C::Pixel, Vec<<C::Pixel as Pixel>::Subpixel>>::new(
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
pub enum ImageRenderError<T: Display> {
    #[error("Image Error {0:?}")]
    ImageError(#[from] ImageError),
    #[error("Computing bounds failed")]
    BoundingBoxError(#[from] BoundingBoxError<T>),
    #[error("Bounding box is empty")]
    EmptyBoundingBox,
    #[error("Bounding box is unbounded in at least one direction")]
    UnboundedBoundingBox,
}

#[cfg(test)]
mod tests {
    use super::Image;
    use crate::{
        shapes::Cuboid,
        traits::{Bounded, Generator},
    };
    use googletest::prelude::*;
    use tempfile::Builder;

    #[googletest::test]
    fn saves_cuboid_image() -> Result<()> {
        let cuboid = Cuboid {
            extents: [2, 2],
            center: [0, 0],
        };
        let image = cuboid.bitmap().map(|b| match b {
            true => image::Rgb([255_u8, 255, 255]),
            false => image::Rgb([0_u8, 0, 0]),
        });
        let tempfile = Builder::new().suffix(".png").tempfile()?;
        image.save_image(tempfile.path(), [1, 1])?;

        let saved_image = image::open(tempfile.path())?;
        expect_that!(saved_image.width(), eq(4));
        expect_that!(saved_image.height(), eq(4));
        Ok(())
    }
}
