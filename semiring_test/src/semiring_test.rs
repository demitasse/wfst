extern crate rand;
use rand::{Rng, StdRng};

extern crate semiring;
use semiring::*;

//////////////////////////DEFINE HOW DIFFERENT WEIGHTS CAN BE CREATED FROM U32
pub trait RandomWeight: Weight {
    fn from_u32(u32) -> Self;
}

impl<T: Float<T>> RandomWeight for TropicalWeight<T> {
    fn from_u32(n: u32) -> TropicalWeight<T> {
        TropicalWeight::new(Some(T::from_u32(n)))
    }
}

impl<T: Float<T>> RandomWeight for LogWeight<T> {
    fn from_u32(n: u32) -> LogWeight<T> {
        LogWeight::new(Some(T::from_u32(n)))
    }
}

impl<T: Float<T>> RandomWeight for MinmaxWeight<T> {
    fn from_u32(n: u32) -> MinmaxWeight<T> {
        MinmaxWeight::new(Some(T::from_u32(n)))
    }
}


//////////////////////////////DEFINE HOW RANDOM WEIGHTS CAN BE CREATED USING RNG

//This bounds the random integers generated: [0, K)
const K: u32 = 5;

pub trait RandomWeightGenerator {
    fn genweight<T: RandomWeight>(&mut self, allow_zero: bool) -> T;
}

impl RandomWeightGenerator for StdRng {
    fn genweight<T: RandomWeight>(&mut self, allow_zero: bool) -> T {
        let n = self.gen_range(0, K + allow_zero as u32);
        if allow_zero && n == K {
            T::zero()
        } else {
            T::from_u32(n)
        }
    }
}
