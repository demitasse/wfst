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

//! This module implements some Weight types based on floating point
//! numbers. See the source files `main_semiring.rs` and
//! `test_semiring.rs` for simple examples of intended use.

use super::*;
use super::float::Float;

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
