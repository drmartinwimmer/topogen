//! Deterministic, composable N-dimensional spatial field and map generator.
//!
//! `topogen` models procedural maps, signed distance fields, and textures as lazy mathematical
//! functions implementing [`traits::Generator`]. Spatial shapes and fields implement
//! [`traits::Euclidean`], [`traits::Bounded`], [`traits::Center`], and [`traits::Shape`], allowing
//! transformations and CSG-style combinators to be chained without pre-allocating grids.
//!
//! # Example
//!
//! ```rust
//! use topogen::{
//!     shapes::Sphere,
//!     traits::{Euclidean, Generator, Shape},
//! };
//!
//! let ring = Sphere {
//!     center: [0.0_f64, 0.0],
//!     radius: 5.0,
//! }
//! .signed_distance_map()
//! .translate([2.0, -1.0])
//! .map(|d| d.abs() < 0.5);
//!
//! assert!(ring.generate([7.0, -1.0]));
//! assert!(!ring.generate([2.0, -1.0]));
//! ```

extern crate self as topogen;
extern crate topogen_derive;

/// Foundational generator primitives such as constants, borrows, and radial sine waves.
pub mod core;
/// 2D image rendering adapters and pixel conversions.
#[cfg(feature = "image")]
pub mod image;
/// Deterministic procedural noise generators.
#[cfg(feature = "noise")]
pub mod noise;
/// Combinators and spatial operators transforming or combining generators and shapes.
pub mod ops;
/// N-dimensional geometric primitives and axis-aligned bounding boxes.
pub mod shapes;
#[cfg(test)]
mod testing;
/// Core traits defining generators, Euclidean coordinate spaces, bounds, centers, and shapes.
pub mod traits;
