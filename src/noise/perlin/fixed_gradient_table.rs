use num_traits::Float;
use std::marker::PhantomData;

use super::GradientTable;

pub struct FixedGradientTable<O: Float, const D: usize> {
    _output: PhantomData<[O; D]>,
}

impl<O: Float, const D: usize> Default for FixedGradientTable<O, D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<O: Float, const D: usize> FixedGradientTable<O, D> {
    pub fn new() -> Self {
        Self {
            _output: PhantomData,
        }
    }
}

impl<O: Float> GradientTable<O, 1> for FixedGradientTable<O, 1> {
    fn get(&self, entry: u8) -> [O; 1] {
        // Bitmask `entry & 1` restricts the value to `0..=1`.
        #[expect(clippy::unreachable, reason = "entry & 1 is bounded to 0..=1")]
        match entry & 1 {
            0 => [O::one()],
            1 => [-O::one()],
            2..=u8::MAX => unreachable!(),
        }
    }
}

impl<O: Float> GradientTable<O, 2> for FixedGradientTable<O, 2> {
    fn get(&self, entry: u8) -> [O; 2] {
        // Bitmask `entry & 3` restricts the value to `0..=3`.
        #[expect(clippy::unreachable, reason = "entry & 3 is bounded to 0..=3")]
        match entry & 3 {
            0 => [O::one(), O::one()],
            1 => [-O::one(), O::one()],
            2 => [O::one(), -O::one()],
            3 => [-O::one(), -O::one()],
            4..=u8::MAX => unreachable!(),
        }
    }
}

impl<O: Float> GradientTable<O, 3> for FixedGradientTable<O, 3> {
    fn get(&self, entry: u8) -> [O; 3] {
        // Bitmask `entry & 15` restricts the value to `0..=15`.
        #[expect(clippy::unreachable, reason = "entry & 15 is bounded to 0..=15")]
        match entry & 15 {
            0 | 12 => [O::one(), O::one(), O::zero()],
            1 | 13 => [-O::one(), O::one(), O::zero()],
            2 => [O::one(), -O::one(), O::zero()],
            3 => [-O::one(), -O::one(), O::zero()],
            4 => [O::one(), O::zero(), O::one()],
            5 => [-O::one(), O::zero(), O::one()],
            6 => [O::one(), O::zero(), -O::one()],
            7 => [-O::one(), O::zero(), -O::one()],
            8 => [O::zero(), O::one(), O::one()],
            9 | 14 => [O::zero(), -O::one(), O::one()],
            10 => [O::zero(), O::one(), -O::one()],
            11 | 15 => [O::zero(), -O::one(), -O::one()],
            16..=u8::MAX => unreachable!(),
        }
    }
}
