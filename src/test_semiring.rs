// Author: Daniel van Niekerk <dvn.demitasse@gmail.com>
//
// Copyright 2016 The Department of Arts and Culture of the Government
// of South Africa
//
// See the "LICENCE" file for information on usage and redistribution
// of this file.

// This file contains portions of code ported from OpenFst
// (http://www.openfst.org) under the following licence and
// attribution:
//
// """
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Copyright 2005-2010 Google, Inc.
// Author: riley@google.com (Michael Riley)
// """

extern crate rand;
use rand::{SeedableRng, StdRng};

use std::fmt::Debug;

extern crate wfst;
use wfst::semiring::test::{RandomWeight, RandomWeightGenerator};
use wfst::semiring::*;
use wfst::semiring::floatweight::*;

extern crate serde;
use serde::{Serialize, Deserialize};
extern crate bincode;
use bincode::{serialize, deserialize, Infinite};

// Tests (plus, times, zero, one) defines a commutative semiring.
fn test_semiring1<T: Weight + Semiring + Commutative>(w1: &T, w2: &T, w3: &T) {
    // Checks that the operations are closed.
    assert!(w1.plus(w2).is_member());
    assert!(w1.times(w2).is_member());

    // Checks that the operations are associative.
    assert!(w1.plus(&w2.plus(w3)).approx_eq(&w1.plus(w2).plus(w3), None));
    assert!(w1.times(&w2.times(w3)).approx_eq(&w1.times(w2).times(w3), None));

    // Checks the identity elements.
    assert!(w1.plus(&T::zero()).eq(w1));
    assert!(T::zero().plus(w1).eq(w1));
    assert!(w1.times(&T::one()).eq(w1));
    assert!(T::one().times(w1).eq(w1));

    // Check the no weight element.
    assert!(!T::none().is_member());
    assert!(!w1.plus(&T::none()).is_member());
    assert!(!T::none().plus(w1).is_member());
    assert!(!w1.times(&T::none()).is_member());
    assert!(!T::none().times(w1).is_member());

    // Checks that the operations commute.
    assert!(w1.plus(w2).approx_eq(&w2.plus(w1), None));
    assert!(w1.times(w2).approx_eq(&w2.times(w1), None));

    // Checks zero() is the annihilator.
    assert!(w1.times(&T::zero()).eq(&T::zero()));
    assert!(T::zero().times(w1).eq(&T::zero()));
    
    // Check power(w, 0) is one()
    assert!(power(w1, 0).eq(&T::one()));

    // Check power(w, 1) is w
    assert!(power(w1, 1).eq(w1));

    // Check power(w, 3) is times(w, times(w, w))
    assert!(power(w1, 3).eq(&w1.times(&w1.times(w1))));

    // Checks distributivity.
    assert!(w1.times(&w2.plus(w3)).approx_eq(&w1.times(w2).plus(&w1.times(w3)), None)); //LeftSemiring
    assert!(w1.plus(w2).times(w3).approx_eq(&w1.times(w3).plus(&w2.times(w3)), None)); //RightSemiring
}

fn test_semiring2<T: Weight + Idempotent + Path>(w1: &T, w2: &T) {
    assert!(w1.plus(w1).eq(w1)); //Idempotent
    assert!(w1.plus(w2).eq(w1) || w1.plus(w2).eq(w2)); //Path
}

// Tests division operations
fn test_division<T: Weight + Semiring + Commutative>(w1: &T, w2: &T) {
    let p = w1.times(w2);

    //LeftSemiring
    let d = p.divide(w1, Some(DivideType::Divleft));
    if d.is_member() {
        assert!(p.approx_eq(&w1.times(&d), None));
    }
    assert!(!w1.divide(&T::none(), Some(DivideType::Divleft)).is_member());
    assert!(!T::none().divide(w1, Some(DivideType::Divleft)).is_member());

    //RightSemiring
    let d = p.divide(w2, Some(DivideType::Divright));
    if d.is_member() {
        assert!(p.approx_eq(&d.times(w2), None));
    }
    assert!(!w1.divide(&T::none(), Some(DivideType::Divright)).is_member());
    assert!(!T::none().divide(w1, Some(DivideType::Divright)).is_member());

    //Commutative
    let d = p.divide(w1, Some(DivideType::Divright));
    if d.is_member() {
        assert!(p.approx_eq(&d.times(w1), None));
    }
}

