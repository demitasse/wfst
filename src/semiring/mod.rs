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
pub trait Float<T>: Debug + Clone + PartialOrd + Add<Output=T> + Sub<Output=T> {
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
    fn zero() -> f64 { 0.0 }
    fn one() -> f64 { 1.0 }
    fn nan() -> f64 { f64::NAN }
    fn infty() -> f64 { f64::INFINITY }
    fn neg_infty() -> f64 { f64::NEG_INFINITY }

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
    fn zero() -> f32 { 0.0 }
    fn one() -> f32 { 1.0 }
    fn nan() -> f32 { f32::NAN }
    fn infty() -> f32 { f32::INFINITY }
    fn neg_infty() -> f32 { f32::NEG_INFINITY }

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

pub trait Weight: PartialEq + Clone + Debug {
    fn is_member(&self) -> bool;
    fn plus(&self, rhs: &Self) -> Self;
    fn times(&self, rhs: &Self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn none() -> Self;
    fn approx_eq(&self, rhs: &Self, delta: Option<f32>) -> bool;
    fn quantize(&self, delta: Option<f32>) -> Self;
    fn divide(&self, rhs: &Self, divtype: Option<DivideType>) -> Self;
    fn reverse(&self) -> Self;
    fn wtype() -> String;
}

pub enum DivideType {
    Divleft,
    Divright,
    Divany
}

/// FORALL a,b,c: Times(c, Plus(a,b)) = Plus(Times(c,a), Times(c, b))
pub trait LeftSemiring {}
/// FORALL a,b,c: Times(Plus(a,b), c) = Plus(Times(a,c), Times(b, c))
pub trait RightSemiring {}
pub trait Semiring: LeftSemiring + RightSemiring {}
/// FORALL a,b: Times(a,b) = Times(b,a)
pub trait Commutative {}
// FORALL a: Plus(a, a) = a
pub trait Idempotent {}
// FORALL a,b: Plus(a,b) = a or Plus(a,b) = b
pub trait Path {}

// Power is the iterated product for arbitrary semirings such that
// Power(w, 0) is One() for the semiring, and
// Power(w, n) = Times(Power(w, n-1), w)
pub fn power<T: Weight>(w: &T, n: u8) -> T {
    let mut result = T::one();
    for _ in 0..n {
        result = result.times(w);
    }
    result
}

// NATURAL ORDER
// By definition:
//                 a <= b iff a + b = a
// The natural order is a negative partial order iff the semiring is
// idempotent. It is trivially monotonic for plus. It is left
// (resp. right) monotonic for times iff the semiring is left
// (resp. right) distributive. It is a total order iff the semiring
// has the path property. See Mohri, "Semiring Framework and
// Algorithms for Shortest-Distance Problems", Journal of Automata,
// Languages and Combinatorics 7(3):321-350, 2002. We define the
// strict version of this order below.
pub fn le<T: Weight + Idempotent>(w1: &T, w2: &T) -> bool {
    w1.plus(w2).eq(w1) && !w1.eq(w2)
}


////////////////////////////////////////////////////////////////////////////////
// Weight and semiring implementations
////////////////////////////////////////////////////////////////////////////////
//TROPICAL SEMIRING: (min, +, inf, 0)
#[derive(Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct TropicalWeight<T: Float<T>> {
    val: Option<T>
}

impl<T: Float<T>> TropicalWeight<T> {
    pub fn new(val: Option<T>) -> Self {
        TropicalWeight {val: val}
    }
}

impl<T: Float<T>> Weight for TropicalWeight<T> {
    fn plus(&self, rhs: &Self) -> Self {
        if (!self.is_member()) || (!rhs.is_member()) {
            Self::new(None)
        } else if self.val < rhs.val {
            self.clone()
        } else {
            rhs.clone()
        }        
    }

    fn times(&self, rhs: &Self) -> Self {
        if (!self.is_member()) || (!rhs.is_member()) {
            Self::new(None)
        } else {
            let (v1, v2) = (self.val.clone().unwrap(), rhs.val.clone().unwrap());
            Self::new(Some(v1 + v2))
        }
    }

    fn zero() -> Self {
        Self::new(Some(T::infty()))
    }

    fn one() -> Self {
        Self::new(Some(T::zero()))
    }

    fn none() -> Self {
        Self::new(None)
    }

