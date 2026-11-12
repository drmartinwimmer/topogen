use super::IntoPixel;
use palette::{
    Hsl, Hsla, IntoColor, Srgb, Srgba, encoding,
    rgb::{Rgb, Rgba},
    stimulus::IntoStimulus,
};

impl<T: IntoStimulus<u8>> IntoPixel for Rgb<encoding::Srgb, T> {
    type Pixel = image::Rgb<u8>;

    fn into_pixel(self) -> Self::Pixel {
        image::Rgb::<u8>(self.into_format::<u8>().into())
    }
}

impl<T: IntoStimulus<u8>> IntoPixel for Rgba<encoding::Srgb, T> {
    type Pixel = image::Rgba<u8>;

    fn into_pixel(self) -> Self::Pixel {
        image::Rgba::<u8>(self.into_format::<u8, u8>().into())
    }
}

impl IntoPixel for Hsl<encoding::Srgb, f32> {
    type Pixel = image::Rgb<u8>;

    fn into_pixel(self) -> Self::Pixel {
        let rgb: Srgb = self.into_color();
        rgb.into_pixel()
    }
}

impl IntoPixel for Hsla<encoding::Srgb, f32> {
    type Pixel = image::Rgba<u8>;

    fn into_pixel(self) -> Self::Pixel {
        let rgba: Srgba = self.into_color();
        rgba.into_pixel()
    }
}

#[cfg(test)]
mod tests {
    use crate::image::Image;
    use crate::shapes::Cuboid;
    use crate::traits::{Bounded, Generator};
    use googletest::prelude::*;
    use palette::{Hsl, RgbHue};
    use tempfile::Builder;

    #[googletest::test]
    fn saves_image_with_palette() -> Result<()> {
        let cuboid = Cuboid {
            extents: [2, 2],
            center: [0, 0],
        };
        let image = cuboid.bitmap().map(|b| match b {
            true => Hsl::new(RgbHue::new(0.3), 0.5_f32, 1.0),
            false => Hsl::new(RgbHue::new(0.3), 0.5, 0.0),
        });
        let tempfile = Builder::new().suffix(".png").tempfile()?;
        image.save_image(tempfile.path(), [1, 1])?;

        let saved_image = image::open(tempfile.path())?;
        expect_that!(saved_image.width(), eq(4));
        expect_that!(saved_image.height(), eq(4));
        Ok(())
    }
}
