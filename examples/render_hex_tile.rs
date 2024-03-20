extern crate generative_map;
extern crate image;

use generative_map::{
    image::Image,
    ops::Union,
    shapes::{BoundingBox, Sphere},
    traits::{Bounded, Euclidean, Generator, Shape},
};
use palette::Hsla;

struct Terrain {
    color: palette::Hsl,
}

const WIDTH: f32 = 312.;
const HEIGHT: f32 = 270.;
const FADE: f32 = 0.98;

fn tile<const N: usize>(
    position: [f32; 2],
    terrain_id: usize,
) -> impl Generator<[f32; 2], Output = [f32; N]> + Bounded<f32, 2>
where
    [f32; N]: Default,
{
    let circle = Sphere {
        radius: WIDTH / 2.0 - 10.0,
        center: position,
    };
    let distance_map = circle.distance_map();
    distance_map
        .map(move |d| {
            let weight = (1.0 * FADE.powf(d)).max(0.0);
            let mut weights = [0.0; N];
            weights[terrain_id] = weight;
            weights
        })
        .bound_to(Sphere {
            radius: WIDTH * 1.5,
            center: position,
        })
}

fn combine_terrain_weights<const N: usize>(mut a: [f32; N], b: [f32; N]) -> [f32; N] {
    a.iter_mut().zip(b).for_each(|(a, b)| *a += b);
    a
}

fn render_dominant_terrain<const N: usize>(
    weights: &[f32; N],
    terrains: &[Terrain; N],
) -> palette::Hsla {
    let mut first = true;
    let mut max_id: Option<usize> = None;
    let weight_sum: f32 = weights.iter().sum();
    for (i, weight) in weights.iter().enumerate() {
        if first && *weight > 0.0001 || *weight * 2.2 > weight_sum {
            max_id = Some(i);
            // To avoid sharp corners when 3 (or more) terrains meet, bias towards the terrains in earlier array positions by adding up their weights.
            first = false;
        }
    }
    match max_id {
        Some(id) => Hsla {
            color: terrains[id].color,
            alpha: 1.0,
        },
        None => Hsla::new(0.0, 0.0, 0.0, 0.0),
    }
}

fn main() {
    let terrains = [
        // Desert
        Terrain {
            color: palette::Hsl::new(palette::RgbHue::new(43.), 0.42, 0.77),
        },
        // Grassland
        Terrain {
            color: palette::Hsl::new(palette::RgbHue::new(62.), 0.34, 0.65),
        },
        // Mountain
        Terrain {
            color: palette::Hsl::new(palette::RgbHue::new(27.), 0.20, 0.67),
        },
        // Water
        Terrain {
            color: palette::Hsl::new(palette::RgbHue::new(176.), 0.19, 0.52),
        },
        // Sea
        Terrain {
            color: palette::Hsl::new(palette::RgbHue::new(192.), 0.31, 0.38),
        },
    ];

    let center = tile([0.0, 0.0], 1);
    let nw = tile([-WIDTH / 2.0, -HEIGHT], 1);
    let w = tile([-WIDTH, 0.0], 1);
    let sw = tile([-WIDTH / 2.0, HEIGHT], 0);
    let se = tile([WIDTH / 2.0, HEIGHT], 3);
    let e = tile([WIDTH, 0.0], 3);
    let ne = tile([WIDTH / 2.0, -HEIGHT], 3);

    let union = Union::new(
        vec![center, nw, w, sw, se, e, ne],
        combine_terrain_weights::<5>,
    )
    .unwrap()
    .with_bounding_box(BoundingBox::new_centered([0.0, 0.0], [WIDTH, HEIGHT]));

    let image = union.map(|weights| render_dominant_terrain(&weights, &terrains));
    image.save_image("hex_tile.png", [1.0, 1.0]).unwrap();
}
