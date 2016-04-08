extern crate rand;
use rand::{Rng, StdRng};

extern crate semiring;
use semiring::*;

//This bounds the random integers generated: [0, K)
const K: u32 = 5;

pub trait RandomWeightGenerator<T> {
    fn genweight(&mut self, allow_zero: bool) -> T;
}

// pub trait FloatWeight {
//     fn from_u32(u32) -> Self;
// }

impl<T: Float<T>> RandomWeightGenerator<TropicalWeight<T>> for StdRng {
    fn genweight(&mut self, allow_zero: bool) -> TropicalWeight<T> {
        let n = self.gen_range(0, K + allow_zero as u32);
        if allow_zero && n == K {
            TropicalWeight::zero()
        } else {
            TropicalWeight::new(Some(T::from_u32(n)))
        }
    }
}

