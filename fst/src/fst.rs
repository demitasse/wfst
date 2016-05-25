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

// pub type Std32Arc = Arc<TropicalWeight<f32>>;
// pub type Std64Arc = Arc<TropicalWeight<f64>>;
// pub type Log32Arc = Arc<LogWeight<f32>>;
// pub type Log64Arc = Arc<LogWeight<f64>>;
// pub type Minmax32Arc = Arc<MinmaxWeight<f32>>;
// pub type Minmax64Arc = Arc<MinmaxWeight<f64>>;

// impl Std64Arc {
//     pub fn new(i: Label, o: Label, w: Option<f64>, s: StateId) -> Std64Arc {
//         Arc { ilabel: i,
//               olabel: o,
//               weight: TropicalWeight::<f64>::new(w),
//               nextstate: s }
//     }
// }

// impl Std32Arc {
//     pub fn new(i: Label, o: Label, w: Option<f32>, s: StateId) -> Std32Arc {
//         Arc { ilabel: i,
//               olabel: o,
//               weight: TropicalWeight::<f32>::new(w),
//               nextstate: s }
//     }
// }
