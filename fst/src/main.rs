extern crate semiring;
extern crate fst;

use semiring::TropicalWeight;
use fst::{VecFst, Arc, MutableFst};


fn main() {

    let a = Arc::new(0, 0, TropicalWeight::<f32>::new(Some(12.0000001)), 1);
    let b = Arc::new(0, 0, TropicalWeight::<f64>::new(Some(12.0000001)), 1);
    let c = b.clone();

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    let mut aa = VecFst::<TropicalWeight<f32>>::new();
    aa.add_state(TropicalWeight::new(Some(23.0)));
    aa.add_state(TropicalWeight::new(Some(24.0)));
    aa.set_start(0);
    println!("{:?}", aa);
}
