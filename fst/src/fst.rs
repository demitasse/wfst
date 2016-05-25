extern crate rustc_serialize;

extern crate semiring;
use semiring::*;


pub type Label = u64;
pub type StateId = u64;

#[derive(Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct Arc<T: Weight> {
    ilabel: Label,
    olabel: Label,
    weight: T,
    nextstate: StateId,
}

impl<T: Weight> Arc<T> {
    pub fn new(i: Label, o: Label, w: T, s: StateId) -> Arc<T> {
        Arc { ilabel: i,
              olabel: o,
              weight: w,
              nextstate: s }
    }
}

