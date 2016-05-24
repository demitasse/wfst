extern crate rustc_serialize;

extern crate semiring;
use semiring::*;


pub type Label = u32;
pub type StateId = u32;

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

pub type Std32Arc = Arc<TropicalWeight<f32>>;
pub type Std64Arc = Arc<TropicalWeight<f64>>;
pub type Log32Arc = Arc<LogWeight<f32>>;
pub type Log64Arc = Arc<LogWeight<f64>>;
pub type Minmax32Arc = Arc<MinmaxWeight<f32>>;
pub type Minmax64Arc = Arc<MinmaxWeight<f64>>;

