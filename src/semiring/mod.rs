// Author: Daniel van Niekerk <dvn.demitasse@gmail.com>
//
// Copyright 2016 The Department of Arts and Culture of the Government
// of South Africa
//
// See the "LICENCE" file for information on usage and redistribution
// of this file.

// This file contains portions of code ported from OpenFst
// (http://www.openfst.org) under the following licence and
// attribution:
//
// """
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Copyright 2005-2010 Google, Inc.
// Author: riley@google.com (Michael Riley)
// """
////////////////////////////////////////////////////////////////////////////////

//! This module implements the Weight trait which specifies a
//! semiring. See the source files `main_semiring.rs` and
//! `test_semiring.rs` for simple examples of intended use.

use std::fmt::Debug;
use std::ops::{Add, Sub};
use std::{f64, f32};
use std::option::Option;

pub mod test;

////////////////////////////////////////////////////////////////////////////////
// Code for abstracting floats

// Used to implement `approx_eq()` and `quantize()`
const DEFAULT_DELTA: f32 = 1.0 / 1024.0;

// Internal float trait for our implementations over either f32 or f64
pub trait Float<T>: Debug + Copy + PartialOrd + Add<Output=T> + Sub<Output=T> {
    fn zero() -> T;
    fn one() -> T;
    fn nan() -> T;
    fn infty() -> T;
    fn neg_infty() -> T;
    fn logexp(self) -> T;
    fn approx_eq(self, rhs: T, delta: Option<f32>) -> bool;
    fn quantize(self, delta: Option<f32>) -> T;
    fn from_u32(u32) -> T;
    fn get_precision() -> &'static str;
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

    fn logexp(self) -> f64 {
        (1.0 + (-self).exp()).ln()
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
            self == f64::NAN
        {
            self
        } else {
            (self / d + 0.5).floor() * d
        }
    }

    fn from_u32(i: u32) -> f64 {
        i as f64
    }

    fn get_precision() -> &'static str {
        "64"
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

    fn logexp(self) -> f32 {
        (1.0 + (-self).exp()).ln()
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
            self == f32::NAN
        {
            self
        } else {
            (self / d + 0.5).floor() * d
        }
    }

    fn from_u32(i: u32) -> f32 {
        i as f32
    }

    fn get_precision() -> &'static str {
        "32"
    }
}

////////////////////////////////////////////////////////////////////////////////
pub const LEFT_SEMIRING: u64 = 0x01; // FORALL a,b,c: Times(c, Plus(a,b)) = Plus(Times(c,a), Times(c, b))
pub const RIGHT_SEMIRING: u64 = 0x02; // FORALL a,b,c: Times(Plus(a,b), c) = Plus(Times(a,c), Times(b, c))
pub const SEMIRING: u64 = LEFT_SEMIRING | RIGHT_SEMIRING;
pub const COMMUTATIVE: u64 = 0x04; // FORALL a,b: Times(a,b) = Times(b,a)
pub const IDEMPOTENT: u64 = 0x08; // FORALL a: Plus(a, a) = a
pub const PATH: u64 = 0x10; // FORALL a,b: Plus(a,b) = a or Plus(a,b) = b

// Define Weight (demit: may want to rethink having this `Copy`)
pub trait Weight: Copy + Debug {
    fn is_member(&self) -> bool;
    fn plus(self, rhs: Self) -> Self;
    fn times(self, rhs: Self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn none() -> Self;
    fn eq(self, rhs: Self) -> bool;
    fn approx_eq(self, rhs: Self, delta: Option<f32>) -> bool;
    fn quantize(self, delta: Option<f32>) -> Self;
    fn divide(self, rhs: Self, divtype: Option<DivideType>) -> Self;
    fn reverse(self) -> Self;
    fn properties() -> u64;
    fn wtype() -> String;
}

pub enum DivideType {
    Divleft,
    Divright,
    Divany
}

pub fn power<T: Weight>(w: T, n: u8) -> T {
    let mut result = T::one();
    for _ in 0..n {
        result = result.times(w);
    }
    result
}

////////////////////////////////////////////////////////////////////////////////
// Weight and thus also semiring implementations

// We give most of these structs "Copy semantics", i.e. it will be
// copied instead of "moved". We may want to revisit this decision,
// but in the meantime this is more in line with the behaviour of
// numeric types.

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

impl<T: Float<T>> Weight for TropicalWeight<T> {

