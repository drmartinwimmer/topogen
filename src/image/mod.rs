#[cfg(feature = "palette")]
pub mod palette;

use crate::geometry::WithBoundingBox;
use crate::Generator;

pub trait Image<T>: Generator<[T; 2]> {
    fn save_image(self, path: &str, step: [T; 2]) -> Result<(), Error>;
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

macro_rules! impl_image {
    ($t:ty) => {
        impl<C, G> Image<$t> for G
        where
            G: Generator<[$t; 2], Output = C> + WithBoundingBox<$t, 2>,
            C: IntoPixel,
            C::Pixel: image::PixelWithColorType,
            [<C::Pixel as image::Pixel>::Subpixel]: image::EncodableLayout,
        {
            fn save_image(self, path: &str, step: [$t; 2]) -> Result<(), Error> {
                let (start, end) = self.bounding_box().bounds()?;
                let size = [
                    ((end[0] - start[0]) / step[0]) as u32,
                    ((end[1] - start[1]) / step[1]) as u32,
                ];
                let mut image = image::ImageBuffer::<
                    C::Pixel,
                    Vec<<C::Pixel as image::Pixel>::Subpixel>,
                >::new(size[0], size[1]);
                for ix in 0..size[0] {
                    let gx = start[0] + ix as $t * step[0];
                    for iy in 0..size[1] {
                        let gy = start[1] + iy as $t * step[1];
                        image.put_pixel(ix, iy, self.generate([gx, gy]).into_pixel());
                    }
                }
                image.save(path)?;
                Ok(())
            }
        }
    };
}

macro_rules! impl_image_all {
    ($($t:ty),+) => {$(
        impl_image!($t);
    )+};
}

impl_image_all!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Image Error {0:?}")]
    ImageError(#[from] image::ImageError),
    #[error("Image is unbounded")]
    BoundingBoxError(#[from] crate::geometry::object::bounding_box::Error),
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;

    use super::Image;
    use crate::{
        geometry::{object::Cuboid, GeometricObject},
        Generator,
    };

    #[test]
    fn saves_image() {
        let cuboid = Cuboid {
            sides: [2, 2],
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
        assert_eq!(saved_image.width(), 2);
        assert_eq!(saved_image.height(), 2);
    }
}