    fn is_member(&self) -> bool {
        if let Some(val) = self.val.clone() {
            !(val == T::nan() || val == T::neg_infty())
        } else {
            false
        }
    }

    fn approx_eq(&self, rhs: &Self, delta: Option<f32>) -> bool {
        if let Some(val) = self.val.clone() {
            if let Some(val2) = rhs.val.clone() {
                val.approx_eq(val2, delta)
            } else {
                false
            }
        } else {
            false
        }        
    }

    fn quantize(&self, delta: Option<f32>) -> Self {
        if let Some(val) = self.val.clone() {
            Self::new(Some(val.quantize(delta)))    
        } else {
            Self::new(None)
        }
    }

    #[allow(unused_variables)]
    fn divide(&self, rhs: &Self, divtype: Option<DivideType>) -> Self {
        if (!self.is_member()) || (!rhs.is_member()) {
            Self::new(None)
        } else {
            let (v1, v2) = (self.val.clone().unwrap(), rhs.val.clone().unwrap());
            if v2 == T::infty() {
                Self::new(None)
            } else if v1 == T::infty() {
                self.clone()
            } else {
                Self::new(Some(v1 - v2))
            }
        }
    }

    fn reverse(&self) -> Self {
        self.clone()
    }

    fn wtype() -> String {
        format!("tropical{}", T::get_precision())
    }
}

impl<T: Float<T>> PartialEq for TropicalWeight<T> {
    fn eq(&self, rhs: &Self) -> bool {
        if let Some(val) = self.val.clone() {
            if let Some(val2) = rhs.val.clone() {
                val == val2
            } else {
                false
            }
        } else {
            false
        }        
    }
}

impl<T: Float<T>> LeftSemiring for TropicalWeight<T> {}
impl<T: Float<T>> RightSemiring for TropicalWeight<T> {}
impl<T: Float<T>> Semiring for TropicalWeight<T> {}
impl<T: Float<T>> Commutative for TropicalWeight<T> {}
impl<T: Float<T>> Idempotent for TropicalWeight<T> {}
impl<T: Float<T>> Path for TropicalWeight<T> {}

////////////////////////////////////////////////////////////////////////////////
//LOG SEMIRING: (log(e^-x + e^y), +, inf, 0)
#[derive(Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct LogWeight<T: Float<T>> {
    val: Option<T>
}

impl<T: Float<T>> LogWeight<T> {
    pub fn new(val: Option<T>) -> Self {
        LogWeight {val: val}
    }
}

impl<T: Float<T>> Weight for LogWeight<T> {

    fn plus(&self, rhs: &Self) -> Self {
        if (!self.is_member()) || (!rhs.is_member()) {
            Self::new(None)
        } else {
            let (v1, v2) = (self.val.clone().unwrap(), rhs.val.clone().unwrap());
            if v1 == T::infty() {
                rhs.clone()
            } else if v2 == T::infty() {
                self.clone()
            } else if v1 > v2 {
                Self::new(Some(v2.clone() - (v1 - v2).logexp()))
            } else {
                Self::new(Some(v1.clone() - (v2 - v1).logexp()))
            }
        }
    }

    fn times(&self, rhs: &Self) -> Self {
        if (!self.is_member()) || (!rhs.is_member()) {
            Self::new(None)
        } else {
            let (v1, v2) = (self.val.clone().unwrap(), rhs.val.clone().unwrap());
            if v1 == T::infty() {
                self.clone()
            } else if v2 == T::infty() {
                rhs.clone()
            } else {
                Self::new(Some(v1 + v2))
            }
        }
    }

    fn zero() -> Self {
        Self::new(Some(T::infty()))
    }

    fn one() -> Self {
        Self::new(Some(T::zero()))
    }

    fn none() -> Self {
        Self::new(None)
    }

    fn is_member(&self) -> bool {
        if let Some(val) = self.val.clone() {
            !(val == T::nan() || val == T::neg_infty())
        } else {
            false
        }
    }

    fn approx_eq(&self, rhs: &Self, delta: Option<f32>) -> bool {
        if let Some(val) = self.val.clone() {
            if let Some(val2) = rhs.val.clone() {
                val.approx_eq(val2, delta)
            } else {
                false
            }
        } else {
            false
        }        
    }

    fn quantize(&self, delta: Option<f32>) -> Self {
        if let Some(val) = self.val.clone() {
            Self::new(Some(val.quantize(delta)))    
        } else {
            Self::new(None)
        }
    }