    fn plus(self, rhs: TropicalWeight<T>) -> TropicalWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            TropicalWeight::new(None)
        } else if self.val < rhs.val {
            self   //because TropicalWeight is Copy
        } else {
            rhs    //because TropicalWeight is Copy
        }        
    }

    fn times(self, rhs: TropicalWeight<T>) -> TropicalWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            TropicalWeight::new(None)
        } else {
            let (v1, v2) = (self.val.unwrap(), rhs.val.unwrap());
            TropicalWeight::new(Some(v1 + v2))
        }
    }

    fn zero() -> TropicalWeight<T> {
        TropicalWeight::new(Some(T::infty()))
    }

    fn one() -> TropicalWeight<T> {
        TropicalWeight::new(Some(T::zero()))
    }

    fn none() -> TropicalWeight<T> {
        TropicalWeight::new(None)
    }

    fn is_member(&self) -> bool {
        if let Some(val) = self.val {
            !(val == T::nan() || val == T::neg_infty())
        } else {
            false
        }
    }

    fn eq(self, rhs: Self) -> bool {
        if let Some(val) = self.val {
            if let Some(val2) = rhs.val {
                val == val2
            } else {
                false
            }
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
        } else {
            let (v1, v2) = (self.val.unwrap(), rhs.val.unwrap());
            if v2 == T::infty() {
                TropicalWeight::new(None)
            } else if v1 == T::infty() {
                self   //because TropicalWeight is Copy
            } else {
                TropicalWeight::new(Some(v1 - v2))
            }
        }
    }

    fn reverse(self) -> TropicalWeight<T> {
        self
    }

    fn properties() -> u64 {
        SEMIRING | COMMUTATIVE | IDEMPOTENT | PATH
    }

    fn wtype() -> String {
        format!("tropical{}", T::get_precision())
    }
}

////////////////////////////////////////////////////////////////////////////////
//LOG SEMIRING: (log(e^-x + e^y), +, inf, 0)
#[derive(Copy, Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct LogWeight<T: Float<T>> {
    val: Option<T>
}

impl<T: Float<T>> LogWeight<T> {
    pub fn new(val: Option<T>) -> LogWeight<T> {
        LogWeight {val: val}
    }
}

impl<T: Float<T>> Weight for LogWeight<T> {

