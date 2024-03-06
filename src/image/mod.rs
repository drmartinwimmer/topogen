use crate::Generator;
use image::Pixel;

pub trait Image<T>: Generator<[T; 2]> {
    fn save_image(
        self,
        path: &str,
        start: [T; 2],
        size: [T; 2],
        step: [T; 2],
    ) -> Result<(), image::ImageError>;
}

macro_rules! impl_image {
    ($t:ty) => {
        impl<C, G> Image<$t> for G
        where
            G: Generator<[$t; 2], Output = C>,
            C: Pixel + image::PixelWithColorType,
            [<C as image::Pixel>::Subpixel]: image::EncodableLayout,
        {
            fn save_image(
                self,
                path: &str,
                start: [$t; 2],
                end: [$t; 2],
                step: [$t; 2],
            ) -> Result<(), image::ImageError> {
                let size = [
                    ((end[0] - start[0]) / step[0]) as u32,
                    ((end[1] - start[1]) / step[1]) as u32,
                ];
                let mut image = image::ImageBuffer::<C, Vec<C::Subpixel>>::new(size[0], size[1]);
                for ix in 0..size[0] {
                    let gx = start[0] + ix as $t * step[0];
                    for iy in 0..size[1] {
                        let gy = start[1] + iy as $t * step[1];
                        image.put_pixel(ix, iy, self.generate([gx, gy]));
                    }
                }
                image.save(path)
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
        image
            .save_image(path.to_str().unwrap(), [0, 0], [3, 3], [1, 1])
            .unwrap();

        let saved_image = image::open(path).unwrap();
        assert_eq!(saved_image.width(), 3);
        assert_eq!(saved_image.height(), 3);
    }
}
