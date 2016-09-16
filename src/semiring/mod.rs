// Author: Daniel van Niekerk <dvn.demitasse@gmail.com>
//
// Copyright 2016 The Department of Arts and Culture of the Government
// of South Africa
//
// See the "LICENCE" file for information on usage and redistribution
// of this file.

//! This module defines the Weight and other traits which specifies a
//! semiring and other properties. See the source files
//! `main_semiring.rs` and `test_semiring.rs` for simple examples of
//! intended use.

use std::fmt::Debug;
use std::option::Option;

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

/// ∀ a,b,c: c ⊗ (a ⊕ b) = (c ⊗ a) ⊕ (c ⊗ b)
pub trait LeftSemiring {}
/// ∀ a,b,c: (a ⊕ b) ⊗ c = (a ⊗ c) ⊕ (b ⊗ c)
pub trait RightSemiring {}
/// Both `LeftSemiring` and `RightSemiring` apply
pub trait Semiring: LeftSemiring + RightSemiring {}
/// ∀ a,b: a ⊗ b = b ⊗ a
pub trait Commutative {}
/// ∀ a: a ⊕ a = a
pub trait Idempotent {}
/// ∀ a,b: a ⊕ b = a ∨ a ⊕ b = b
pub trait Path {}

/// Power is the iterated product for arbitrary semirings such that
/// Power(w, 0) is One() for the semiring, and
/// Power(w, n) = Times(Power(w, n-1), w)
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

mod float;
pub mod test;
pub mod floatweight;
