use std::fmt::Debug;

extern crate rand;
extern crate semiring_test;
extern crate semiring;

use rand::{SeedableRng, StdRng};
use semiring_test::{RandomWeight, RandomWeightGenerator};
use semiring::*;


// Tests (plus, times, zero, one) defines a commutative semiring.
fn test_semiring<T: RandomWeight>(w1: T, w2: T, w3: T) {
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

    


}

// Test a variety of identities and properties that must hold for the
// Weight implementation to be well-defined.  Note in the tests we use
// approx_eq() rather than == where the weights might be inexact.
fn test<T: RandomWeight + Debug>(rng: &mut StdRng,
                                 n_iterations: u32,
                                 test_division: bool) {
    for _ in 0..n_iterations {
        let w1 = rng.genweight::<T>(true);
        let w2 = rng.genweight::<T>(true);
        let w3 = rng.genweight::<T>(true);
        println!("");
        println!("w1 = {:?}", w1);
        println!("w2 = {:?}", w2);
        println!("w3 = {:?}", w3);
        test_semiring(w1, w2, w3);
    }
}
    

fn main() {
    let seed: &[_] = &[7,7,7];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    println!("Testing `TropicalWeight<f32>`:");
    test::<TropicalWeight<f32>>(&mut rng, 10, true);
    println!("============================================================\n");
    // println!("Testing `LogWeight<f32>`:");
    // test::<LogWeight<f32>>(&mut rng, 10, true);
    // println!("============================================================\n");
    println!("Testing `MinmaxWeight<f32>`:");
    test::<MinmaxWeight<f32>>(&mut rng, 10, true);
    println!("============================================================\n");
}
