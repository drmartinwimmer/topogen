use crate::Generator;

pub trait Bitmap<T, const D: usize>: Generator<[T; D], Output = bool> {}
