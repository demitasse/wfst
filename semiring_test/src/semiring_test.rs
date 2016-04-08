extern crate rand;
use rand::{Rng, StdRng};

extern crate semiring;
use semiring::*;

//This bounds the random integers generated: [0, K)
const K: u32 = 5;

// //Will no try to move the from_u32() implementation here
// trait RandomWeight: Weight {

// }

pub trait RandomWeightGenerator {
    fn genweight<T: Weight>(&mut self, allow_zero: bool) -> T;
}

impl RandomWeightGenerator for StdRng {
    fn genweight<T: Weight>(&mut self, allow_zero: bool) -> T {
        let n = self.gen_range(0, K + allow_zero as u32);
        if allow_zero && n == K {
            T::zero()
        } else {
            T::from_u32(n)
        }
    }
}

