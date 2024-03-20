mod bitmap;
mod bound_to;
mod distance_map;
mod map;
//mod repeat;
mod scale;
mod translate;
mod union;
mod with_bounding_box;

pub use bitmap::Bitmap;
pub use bound_to::BoundTo;
pub use distance_map::DistanceMap;
pub use map::{Map, MapInput};
//pub use repeat::Repeat;
pub use scale::Scale;
pub use translate::Translate;
pub use union::{Error as UnionError, Union};
pub use with_bounding_box::WithBoundingBox;
