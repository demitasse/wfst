// Author: Daniel van Niekerk <dvn.demitasse@gmail.com>
//
// Copyright 2016 The Department of Arts and Culture of the Government
// of South Africa
//
// See the "LICENCE" file for information on usage and redistribution
// of this file.
////////////////////////////////////////////////////////////////////////////////

//! This crate implements Weighted Finite-State Transducers (WFSTs) as
//! described in:
//! 
//! Mehryar Mohri, Fernando Pereira, and Michael Riley. "The design
//! principles of a weighted finite-state transducer library," In:
//! *Theoretical Computer Science* vol. 231 issue 1 (2000): pp. 17-32.
//!
//! This is a re-implementation in Rust, containing ported fragments
//! from the following existing projects (see appropriate licences and
//! attribution in the source repository):
//!
//!  * OpenFst (http://www.openfst.org)
//!  * CMU Sphinx (http://cmusphinx.sourceforge.net/)
extern crate rustc_serialize;

use std::fmt::Debug;

////////////////////////////////////////////////////////////////////////////////
////////// SEMIRING MODULE PROVIDES WEIGHT TYPES
////////////////////////////////////////////////////////////////////////////////
pub mod semiring;
use semiring::Weight;

////////////////////////////////////////////////////////////////////////////////
////////// FST AND MUTABLE FST INTERFACES
////////////////////////////////////////////////////////////////////////////////

// This interface is informed by Section 4.1 of:
// Mehryar Mohri, Fernando Pereira, and Michael Riley. "The design
// principles of a weighted finite-state transducer library."
// Theoretical Computer Science 231.1 (2000): 17-32.
pub type Label = usize;
pub type StateId = usize;

pub trait Fst<W: Weight>: Debug {
    type Arc: Arc<W>;
    type Iter: Iterator<Item=Self::Arc>;
    type Symtab: IntoIterator<Item=String>;
    fn get_start(&self) -> Option<StateId>;
    fn get_finalweight(&self, StateId) -> W;
    fn arc_iter(&self, StateId) -> Self::Iter;
    fn get_isyms(&self) -> Option<Self::Symtab>;
    fn get_osyms(&self) -> Option<Self::Symtab>;
    fn is_final(&self, StateId) -> bool;
}

// This interface defined by looking at OpenFST (C++ and Java
// interfaces):
pub trait MutableFst<W: Weight>: Fst<W> {
    fn new() -> Self;
    fn set_start(&mut self, id: StateId);
    fn add_state(&mut self, finalweight: W) -> StateId;
    fn del_state(&mut self, StateId);
    fn del_states<T: IntoIterator<Item=StateId>>(&mut self, states: T);
    fn add_arc(&mut self, source: StateId, target: StateId, ilabel: Label, olabel: Label, weight: W);
    fn set_finalweight(&mut self, id: StateId, finalweight: W);
    fn set_isyms<T: IntoIterator<Item=String>>(&mut self, symtab: T);
    fn set_osyms<T: IntoIterator<Item=String>>(&mut self, symtab: T);
}

pub trait ExpandedFst<W: Weight>: Fst<W> + Clone {
    fn get_numstates(&self) -> usize;
}

pub trait Arc<W: Weight>: PartialEq + Debug + Clone  {
    fn ilabel(&self) -> Label;
    fn olabel(&self) -> Label;
    fn weight(&self) -> W;
    fn nextstate(&self) -> StateId;
}

////////////////////////////////////////////////////////////////////////////////
////////// GENERIC FST ALGORITHMS
////////////////////////////////////////////////////////////////////////////////
pub mod utils;
pub mod algorithms;


////////////////////////////////////////////////////////////////////////////////
////////// SPECIFIC FST IMPLEMENTATIONS
////////////////////////////////////////////////////////////////////////////////
pub mod wfst_vec;
