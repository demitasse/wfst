extern crate num;
use std::{f64, f32};
use std::option::Option;

pub trait Weight {
    fn is_member(&self) -> bool;
    fn plus(self, rhs: Self) -> Self;
    fn times(self, rhs: Self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
}

//We give the struct "Copy semantics", i.e. it will be copied instead
// of "moved". We may want to revisit this decision, but in the
// meantime this is more in line with the behaviour of numeric types.
#[derive(Copy, Clone, Debug)]
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
}

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
}
