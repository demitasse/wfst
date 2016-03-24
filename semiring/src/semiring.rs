extern crate rustc_serialize;
use rustc_serialize::{Encodable};

use std::ops::{Add, Sub};
use std::{f64, f32};
use std::option::Option;


////////////////////////////////////////////////////////////////////////////////
// Code for abstracting floats

// Used to implement `approx_eq()` and `quantize()`
const DEFAULT_DELTA: f32 = 1.0 / 1024.0;

// Internal float trait for our implementations over either f32 or f64
pub trait Float<T>: Copy + PartialOrd + Add<Output=T> + Sub<Output=T> {
    fn zero() -> T;
    fn one() -> T;
    fn nan() -> T;
    fn infty() -> T;
    fn neg_infty() -> T;
    fn approx_eq(self, rhs: T, delta: Option<f32>) -> bool;
    fn quantize(self, delta: Option<f32>) -> T;
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

    fn approx_eq(self, rhs: f64, delta: Option<f32>) -> bool {
        let d = if let Some(d) = delta {
            d as f64
        } else {
            DEFAULT_DELTA as f64
        };
        self <= rhs + d && rhs <= self + d
    }

    fn quantize(self, delta: Option<f32>) -> f64 {
        let d = if let Some(d) = delta {
            d as f64
        } else {
            DEFAULT_DELTA as f64
        };
        if self == f64::NEG_INFINITY ||
            self == f64::INFINITY ||
            self == f64::NAN {
                self
            } else {
                (self / d + 0.5).floor() * d
            }
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

    fn approx_eq(self, rhs: f32, delta: Option<f32>) -> bool {
        let d = if let Some(d) = delta {
            d
        } else {
            DEFAULT_DELTA
        };
        self <= rhs + d && rhs <= self + d
    }

    fn quantize(self, delta: Option<f32>) -> f32 {
        let d = if let Some(d) = delta {
            d
        } else {
            DEFAULT_DELTA
        };
        if self == f32::NEG_INFINITY ||
            self == f32::INFINITY ||
            self == f32::NAN {
                self
            } else {
                (self / d + 0.5).floor() * d
            }
    }
}

////////////////////////////////////////////////////////////////////////////////
const LEFT_SEMIRING: u64 = 0x01; // FORALL a,b,c: Times(c, Plus(a,b)) = Plus(Times(c,a), Times(c, b))
const RIGHT_SEMIRING: u64 = 0x02; // FORALL a,b,c: Times(Plus(a,b), c) = Plus(Times(a,c), Times(b, c))
const SEMIRING: u64 = LEFT_SEMIRING | RIGHT_SEMIRING;
const COMMUTATIVE: u64 = 0x04; // FORALL a,b: Times(a,b) = Times(b,a)
const IDEMPOTENT: u64 = 0x08; // FORALL a: Plus(a, a) = a
const PATH: u64 = 0x10; // FORALL a,b: Plus(a,b) = a or Plus(a,b) = b

// Define Weight
pub trait Weight<ReverseWeight> {
    fn is_member(&self) -> bool;
    fn plus(self, rhs: Self) -> Self;
    fn times(self, rhs: Self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn approx_eq(self, rhs: Self, delta: Option<f32>) -> bool;
    fn quantize(self, delta: Option<f32>) -> Self;
    fn divide(self, rhs: Self, divtype: Option<DivideType>) -> Self;
    fn reverse(self) -> ReverseWeight;
    fn properties() -> u64;
}

pub enum DivideType {
    Divleft,
    Divright,
    Divany
}


////////////////////////////////////////////////////////////////////////////////
// Weight and thus also semiring implementations

// We most of these structs "Copy semantics", i.e. it will be copied
// instead of "moved". We may want to revisit this decision, but in
// the meantime this is more in line with the behaviour of numeric
// types.

////////////////////////////////////////////////////////////////////////////////
//TROPICAL SEMIRING: (min, +, inf, 0)
#[derive(Copy, Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct TropicalWeight<T: Float<T>> {
    val: Option<T>
}

impl<T: Float<T>> TropicalWeight<T> {
    pub fn new(val: Option<T>) -> TropicalWeight<T> {
        TropicalWeight {val: val}
    }
}

impl<T: Float<T>> Weight<TropicalWeight<T>> for TropicalWeight<T> {
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

    fn approx_eq(self, rhs: Self, delta: Option<f32>) -> bool {
        if let Some(val) = self.val {
            if let Some(val2) = rhs.val {
                val.approx_eq(val2, delta)
            } else {
                false
            }
        } else {
            false
        }        
    }

    fn quantize(self, delta: Option<f32>) -> TropicalWeight<T> {
        if let Some(val) = self.val {
            TropicalWeight::new(Some(val.quantize(delta)))    
        } else {
            TropicalWeight::new(None)
        }
    }

    fn divide(self, rhs: TropicalWeight<T>, divtype: Option<DivideType>) -> TropicalWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            TropicalWeight::new(None)
        } else if rhs.val.unwrap() == T::infty() {
            TropicalWeight::new(None)
        } else if self.val.unwrap() == T::infty() {
            TropicalWeight::new(Some(T::infty()))
        } else {
            TropicalWeight::new(Some(self.val.unwrap() - rhs.val.unwrap()))
        }
    }

    fn reverse(self) -> TropicalWeight<T> {
        self
    }

    fn properties() -> u64 {
        SEMIRING | COMMUTATIVE | IDEMPOTENT | PATH
    }
}

////////////////////////////////////////////////////////////////////////////////
//LOG SEMIRING: (log(e^-x + e^y), +, inf, 0)
