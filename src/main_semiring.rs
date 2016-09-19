// Author: Daniel van Niekerk <dvn.demitasse@gmail.com>
//
// Copyright 2016 The Department of Arts and Culture of the Government
// of South Africa
//
// See the "LICENCE" file for information on usage and redistribution
// of this file.

extern crate wfst;

use wfst::semiring::Weight;
use wfst::semiring::floatweight::{TropicalWeight};

use std::collections::BinaryHeap;

fn main() {
    {
        let t = TropicalWeight::new(Some(12.0f64));
        let tt = TropicalWeight::new(Some(12.5f64)).quantize(Some(1.0));
        println!("{}", tt.approx_eq(&t, Some(1.0)));
        println!("{:?} ⊕ {:?} = {:?}", t, tt, t.plus(&tt)); 
        println!("{:?} ⊗ {:?} = {:?}", t, tt, t.times(&tt)); 
        println!("t.times(tt).divide(tt, None) ==> {:?}", t.times(&tt).divide(&tt, None)); 
    }
    let t: TropicalWeight<f32> = TropicalWeight::one().quantize(None);
    let tt = TropicalWeight::zero().quantize(None);
    println!("{}", tt.is_member());
    println!("{:?} ⊕ {:?} = {:?}", t, tt, t.plus(&tt)); 
    println!("{:?} ⊗ {:?} = {:?}", t, tt, t.times(&tt));
    println!("{:?}", t.times(&tt).divide(&tt, None));
    println!("==============================");

    let mut heap = BinaryHeap::new();
    heap.push(TropicalWeight::<f32>::zero());
    heap.push(TropicalWeight::new(Some( 11.0         )));
    heap.push(TropicalWeight::new(Some( 10.0         )));
    heap.push(TropicalWeight::new(Some( 9.0          )));
    heap.push(TropicalWeight::new(Some( 8.0          )));
    heap.push(TropicalWeight::new(Some( 7.0          )));
    heap.push(TropicalWeight::new(Some( 6.0          )));
    heap.push(TropicalWeight::new(Some( 5.0          )));
    heap.push(TropicalWeight::new(Some( 4.0          )));
    heap.push(TropicalWeight::new(Some( 3.0          )));
    heap.push(TropicalWeight::new(Some( 2.0          )));
    heap.push(TropicalWeight::new(Some( 1.0          )));
    heap.push(TropicalWeight::new(Some( 0.6989700043 )));
    heap.push(TropicalWeight::new(Some( 0.5228787453 )));
    heap.push(TropicalWeight::new(Some( 0.3979400087 )));
    heap.push(TropicalWeight::new(Some( 0.3010299957 )));
    heap.push(TropicalWeight::new(Some( 0.2218487496 )));
    heap.push(TropicalWeight::new(Some( 0.15490196   )));
    heap.push(TropicalWeight::new(Some( 0.096910013  )));
    heap.push(TropicalWeight::new(Some( 0.0457574906 )));
    heap.push(TropicalWeight::one());

    while let Some(w) = heap.pop() {
        println!("{:?}", w);
    }
}
