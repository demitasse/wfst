extern crate rustc_serialize;
use rustc_serialize::{Encodable};

use std::ops::Add;
use std::{f64, f32};
use std::option::Option;


////////////////////////////////////////////////////////////////////////////////
// Code for abstracting floats

// Used to implement `approx_eq()`
const DELTA: f32 = 1.0 / 1024.0;

// Internal float trait for our implementations over either f32 or f64
pub trait Float<T>: Copy + PartialOrd + Add<Output=T> {
    fn zero() -> T;
    fn one() -> T;
    fn nan() -> T;
    fn infty() -> T;
    fn neg_infty() -> T;
    fn approx_eq(self, rhs: T) -> bool;
}

impl Float<f64> for f64 {
    fn zero() -> f64 {
        0.0
    }

    fn one() -> f64 {
        1.0
    }

    fn nan() -> f64 {
        f64::NAN
    }

    fn infty() -> f64 {
        f64::INFINITY
    }

    fn neg_infty() -> f64 {
        f64::NEG_INFINITY
    }

    fn approx_eq(self, rhs: f64) -> bool {
        self <= rhs + DELTA as f64 && rhs <= self + DELTA as f64
    }
}

impl Float<f32> for f32 {
    fn zero() -> f32 {
        0.0
    }

    fn one() -> f32 {
        1.0
    }

    fn nan() -> f32 {
        f32::NAN
    }

    fn infty() -> f32 {
        f32::INFINITY
    }

    fn neg_infty() -> f32 {
        f32::NEG_INFINITY
    }

    fn approx_eq(self, rhs: f32) -> bool {
        self <= rhs + DELTA as f32 && rhs <= self + DELTA as f32
    }
}

////////////////////////////////////////////////////////////////////////////////
// Implementation of Weights to define different semirings
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
pub struct TropicalWeight<T: Float<T>> {
    val: Option<T>
}

impl<T: Float<T>> TropicalWeight<T> {
    pub fn new(val: Option<T>) -> TropicalWeight<T> {
        TropicalWeight {val: val}
    }
}

impl<T: Float<T>> Weight for TropicalWeight<T> {
    fn plus(self, rhs: TropicalWeight<T>) -> TropicalWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            TropicalWeight::new(None)
        } else if self.val < rhs.val {
            TropicalWeight::new(self.val)
        } else {
            TropicalWeight::new(rhs.val)
        }        
    }

    fn times(self, rhs: TropicalWeight<T>) -> TropicalWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            TropicalWeight::new(None)
        } else {
            TropicalWeight::new(Some(self.val.unwrap() + rhs.val.unwrap()))
        }
    }

    fn zero() -> TropicalWeight<T> {
        TropicalWeight::new(Some(T::infty()))
    }

    fn one() -> TropicalWeight<T> {
        TropicalWeight::new(Some(T::zero()))
    }

    fn is_member(&self) -> bool {
        if let Some(val) = self.val {
            !(val == T::nan() || val == T::neg_infty())
        } else {
            false
        }
    }

    fn approx_eq(self, rhs: Self) -> bool {
        if let Some(val) = self.val {
            if let Some(val2) = rhs.val {
                val.approx_eq(val2)
            } else {
                false
            }
        } else {
            false
        }        
    }
}
