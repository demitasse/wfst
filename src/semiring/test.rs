// Author: Daniel van Niekerk <dvn.demitasse@gmail.com>
//
// Copyright 2016 The Department of Arts and Culture of the Government
// of South Africa
//
// See the "LICENCE" file for information on usage and redistribution
// of this file.
////////////////////////////////////////////////////////////////////////////////

//! This module implements the generation of random weights for
//! testing purposes. See the source file `test_semiring.rs` for
//! simple examples of intended use.

extern crate rand;
use self::rand::{Rng, StdRng};

use super::*;

//////////////////////////DEFINE HOW DIFFERENT WEIGHTS CAN BE CREATED FROM U32
pub trait RandomWeight: Weight {
    fn from_u32(u32) -> Self;
}

impl<T: Float<T>> RandomWeight for TropicalWeight<T> {
    fn from_u32(n: u32) -> Self {
        Self::new(Some(T::from_u32(n)))
    }
}

impl<T: Float<T>> RandomWeight for LogWeight<T> {
    fn from_u32(n: u32) -> Self {
        Self::new(Some(T::from_u32(n)))
    }
}

impl<T: Float<T>> RandomWeight for MinmaxWeight<T> {
    fn from_u32(n: u32) -> Self {
        Self::new(Some(T::from_u32(n)))
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
