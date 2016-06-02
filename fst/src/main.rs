extern crate semiring;
extern crate fst;

use semiring::TropicalWeight;
use fst::{VecFst, Arc, MutableFst, Fst};


fn main() {

    let a = Arc::new(0, 0, TropicalWeight::<f32>::new(Some(12.0000001)), 1);
    let b = Arc::new(0, 0, TropicalWeight::<f64>::new(Some(12.0000001)), 1);
    let c = b.clone();

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("");

    let mut aa = VecFst::<TropicalWeight<f32>>::new();
    let s0 = aa.add_state(TropicalWeight::new(Some(23.0)));
    let s1 = aa.add_state(TropicalWeight::new(Some(24.0)));
    let s2 = aa.add_state(TropicalWeight::new(Some(25.0)));
    aa.set_start(0);
    aa.add_arc(s0, s1, 0, 0, TropicalWeight::new(Some(0.0)));
    aa.add_arc(s1, s2, 0, 0, TropicalWeight::new(Some(1.0)));
    aa.add_arc(s1, s2, 0, 0, TropicalWeight::new(Some(2.0)));
    println!("{:?}", aa);
    println!("");
    for arc in aa.arc_iter(1) {
        //CAN'T DO ANY OF THE FOLLOWING, BECAUSE ITERATOR BORROWS `aa`
        // AS IMMUTABLE:
        //let s3 = aa.add_state(TropicalWeight::new(Some(23.0)));
        //aa.add_arc(s0, s2, 0, 0, TropicalWeight::new(Some(2.0)));
        println!("{:?}", arc);
    }
    println!("");

    for arc in aa.arc_iter(1).cloned().collect::<Vec<_>>() {
        // CAN DO THIS NOW BECAUSE COLLECTED CLONES OF ARCS:
        let s3 = aa.add_state(TropicalWeight::new(Some(23.0)));
        aa.add_arc(s0, s3, 0, 0, TropicalWeight::new(Some(3.0)));

        println!("{:?}", arc);
    }
    println!("");
    println!("{:?}", aa);    

}
