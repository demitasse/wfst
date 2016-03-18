extern crate rustc_serialize;
use rustc_serialize::{Encodable};

use std::{f64, f32};
use std::option::Option;

const DELTA: f32 = 1.0 / 1024.0;

pub trait Weight {
    fn is_member(&self) -> bool;
    fn plus(self, rhs: Self) -> Self;
    fn times(self, rhs: Self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn approx_eq(self, rhs: Self) -> bool;
}

//We give the struct "Copy semantics", i.e. it will be copied instead
// of "moved". We may want to revisit this decision, but in the
// meantime this is more in line with the behaviour of numeric types.
#[derive(Copy, Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct TropicalWeight<T> {
    val: Option<T>
}

impl<T> TropicalWeight<T> {
    pub fn new(val: Option<T>) -> TropicalWeight<T> {
        TropicalWeight {val: val}
    }
}

impl Weight for TropicalWeight<f64> {
    fn plus(self, rhs: TropicalWeight<f64>) -> TropicalWeight<f64> {
        if (!self.is_member()) || (!rhs.is_member()) {
            TropicalWeight::new(None)
        } else if self.val < rhs.val {
            TropicalWeight::new(self.val)
        } else {
            TropicalWeight::new(rhs.val)
        }        
    }

    fn times(self, rhs: TropicalWeight<f64>) -> TropicalWeight<f64> {
        if (!self.is_member()) || (!rhs.is_member()) {
            TropicalWeight::new(None)
        } else {
            TropicalWeight::new(Some(self.val.unwrap() + rhs.val.unwrap()))
        }
    }

    fn zero() -> TropicalWeight<f64> {
        TropicalWeight::new(Some(f64::INFINITY))
    }

    fn one() -> TropicalWeight<f64> {
        TropicalWeight::new(Some(0.0))
    }

    fn is_member(&self) -> bool {
        if let Some(val) = self.val {
            !(val == f64::NAN || val == f64::NEG_INFINITY)
        } else {
            false
        }
    }

    fn approx_eq(self, rhs: Self) -> bool {
        if let Some(val) = self.val {
            if let Some(val2) = rhs.val {
                val <= val2 + DELTA as f64 && val2 <= val + DELTA as f64
            } else {
                false
            }
        } else {
            false
        }        
    }
}

//DEMITASSE: May be a way to collapse the following into the above
// (written for f64)
impl Weight for TropicalWeight<f32> {
    fn plus(self, rhs: TropicalWeight<f32>) -> TropicalWeight<f32> {
        if (!self.is_member()) || (!rhs.is_member()) {
            TropicalWeight::new(None)
        } else if self.val < rhs.val {
            TropicalWeight::new(self.val)
        } else {
            TropicalWeight::new(rhs.val)
        }        
    }

    fn times(self, rhs: TropicalWeight<f32>) -> TropicalWeight<f32> {
        if (!self.is_member()) || (!rhs.is_member()) {
            TropicalWeight::new(None)
        } else {
            TropicalWeight::new(Some(self.val.unwrap() + rhs.val.unwrap()))
        }
    }

    fn zero() -> TropicalWeight<f32> {
        TropicalWeight::new(Some(f32::INFINITY))
    }

    fn one() -> TropicalWeight<f32> {
        TropicalWeight::new(Some(0.0))
    }

    fn is_member(&self) -> bool {
        if let Some(val) = self.val {
            !(val == f32::NAN || val == f32::NEG_INFINITY)
        } else {
            false
        }
    }

    fn approx_eq(self, rhs: Self) -> bool {
        if let Some(val) = self.val {
            if let Some(val2) = rhs.val {
                val <= val2 + DELTA as f32 && val2 <= val + DELTA as f32
            } else {
                false
            }
        } else {
            false
        }        
    }

}
