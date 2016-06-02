// extern crate semiring;
//extern crate fst;

use semiring::{Weight};
use {Fst, MutableFst};

pub fn extendfinal<'a, W: Weight, T: Fst<'a, W>, U: MutableFst<'a, W>> (ifst: T) -> U {
    println!("{:?}", ifst.get_start());
    unimplemented!();
}
