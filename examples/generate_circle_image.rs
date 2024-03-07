extern crate generative_map;
extern crate image;

use generative_map::{
    geometry::{object::Sphere, GeometricObject},
    image::Image,
    Generator,
};

fn main() {
    let circle = Sphere {
        radius: 1.0,
        center: [0.0, 0.0],
    };
    let red_circle = circle.bitmap().map(|b| match b {
        true => image::Rgb([255_u8, 0, 0]),
        false => image::Rgb([0_u8, 0, 0]),
    });
    red_circle.save_image("circle.png", [0.1, 0.1]).unwrap();
}
