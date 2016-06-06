extern crate rustc_serialize;

use std::fmt::Debug;
use std::vec;
use std::slice;

extern crate semiring;
use semiring::*;

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
    type State: State<W> + Debug;
    fn get_start(&self) -> Option<StateId>;
    fn get_finalweight(&self, StateId) -> W;       //Weight is Copy
    fn state(&self, StateId) -> Option<&Self::State>;
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

pub trait Arc<W: Weight>: Clone {
    fn ilabel(&self) -> Label;
    fn olabel(&self) -> Label;
    fn weight(&self) -> W;
    fn nextstate(&self) -> StateId;
}

pub trait State<W: Weight>: IntoIterator {
    type Arc: Arc<W> + Debug;
}

////////////////////////////////////////////////////////////////////////////////
////////// FST IMPLEMENTATION 1: A Mutable FST using Vectors
////////////////////////////////////////////////////////////////////////////////

////////// ARC
#[derive(Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct StdArc<W: Weight> {
    ilabel: Label,
    olabel: Label,
    weight: W,
    nextstate: StateId,
}

impl<W: Weight> Arc<W> for StdArc<W> {
    fn ilabel(&self) -> Label {
        self.ilabel
    }

    fn olabel(&self) -> Label {
        self.olabel
    }

    fn weight(&self) -> W {
        self.weight
    }

    fn nextstate(&self) -> StateId {
        self.nextstate
    }
}

impl<W: Weight> StdArc<W> {
    pub fn new(i: Label, o: Label, w: W, s: StateId) -> StdArc<W> {
        StdArc { ilabel: i,
                 olabel: o,
                 weight: w,
                 nextstate: s }
    }
}

////////// STATE
#[derive(Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct VecState<W: Weight> {
    finalweight: W,
    arcs: Vec<StdArc<W>>
}

impl<W: Weight> VecState<W> {
    fn new(finalweight: W) -> VecState<W> {
        VecState { finalweight: finalweight,
                   arcs: Vec::new() }
    }
}

impl<W: Weight> State<W> for VecState<W> {
    type Arc = StdArc<W>;
}

////////// FST
#[derive(Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct VecFst<W: Weight> {
    states: Vec<VecState<W>>,   //we need to make sure that element indexes are always consistent with arcs
    startstate: Option<usize>,
    isyms: Option<Vec<String>>,
    osyms: Option<Vec<String>>
}

impl<W: Weight> Fst<W> for VecFst<W> {
    type State = VecState<W>;

    fn get_start(&self) -> Option<StateId> {
        self.startstate
    }

    fn get_finalweight(&self, id: StateId) -> W {
        self.states[id].finalweight
    }

    fn state(&self, id: StateId) -> Option<&Self::State> {
        self.states.get(id)
    }
}

impl<W: Weight> MutableFst<W> for VecFst<W> {  
    fn set_start(&mut self, id: StateId) {
        assert!(id < self.states.len());
        self.startstate = Some(id);
    }

    fn add_state(&mut self, finalweight: W) -> StateId {
        let id = self.states.len();
        self.states.push(VecState::new(finalweight));
        id
    }

    fn del_state(&mut self, id: StateId) {
        assert!(id != self.startstate.unwrap());
        self.states.remove(id);
        //update arcs in remaining states
        for i in 0..self.states.len() {
            for j in 0..self.states[i].arcs.len() {
                if self.states[i].arcs[j].nextstate == id {
                    self.states[i].arcs.remove(j);
                } else if self.states[i].arcs[j].nextstate > id {
                    self.states[i].arcs[j].nextstate -= 1;
                }
            }
        }
    }

    fn add_arc(&mut self, source: StateId, target: StateId, ilabel: Label, olabel: Label, weight: W) {
        assert!(source < self.states.len());
        assert!(target < self.states.len());
        self.states[source].arcs.push(StdArc::new(ilabel, olabel, weight, target));
    }

    fn set_finalweight(&mut self, id: StateId, finalweight: W) {
        assert!(id < self.states.len());
        self.states[id].finalweight = finalweight;
    }
}

impl<W: Weight> ExpandedFst<W> for VecFst<W> {  
    fn get_numstates(&self) -> usize {
        self.states.len()
    }
}


impl<W: Weight> VecFst<W> {
    pub fn new() -> VecFst<W> {
        VecFst { states: Vec::new(),
                 startstate: None,
                 isyms: None,
                 osyms: None }

    }
}

////////// DEMIT: Implement IntoIterator on State?
impl<W: Weight> IntoIterator for VecState<W> {
    type Item = <VecState<W> as State<W>>::Arc;
    type IntoIter = vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.arcs.into_iter()
    }
}

impl<'a, W: Weight> IntoIterator for &'a VecState<W> {
    type Item = &'a <VecState<W> as State<W>>::Arc;
    type IntoIter = slice::Iter<'a, <VecState<W> as State<W>>::Arc>;
    fn into_iter(self) -> Self::IntoIter {
        (&self.arcs).into_iter()
    }
}



////////////////////////////////////////////////////////////////////////////////
////////// MODULES
////////////////////////////////////////////////////////////////////////////////
pub mod operations;
