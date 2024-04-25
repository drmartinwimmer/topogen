use num_traits::Float;
use std::marker::PhantomData;

use super::GradientTable;

pub struct FixedGradientTable<O: Float, const D: usize> {
    _output: PhantomData<[O; D]>,
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
        match entry & 1 {
            0 => [O::one()],
            1 => [-O::one()],
            _ => unreachable!(),
        }
    }
}

impl<O: Float> GradientTable<O, 2> for FixedGradientTable<O, 2> {
    fn get(&self, entry: u8) -> [O; 2] {
        match entry & 3 {
            0 => [O::one(), O::one()],
            1 => [-O::one(), O::one()],
            2 => [O::one(), -O::one()],
            3 => [-O::one(), -O::one()],
            _ => unreachable!(),
        }
    }
}

impl<O: Float> GradientTable<O, 3> for FixedGradientTable<O, 3> {
    fn get(&self, entry: u8) -> [O; 3] {
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
            _ => unreachable!(),
        }
    }
}
