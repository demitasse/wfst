extern crate rand;
use rand::{SeedableRng, StdRng};

extern crate semiring_test;
use semiring_test::RandomWeightGenerator;

extern crate semiring;
use semiring::*;

fn main() {
    let seed: &[_] = &[7,7,7];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f64>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f32>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f32>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f64>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f32>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f32>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f64>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f32>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f32>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f64>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f32>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f32>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f64>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f32>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f32>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f64>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f32>);
    println!("Hello Weight: {:?}", rng.genweight(false) as TropicalWeight<f32>);
}