fn test_reverse<T: Weight>(w1: &T, w2: &T) {
    let rw1 = w1.reverse();
    let rw2 = w2.reverse();

    assert!(rw1.reverse().eq(w1)); 
    assert!(w1.plus(w2).reverse().eq(&rw1.plus(&rw2)));
    assert!(w1.times(w2).reverse().eq(&rw1.times(&rw2)));
}

// Tests eq() is an equivalence relation.
fn test_equality<T: Weight>(w1: &T, w2: &T, w3: &T) {
    // Checks reflexivity.
    assert!(w1.eq(w1));

    // Checks symmetry.
    assert!(w1.eq(w2) == w2.eq(w1));

    // Checks transitivity.
    if w1.eq(w2) && w2.eq(w3) {
        assert!(w1.eq(w3));
    }
}

fn test_io<T: Weight + Serialize + Deserialize>(w1: &T) {
    let encoded = serialize(w1, Infinite).unwrap();
    let ww = deserialize(&encoded).unwrap();
    assert!(w1.eq(&ww));
}

fn test_clone<T: Weight>(w1: &T) {
    let ww: T = w1.clone();
    assert!(w1.eq(&ww));
}

// Test a variety of identities and properties that must hold for the
// Weight implementation to be well-defined.  Note in the tests we use
// approx_eq() rather than == where the weights might be inexact.
fn test12<T: RandomWeight + Semiring + Commutative + Idempotent + Path + Debug + Serialize + Deserialize>(rng: &mut StdRng,
                                                                                                          n_iterations: u32,
                                                                                                          test_div: bool) {
    for _ in 0..n_iterations {
        let w1 = rng.genweight::<T>(true);
        let w2 = rng.genweight::<T>(true);
        let w3 = rng.genweight::<T>(true);
        test_semiring1(&w1, &w2, &w3);
        test_semiring2(&w1, &w2);
        if test_div {
            test_division(&w1, &w2);
        }
        test_reverse(&w1, &w2);
        test_equality(&w1, &w2, &w3);
        test_io(&w1);
        test_clone(&w1);
    }
}

fn test1<T: RandomWeight + Semiring + Commutative + Debug + Serialize + Deserialize>(rng: &mut StdRng,
                                                                                     n_iterations: u32,
                                                                                     test_div: bool) {
    for _ in 0..n_iterations {
        let w1 = rng.genweight::<T>(true);
        let w2 = rng.genweight::<T>(true);
        let w3 = rng.genweight::<T>(true);
        test_semiring1(&w1, &w2, &w3);
        if test_div {
            test_division(&w1, &w2);
        }
        test_reverse(&w1, &w2);
        test_equality(&w1, &w2, &w3);
        test_io(&w1);
        test_clone(&w1);
    }
}
    

fn main() {
    let seed: &[_] = &[7,7,7];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    let n_iterations = 1000000;

    println!("Seed: {:?}", seed);
    println!("============================================================\n");
    //f32
    println!("Testing `TropicalWeight<f32>`:");
    test12::<TropicalWeight<f32>>(&mut rng, n_iterations, true);
    println!("============================================================\n");
    println!("Testing `LogWeight<f32>`:");
    test1::<LogWeight<f32>>(&mut rng, n_iterations, true);
    println!("============================================================\n");
    println!("Testing `MinmaxWeight<f32>`:");
    test12::<MinmaxWeight<f32>>(&mut rng, n_iterations, true);
    println!("============================================================\n");
    //f64
    println!("Testing `TropicalWeight<f64>`:");
    test12::<TropicalWeight<f64>>(&mut rng, n_iterations, true);
    println!("============================================================\n");
    println!("Testing `LogWeight<f64>`:");
    test1::<LogWeight<f64>>(&mut rng, n_iterations, true);
    println!("============================================================\n");
    println!("Testing `MinmaxWeight<f64>`:");
    test12::<MinmaxWeight<f64>>(&mut rng, n_iterations, true);
    println!("============================================================\n");
    
    println!("TESTS PASSED");
}
