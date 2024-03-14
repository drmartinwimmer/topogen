use super::IntoPixel;
use palette::{
    encoding,
    rgb::{Rgb, Rgba},
    stimulus::IntoStimulus,
    Hsl, Hsla, IntoColor, Srgb, Srgba,
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
    use crate::geometry::object::Cuboid;
    use crate::image::Image;
    use crate::traits::{Bounded, Generator};
    use palette::{Hsl, RgbHue};
    use tempdir::TempDir;

    #[test]
    fn saves_image_with_palette() {
        let cuboid = Cuboid {
            half_edges: [2, 2],
            center: [0, 0],
        };
        let image = cuboid.bitmap().map(|b| match b {
            true => Hsl::new(RgbHue::new(0.3), 0.5_f32, 1.0),
            false => Hsl::new(RgbHue::new(0.3), 0.5, 0.0),
        });
        let output_dir = TempDir::new("generative_map_saves_image_with_palette_test").unwrap();
        let path = output_dir.path().join("test.png");
        image.save_image(path.to_str().unwrap(), [1, 1]).unwrap();

        let saved_image = image::open(path).unwrap();
        assert_eq!(saved_image.width(), 2);
        assert_eq!(saved_image.height(), 2);
    }
}
