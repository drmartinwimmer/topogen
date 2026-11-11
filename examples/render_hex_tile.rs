extern crate image;
extern crate topogen;

use palette::{Hsl, Hsla, RgbHue};
use std::path::Path;
use topogen::{
    image::Image,
    ops::Union,
    shapes::{BoundingBox, Sphere},
    traits::{Bounded, Euclidean, EuclideanVec, Generator, Shape},
};

struct Terrain {
    color: Hsl,
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
            if let Some(slot) = weights.get_mut(terrain_id) {
                *slot = weight;
            }
            weights
        })
        .bound_to(Sphere {
            radius: WIDTH * 1.5,
            center: position,
        })
}

fn render_dominant_terrain<const N: usize>(weights: &[f32; N], terrains: &[Terrain; N]) -> Hsla {
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
    max_id.and_then(|id| terrains.get(id)).map_or_else(
        || Hsla::new(0.0, 0.0, 0.0, 0.0),
        |terrain| Hsla {
            color: terrain.color,
            alpha: 1.0,
        },
    )
}

fn main() {
    let terrains = [
        // Desert
        Terrain {
            color: Hsl::new(RgbHue::new(43.), 0.42, 0.77),
        },
        // Grassland
        Terrain {
            color: Hsl::new(RgbHue::new(62.), 0.34, 0.65),
        },
        // Mountain
        Terrain {
            color: Hsl::new(RgbHue::new(27.), 0.20, 0.67),
        },
        // Water
        Terrain {
            color: Hsl::new(RgbHue::new(176.), 0.19, 0.52),
        },
        // Sea
        Terrain {
            color: Hsl::new(RgbHue::new(192.), 0.31, 0.38),
        },
    ];

    let center = tile::<5>([0.0, 0.0], 1);
    let nw = tile([-WIDTH / 2.0, -HEIGHT], 1);
    let w = tile([-WIDTH, 0.0], 1);
    let sw = tile([-WIDTH / 2.0, HEIGHT], 0);
    let se = tile([WIDTH / 2.0, HEIGHT], 3);
    let e = tile([WIDTH, 0.0], 3);
    let ne = tile([WIDTH / 2.0, -HEIGHT], 3);

    let union = Union::new(vec![center, nw, w, sw, se, e, ne], EuclideanVec::add)
        .expect("failed to construct terrain union")
        .with_bounding_box(BoundingBox::new_centered([0.0, 0.0], [WIDTH, HEIGHT]));

    let image = union.map(|weights| render_dominant_terrain(&weights, &terrains));
    image
        .save_image(Path::new("hex_tile.png"), [1.0, 1.0])
        .expect("failed to save hex tile image");
}