    #[allow(unused_variables)]
    fn divide(&self, rhs: &Self, divtype: Option<DivideType>) -> Self {
        if (!self.is_member()) || (!rhs.is_member()) {
            Self::new(None)
        } else {
            let (v1, v2) = (self.val.clone().unwrap(), rhs.val.clone().unwrap());
            if v2 == T::infty() {
                Self::new(None)
            } else if v1 == T::infty() {
                self.clone()
            } else {
                Self::new(Some(v1 - v2))
            }
        }
    }

    fn reverse(&self) -> Self {
        self.clone()
    }

    fn wtype() -> String {
        format!("log{}", T::get_precision())
    }
}

impl<T: Float<T>> PartialEq for LogWeight<T> {
    fn eq(&self, rhs: &Self) -> bool {
        if let Some(val) = self.val.clone() {
            if let Some(val2) = rhs.val.clone() {
                val == val2
            } else {
                false
            }
        } else {
            false
        }        
    }
}

impl<T: Float<T>> LeftSemiring for LogWeight<T> {}
impl<T: Float<T>> RightSemiring for LogWeight<T> {}
impl<T: Float<T>> Semiring for LogWeight<T> {}
impl<T: Float<T>> Commutative for LogWeight<T> {}

////////////////////////////////////////////////////////////////////////////////
//MINMAX SEMIRING: (min, max, inf, -inf)
#[derive(Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct MinmaxWeight<T: Float<T>> {
    val: Option<T>
}

impl<T: Float<T>> MinmaxWeight<T> {
    pub fn new(val: Option<T>) -> Self {
        MinmaxWeight {val: val}
    }
}

impl<T: Float<T>> Weight for MinmaxWeight<T> {

    fn plus(&self, rhs: &Self) -> Self {
        if (!self.is_member()) || (!rhs.is_member()) {
            Self::new(None)
        } else if self.val < rhs.val {
            self.clone()
        } else {
            rhs.clone()
        }        
    }

    fn times(&self, rhs: &Self) -> Self {
        if (!self.is_member()) || (!rhs.is_member()) {
            Self::new(None)
        } else if self.val >= rhs.val {
            self.clone()
        } else {
            rhs.clone()
        }        
    }

    fn zero() -> Self {
        Self::new(Some(T::infty()))
    }

    fn one() -> Self {
        Self::new(Some(T::neg_infty()))
    }

    fn none() -> Self {
        Self::new(None)
    }

    fn is_member(&self) -> bool {
        if let Some(val) = self.val.clone() {
            !(val == T::nan())
        } else {
            false
        }
    }

    fn approx_eq(&self, rhs: &Self, delta: Option<f32>) -> bool {
        if let Some(val) = self.val.clone() {
            if let Some(val2) = rhs.val.clone() {
                val.approx_eq(val2, delta)
            } else {
                false
            }
        } else {
            false
        }        
    }

    fn quantize(&self, delta: Option<f32>) -> Self {
        if let Some(val) = self.val.clone() {
            Self::new(Some(val.quantize(delta)))    
        } else {
            Self::new(None)
        }
    }

    // Defined only for special cases
    #[allow(unused_variables)]
    fn divide(&self, rhs: &Self, divtype: Option<DivideType>) -> Self {
        if (!self.is_member()) || (!rhs.is_member()) {
            Self::new(None)
        } else if self.val >= rhs.val {
            self.clone()
        } else {
            Self::new(None)
        }
    }

    fn reverse(&self) -> Self {
        self.clone()
    }

    fn wtype() -> String {
        format!("minmax{}", T::get_precision())
    }
}

impl<T: Float<T>> PartialEq for MinmaxWeight<T> {
    fn eq(&self, rhs: &Self) -> bool {
        if let Some(val) = self.val.clone() {
            if let Some(val2) = rhs.val.clone() {
                val == val2
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<T: Float<T>> LeftSemiring for MinmaxWeight<T> {}
impl<T: Float<T>> RightSemiring for MinmaxWeight<T> {}
impl<T: Float<T>> Semiring for MinmaxWeight<T> {}
impl<T: Float<T>> Commutative for MinmaxWeight<T> {}
impl<T: Float<T>> Idempotent for MinmaxWeight<T> {}
impl<T: Float<T>> Path for MinmaxWeight<T> {}
