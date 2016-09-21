// Author: Daniel van Niekerk <dvn.demitasse@gmail.com>
//
// Copyright 2016 The Department of Arts and Culture of the Government
// of South Africa
//
// See the "LICENCE" file for information on usage and redistribution
// of this file.

extern crate wfst;

use wfst::semiring::Weight;
use wfst::semiring::floatweight::TropicalWeight;
use wfst::{MutableFst};
use wfst::wfst_vec::VecFst;
use wfst::algorithms::shortestpath::shortest_paths;

// See:
//     Mehryar Mohri and Michael Riley. "An efficient algorithm for
//     the N-best-strings problem," In: *Proceedings of the
//     International Conference on Spoken Language Processing 2002*
//     (ICSLP'02), Denver, Colorado, September 2002.
fn main() {
    // //create "automaton B" (Fig. 2)
    // let mut fst = VecFst::<TropicalWeight<f64>>::new();
    // let s0 = fst.add_state(TropicalWeight::zero());
    // let s1 = fst.add_state(TropicalWeight::zero());
    // let s2 = fst.add_state(TropicalWeight::zero());
    // let s3 = fst.add_state(TropicalWeight::one());
    // fst.set_start(s0);
    // fst.set_isyms(vec!("a", "b", "c", "d").iter().map(|x| String::from(*x)));
    // fst.set_osyms(vec!("a", "b", "c", "d").iter().map(|x| String::from(*x)));
    // fst.add_arc(s0, s1, 0, 0, TropicalWeight::new(Some(0.1)));
    // fst.add_arc(s0, s2, 1, 1, TropicalWeight::new(Some(0.1)));
    // fst.add_arc(s1, s3, 2, 2, TropicalWeight::new(Some(0.4)));
    // fst.add_arc(s1, s3, 3, 3, TropicalWeight::new(Some(0.2)));
    // fst.add_arc(s2, s3, 2, 2, TropicalWeight::new(Some(0.3)));
    // fst.add_arc(s2, s3, 3, 3, TropicalWeight::new(Some(0.2)));
    // println!("==============================");
    // println!("{:?}", fst);
    // let fst2: VecFst<_> = shortest_paths(&mut fst, 2, false);
    // println!("==============================");
    // println!("{:?}", fst2);


    //create example: http://www.openfst.org/twiki/bin/view/FST/ShortestPathDoc
    let mut fst = VecFst::<TropicalWeight<f64>>::new();
    let s0 = fst.add_state(TropicalWeight::zero());
    let s1 = fst.add_state(TropicalWeight::zero());
    let s2 = fst.add_state(TropicalWeight::zero());
    let s3 = fst.add_state(TropicalWeight::new(Some(3.0)));
    fst.set_start(s0);
    fst.set_isyms(vec!("a", "b", "c", "d", "f").iter().map(|x| String::from(*x)));
    fst.set_osyms(vec!("a", "b", "c", "d", "f").iter().map(|x| String::from(*x)));
    fst.add_arc(s0, s1, 0, 0, TropicalWeight::new(Some(3.0)));
    fst.add_arc(s0, s2, 3, 3, TropicalWeight::new(Some(5.0)));
    fst.add_arc(s1, s1, 1, 1, TropicalWeight::new(Some(2.0)));
    fst.add_arc(s1, s3, 2, 2, TropicalWeight::new(Some(4.0)));
    fst.add_arc(s2, s3, 4, 4, TropicalWeight::new(Some(4.0)));
    println!("==============================");
    println!("{:?}", fst);
    let fst2: VecFst<_> = shortest_paths(&mut fst, 1, false);
    println!("==============================");
    println!("{:?}", fst2);

    //DEMIT TODO:
    // - Symbol tables are wrong (no epsilons)
    // - Shortest path seems to be inverted
    // - 
}
