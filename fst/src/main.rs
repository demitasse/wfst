extern crate semiring;
extern crate fst;

use semiring::TropicalWeight;
use fst::Arc;


fn main() {

    let a = Arc::new(0, 0, TropicalWeight::<f32>::new(Some(12.0000001)), 1);
    let b = Arc::new(0, 0, TropicalWeight::<f64>::new(Some(12.0000001)), 1);
    let c = b.clone();

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