    fn plus(self, rhs: LogWeight<T>) -> LogWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            LogWeight::new(None)
        } else {
            let (v1, v2) = (self.val.unwrap(), rhs.val.unwrap());
            if v1 == T::infty() {
                rhs   //because LogWeight is Copy
            } else if v2 == T::infty() {
                self  //because LogWeight is Copy
            } else if v1 > v2 {
                LogWeight::new(Some(v2 - (v1 - v2).logexp()))
            } else {
                LogWeight::new(Some(v1 - (v2 - v1).logexp()))
            }
        }
    }

    fn times(self, rhs: LogWeight<T>) -> LogWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            LogWeight::new(None)
        } else {
            let (v1, v2) = (self.val.unwrap(), rhs.val.unwrap());
            if v1 == T::infty() {
                self   //because LogWeight is Copy
            } else if v2 == T::infty() {
                rhs    //because LogWeight is Copy
            } else {
                LogWeight::new(Some(v1 + v2))
            }
        }
    }

    fn zero() -> LogWeight<T> {
        LogWeight::new(Some(T::infty()))
    }

    fn one() -> LogWeight<T> {
        LogWeight::new(Some(T::zero()))
    }

    fn none() -> LogWeight<T> {
        LogWeight::new(None)
    }

    fn is_member(&self) -> bool {
        if let Some(val) = self.val {
            !(val == T::nan() || val == T::neg_infty())
        } else {
            false
        }
    }

    fn eq(self, rhs: Self) -> bool {
        if let Some(val) = self.val {
            if let Some(val2) = rhs.val {
                val == val2
            } else {
                false
            }
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

    fn quantize(self, delta: Option<f32>) -> LogWeight<T> {
        if let Some(val) = self.val {
            LogWeight::new(Some(val.quantize(delta)))    
        } else {
            LogWeight::new(None)
        }
    }

    fn divide(self, rhs: LogWeight<T>, divtype: Option<DivideType>) -> LogWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            LogWeight::new(None)
        } else {
            let (v1, v2) = (self.val.unwrap(), rhs.val.unwrap());
            if v2 == T::infty() {
                LogWeight::new(None)
            } else if v1 == T::infty() {
                self   //because LogWeight is Copy
            } else {
                LogWeight::new(Some(v1 - v2))
            }
        }
    }

    fn reverse(self) -> LogWeight<T> {
        self
    }

    fn properties() -> u64 {
        SEMIRING | COMMUTATIVE
    }

    fn wtype() -> String {
        format!("log{}", T::get_precision())
    }
}

////////////////////////////////////////////////////////////////////////////////
//MINMAX SEMIRING: (min, max, inf, -inf)
#[derive(Copy, Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct MinmaxWeight<T: Float<T>> {
    val: Option<T>
}

impl<T: Float<T>> MinmaxWeight<T> {
    pub fn new(val: Option<T>) -> MinmaxWeight<T> {
        MinmaxWeight {val: val}
    }
}

impl<T: Float<T>> Weight for MinmaxWeight<T> {

    fn plus(self, rhs: MinmaxWeight<T>) -> MinmaxWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            MinmaxWeight::new(None)
        } else if self.val < rhs.val {
            self   //because MinmaxWeight is Copy
        } else {
            rhs    //because MinmaxWeight is Copy
        }        
    }

    fn times(self, rhs: MinmaxWeight<T>) -> MinmaxWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            MinmaxWeight::new(None)
        } else if self.val >= rhs.val {
            self   //because MinmaxWeight is Copy
        } else {
            rhs    //because MinmaxWeight is Copy
        }        
    }

    fn zero() -> MinmaxWeight<T> {
        MinmaxWeight::new(Some(T::infty()))
    }

    fn one() -> MinmaxWeight<T> {
        MinmaxWeight::new(Some(T::neg_infty()))
    }

    fn none() -> MinmaxWeight<T> {
        MinmaxWeight::new(None)
    }

    fn is_member(&self) -> bool {
        if let Some(val) = self.val {
            !(val == T::nan())
        } else {
            false
        }
    }

    fn eq(self, rhs: Self) -> bool {
        if let Some(val) = self.val {
            if let Some(val2) = rhs.val {
                val == val2
            } else {
                false
            }
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

    fn quantize(self, delta: Option<f32>) -> MinmaxWeight<T> {
        if let Some(val) = self.val {
            MinmaxWeight::new(Some(val.quantize(delta)))    
        } else {
            MinmaxWeight::new(None)
        }
    }

    // Defined only for special cases
    fn divide(self, rhs: MinmaxWeight<T>, divtype: Option<DivideType>) -> MinmaxWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            MinmaxWeight::new(None)
        } else if self.val >= rhs.val {
            self   //because MinmaxWeight is Copy
        } else {
            MinmaxWeight::new(None)
        }
    }

    fn reverse(self) -> MinmaxWeight<T> {
        self
    }

    fn properties() -> u64 {
        SEMIRING | COMMUTATIVE | IDEMPOTENT | PATH        
    }

    fn wtype() -> String {
        format!("minmax{}", T::get_precision())
    }
}
