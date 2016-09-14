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
    fn get_start(&self) -> Option<StateId>;
    fn get_finalweight(&self, StateId) -> W;       //Weight is Copy
    fn arc_iter(&self, StateId) -> Self::Iter;
}

// This interface defined by looking at OpenFST (C++ and Java
// implementations):
pub trait MutableFst<W: Weight>: Fst<W> {
    fn set_start(&mut self, id: StateId);
    fn add_state(&mut self, finalweight: W) -> StateId;
    fn del_state(&mut self, StateId);
    fn add_arc(&mut self, source: StateId, target: StateId, ilabel: Label, olabel: Label, weight: W);
    fn set_finalweight(&mut self, id: StateId, finalweight: W);
}

pub trait ExpandedFst<W: Weight>: Fst<W> {
    fn get_numstates(&self) -> usize;
}

pub trait Arc<W: Weight>: Debug + Clone  {
    fn ilabel(&self) -> Label;
    fn olabel(&self) -> Label;
    fn weight(&self) -> W;
    fn nextstate(&self) -> StateId;
}

////////////////////////////////////////////////////////////////////////////////
////////// GENERIC FST ALGORITHMS
////////////////////////////////////////////////////////////////////////////////
pub mod gen_algo;


////////////////////////////////////////////////////////////////////////////////
////////// SPECIFIC FST IMPLEMENTATIONS
////////////////////////////////////////////////////////////////////////////////
pub mod wfst_vec;
