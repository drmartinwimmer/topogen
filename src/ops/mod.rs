mod bitmap;
mod map;
//mod repeat;
mod union;

pub use bitmap::Bitmap;
pub use map::{Map, MapInput};
//pub use repeat::Repeat;
pub use union::{Error as UnionError, Union};
