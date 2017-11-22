// Author: Daniel van Niekerk <dvn.demitasse@gmail.com>
//
// Copyright 2016 The Department of Arts and Culture of the Government
// of South Africa
//
// See the "LICENCE" file for information on usage and redistribution
// of this file.

//! This module implements the Float trait which abstracts float types
//! for use in Weight. Not generally used directly.

use std::fmt::{Debug};
use std::ops::{Add, Sub};
use std::{f64, f32};

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
