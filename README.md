# topogen

Deterministic, composable N-dimensional spatial field and map generator for Rust.

`topogen` lets you build procedural maps, terrains, textures, and spatial fields by composing mathematical [`Generator`](src/traits/generator.rs) functions, geometric [`Shape`](src/traits/shape.rs)s, signed distance maps, and deterministic noise—evaluating coordinates on demand without pre-allocating grids.

## Features

- **Composable spatial generators**: Map, clamp, scale, translate, bound, and combine N-dimensional fields (`Map`, `MapInput`, `Clamp`, `Scale`, `Translate`, `Union`, `Fbm`, `Bitmap`, `DistanceMap`).
- **Geometric primitives & spatial bounds**: `Sphere`, `Cuboid`, `BoundingBox`, and `Point` with `Bounded` spatial culling for efficient composition of localized features (such as tiles or biomes).
- **Deterministic noise** (`noise` / `noise_fn` features): Seeded `ImprovedPerlin`, `Worley`, and `PermutationTable` generators built for reproducible results, plus optional `noise` crate integration.
- **Image & color rendering** (`image` / `palette` features): Render 2D bounded generators directly to images.
- **Derive macros** (`topogen_derive`): `#[derive(Generator, Euclidean, Bounded, Center, Shape)]` for ergonomic wrapper types.

## Example

```rust
use std::path::Path;
use topogen::{
    image::Image,
    shapes::Sphere,
    traits::{Bounded, Generator},
};

fn main() {
    let circle = Sphere {
        radius: 1.0,
        center: [0.0, 0.0],
    };
    let red_circle = circle.bitmap().map(|inside| match inside {
        true => image::Rgb([255_u8, 0, 0]),
        false => image::Rgb([0_u8, 0, 0]),
    });
    red_circle
        .save_image(Path::new("circle.png"), [0.1, 0.1])
        .unwrap();
}
```

See [`examples/render_hex_tile.rs`](examples/render_hex_tile.rs) for an example blending multiple terrain types using distance maps and `Union`.

## Cargo Features

| Feature | Description |
| --- | --- |
| `noise` | Deterministic noise generators (`ImprovedPerlin`, `Worley`, `PermutationTable`) |
| `noise_fn` | Interoperability with `noise::NoiseFn` (enables `noise`) |
| `image` | Image rendering support via the `image` crate |
| `palette` | Color space support for image rendering via `palette` (enables `image`) |
