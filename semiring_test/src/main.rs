use std::fmt::Debug;

extern crate rand;
extern crate semiring_test;
extern crate semiring;

use rand::{SeedableRng, StdRng};
use semiring_test::{RandomWeight, RandomWeightGenerator};
use semiring::*;

extern crate rustc_serialize;
use rustc_serialize::{Encodable, Decodable};
extern crate bincode;
use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};

// Tests (plus, times, zero, one) defines a commutative semiring.
fn test_semiring<T: Weight>(w1: T, w2: T, w3: T) {
    // Checks that the operations are closed.
    assert!(w1.plus(w2).is_member());
    assert!(w1.times(w2).is_member());

    // Checks that the operations are associative.
    assert!(w1.plus(w2.plus(w3)).approx_eq(w1.plus(w2).plus(w3), None));
    assert!(w1.times(w2.times(w3)).approx_eq(w1.times(w2).times(w3), None));

    // Checks the identity elements.
    assert!(w1.plus(T::zero()).eq(w1));
    assert!(T::zero().plus(w1).eq(w1));
    assert!(w1.times(T::one()).eq(w1));
    assert!(T::one().times(w1).eq(w1));

    // Check the no weight element.
    assert!(!T::none().is_member());
    assert!(!w1.plus(T::none()).is_member());
    assert!(!T::none().plus(w1).is_member());
    assert!(!w1.times(T::none()).is_member());
    assert!(!T::none().times(w1).is_member());

    // Checks that the operations commute.
    assert!(w1.plus(w2).approx_eq(w2.plus(w1), None));
    if T::properties() & COMMUTATIVE != 0 {
        assert!(w1.times(w2).approx_eq(w2.times(w1), None));
    }

    // Checks zero() is the annihilator.
    assert!(w1.times(T::zero()).eq(T::zero()));
    assert!(T::zero().times(w1).eq(T::zero()));
    
    // Check power(w, 0) is one()
    assert!(power(w1, 0).eq(T::one()));

    // Check power(w, 1) is w
    assert!(power(w1, 1).eq(w1));

    // Check power(w, 3) is times(w, times(w, w))
    assert!(power(w1, 3).eq(w1.times(w1.times(w1))));

    // Checks distributivity.
    if T::properties() & LEFT_SEMIRING != 0 {
        assert!(w1.times(w2.plus(w3)).approx_eq(w1.times(w2).plus(w1.times(w3)), None));
    }
    if T::properties() & RIGHT_SEMIRING != 0 {
        assert!(w1.plus(w2).times(w3).approx_eq(w1.times(w3).plus(w2.times(w3)), None));
    }

    if T::properties() & IDEMPOTENT != 0 {
        assert!(w1.plus(w1).eq(w1));
    }

    if T::properties() & PATH != 0 {
        assert!(w1.plus(w2).eq(w1) || w1.plus(w2).eq(w2));
    }

    // Ensure weights form a left or right semiring.
    assert!(T::properties() & (LEFT_SEMIRING | RIGHT_SEMIRING) != 0);

    // Check when Times() is commutative that it is marked as a semiring.
    if T::properties() & COMMUTATIVE != 0 {
        assert!(T::properties() & SEMIRING != 0);
    }    
}

// Tests division operations
fn test_division<T: Weight>(w1: T, w2: T) {
    let p = w1.times(w2);

    if T::properties() & LEFT_SEMIRING != 0 {
        let d = p.divide(w1, Some(DivideType::Divleft));
        if d.is_member() {
            assert!(p.approx_eq(w1.times(d), None));
        }
        assert!(!w1.divide(T::none(), Some(DivideType::Divleft)).is_member());
        assert!(!T::none().divide(w1, Some(DivideType::Divleft)).is_member());
    }

    if T::properties() & RIGHT_SEMIRING != 0 {
        let d = p.divide(w2, Some(DivideType::Divright));
        if d.is_member() {
            assert!(p.approx_eq(d.times(w2), None));
        }
        assert!(!w1.divide(T::none(), Some(DivideType::Divright)).is_member());
        assert!(!T::none().divide(w1, Some(DivideType::Divright)).is_member());
    }

    if T::properties() & COMMUTATIVE != 0 {
        let d = p.divide(w1, Some(DivideType::Divright));
        if d.is_member() {
            assert!(p.approx_eq(d.times(w1), None));
        }
    }
}

fn test_reverse<T: Weight>(w1: T, w2: T) {
    let rw1 = w1.reverse();
    let rw2 = w2.reverse();

    assert!(rw1.reverse().eq(w1)); 
    assert!(w1.plus(w2).reverse().eq(rw1.plus(rw2)));
    assert!(w1.times(w2).reverse().eq(rw1.times(rw2)));
}

// Tests eq() is an equivalence relation.
fn test_equality<T: Weight>(w1: T, w2: T, w3: T) {
    // Checks reflexivity.
    assert!(w1.eq(w1));

    // Checks symmetry.
    assert!(w1.eq(w2) == w2.eq(w1));

    // Checks transitivity.
    if w1.eq(w2) && w2.eq(w3) {
        assert!(w1.eq(w3));
    }
}

fn test_io<T: Weight + Encodable + Decodable>(w1: T) {
    let encoded = encode(&w1, SizeLimit::Infinite).unwrap();
    let ww = decode(&encoded).unwrap();
    assert!(w1.eq(ww));
}

fn test_copy<T: Weight>(w1: T) {
    let ww: T = w1; //This should copy since Weight is Copy
    assert!(w1.eq(ww));
}

// Test a variety of identities and properties that must hold for the
// Weight implementation to be well-defined.  Note in the tests we use
// approx_eq() rather than == where the weights might be inexact.
fn test<T: RandomWeight + Debug + Encodable + Decodable>(rng: &mut StdRng,
                                                         n_iterations: u32,
                                                         test_div: bool) {
    for _ in 0..n_iterations {
        let w1 = rng.genweight::<T>(true);
        let w2 = rng.genweight::<T>(true);
        let w3 = rng.genweight::<T>(true);
        //println!("weight type = {}", T::wtype());
        //println!("w1 = {:?}", w1);
        //println!("w2 = {:?}", w2);
        //println!("w3 = {:?}", w3);
        test_semiring(w1, w2, w3);
        if test_div {
            test_division(w1, w2);
        }
        test_reverse(w1, w2);
        test_equality(w1, w2, w3);
        test_io(w1);
        test_copy(w1);
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
    test::<TropicalWeight<f32>>(&mut rng, n_iterations, true);
    println!("============================================================\n");
    println!("Testing `LogWeight<f32>`:");
    test::<LogWeight<f32>>(&mut rng, n_iterations, true);
    println!("============================================================\n");
    println!("Testing `MinmaxWeight<f32>`:");
    test::<MinmaxWeight<f32>>(&mut rng, n_iterations, true);
    println!("============================================================\n");
    //f64
    println!("Testing `TropicalWeight<f64>`:");
    test::<TropicalWeight<f64>>(&mut rng, n_iterations, true);
    println!("============================================================\n");
    println!("Testing `LogWeight<f64>`:");
    test::<LogWeight<f64>>(&mut rng, n_iterations, true);
    println!("============================================================\n");
    println!("Testing `MinmaxWeight<f64>`:");
    test::<MinmaxWeight<f64>>(&mut rng, n_iterations, true);
    println!("============================================================\n");
    
    println!("TESTS PASSED");
}
