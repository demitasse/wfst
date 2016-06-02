extern crate semiring;
extern crate fst;

use semiring::TropicalWeight;
use fst::{VecFst, StdArc, Arc, Fst, MutableFst, ExpandedFst};
use fst::operations as fstops;

fn main() {

    let a = StdArc::new(0, 0, TropicalWeight::<f32>::new(Some(12.0000001)), 1);
    let b = StdArc::new(0, 0, TropicalWeight::<f64>::new(Some(12.0000001)), 1);
    let c = b.clone();

    println!("Weight: {:?}", a.weight());
    println!("Olabel: {:?}", b.olabel());
    println!("Nextstate: {:?}", c.nextstate());
    println!("");

    let mut fst = VecFst::<TropicalWeight<f32>>::new();
    let s0 = fst.add_state(TropicalWeight::new(Some(23.0)));
    let s1 = fst.add_state(TropicalWeight::new(Some(24.0)));
    let s2 = fst.add_state(TropicalWeight::new(Some(25.0)));
    fst.set_start(0);
    fst.add_arc(s0, s1, 0, 0, TropicalWeight::new(Some(0.0)));
    fst.add_arc(s1, s2, 0, 0, TropicalWeight::new(Some(1.0)));
    fst.add_arc(s1, s2, 0, 0, TropicalWeight::new(Some(2.0)));
    println!("{:?}", fst);
    println!("");
    for arc in fst.arc_iter(1) {
        //CAN'T DO ANY OF THE FOLLOWING, BECAUSE ITERATOR BORROWS `fst`
        // AS IMMUTABLE:
        //let s3 = fst.add_state(TropicalWeight::new(Some(23.0)));
        //fst.add_arc(s0, s2, 0, 0, TropicalWeight::new(Some(2.0)));
        println!("{:?}", arc);
    }
    println!("");

    for arc in fst.arc_iter(1).cloned().collect::<Vec<_>>() {
        // CAN DO THIS NOW BECAUSE COLLECTED CLONES OF ARCS:
        let ss = fst.add_state(TropicalWeight::new(Some(23.0)));
        fst.add_arc(s0, ss, 0, 0, TropicalWeight::new(Some(3.0)));

        println!("{:?}", arc);
    }
    println!("");
    println!("{:?}", fst);    
    
    println!("");
    println!("Number of states: {}", fst.get_numstates());    

    //Not yet implemented!
    let fst2: VecFst<TropicalWeight<f32>> = fstops::extendfinal(fst);
    println!("{:?}", fst2);
}
