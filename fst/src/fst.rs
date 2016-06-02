extern crate rustc_serialize;

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

pub trait Fst<'a, W: Weight> {
    type I: Iterator;
    fn get_start(&self) -> Option<StateId>;
    fn get_finalweight(&self, StateId) -> W;       //Weight is Copy
    fn arc_iter(&'a self, StateId) -> Self::I;
}

// This interface defined by looking at OpenFST (C++ and Java
// implementations):
pub trait MutableFst<'a, W: Weight>: Fst<'a, W> {
    fn set_start(&mut self, id: StateId);
    fn add_state(&mut self, finalweight: W) -> StateId;
    fn add_arc(&mut self, source: StateId, target: StateId, ilabel: Label, olabel: Label, weight: W);
    fn set_finalweight(&mut self, id: StateId, finalweight: W);
}

pub trait ExpandedFst<'a, W: Weight>: Fst<'a, W> {
    fn get_numstates(&self) -> usize;
}

pub trait Arc<W: Weight> {
    fn ilabel(&self) -> Label;
    fn olabel(&self) -> Label;
    fn weight(&self) -> W;
    fn nextstate(&self) -> StateId;
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
struct VecState<W: Weight> {
    finalweight: W,
    arcs: Vec<StdArc<W>>
}

impl<W: Weight> VecState<W> {
    fn new(finalweight: W) -> VecState<W> {
        VecState { finalweight: finalweight,
                   arcs: Vec::new() }
    }
}

////////// FST
#[derive(Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct VecFst<W: Weight> {
    states: Vec<VecState<W>>,   //we need to make sure that element indexes don't change once added
    startstate: Option<usize>,
    isyms: Option<Vec<String>>,
    osyms: Option<Vec<String>>
}

impl <'a, W: 'a + Weight> Fst<'a, W> for VecFst<W> {
    type I = VecArcIterator<'a, W>;

    fn get_start(&self) -> Option<StateId> {
        self.startstate
    }

    fn get_finalweight(&self, id: StateId) -> W {
        self.states[id].finalweight
    }

    fn arc_iter(&'a self, id: StateId) -> VecArcIterator<'a, W> {
        VecArcIterator { state: &self.states[id],
                         arcindex: None }
    }
}

impl <'a, W: 'a + Weight> MutableFst<'a, W> for VecFst<W> {  
    fn set_start(&mut self, id: StateId) {
        assert!(id < self.states.len());
        self.startstate = Some(id);
    }

    fn add_state(&mut self, finalweight: W) -> StateId {
        let id = self.states.len();
        self.states.push(VecState::new(finalweight));
        id
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

impl <'a, W: 'a + Weight> ExpandedFst<'a, W> for VecFst<W> {  
    fn get_numstates(&self) -> usize {
        self.states.len()
    }
}


impl <W: Weight> VecFst<W> {
    pub fn new() -> VecFst<W> {
        VecFst { states: Vec::new(),
                 startstate: None,
                 isyms: None,
                 osyms: None }

    }
}

////////// ARCITERATOR
#[derive(Debug)]
pub struct VecArcIterator<'a, W: 'a + Weight> {
    state: &'a VecState<W>,
    arcindex: Option<usize>
}

impl<'a, W: Weight> Iterator for VecArcIterator<'a, W> {
    type Item = &'a StdArc<W>;

    fn next(&mut self) -> Option<&'a StdArc<W>> {
        self.arcindex =
            if self.arcindex.is_none() {
                Some(0)
            } else {
                Some(self.arcindex.unwrap() + 1)
            };
        if self.arcindex.unwrap() < self.state.arcs.len() {
            Some(&self.state.arcs[self.arcindex.unwrap()])
        } else {
            None
        }
    }
}
