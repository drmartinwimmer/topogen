pub trait Coordinate:
    Copy
    + std::fmt::Display
    + std::fmt::Debug
    + PartialOrd
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::iter::Sum
{
}

impl<T> Coordinate for T where
    T: Copy
        + std::fmt::Display
        + std::fmt::Debug
        + PartialOrd
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::iter::Sum
{
}

pub trait Euclidean<T: Coordinate, const D: usize>: Sized {}
