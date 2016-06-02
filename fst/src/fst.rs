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
pub type StateId = usize;

pub trait Fst<'a, W: Weight> {
    type I: Iterator;
    fn start_state(&self) -> Option<StateId>;
    fn final_weight(&self, StateId) -> W;       //Weight is Copy
    fn arc_iter(&'a self, StateId) -> Self::I;
}

// This interface defined by looking at OpenFST (C++ and Java
// implementations?):
pub trait MutableFst<'a, W: Weight>: Fst<'a, W> {
    fn set_start(&mut self, id: StateId);
    fn add_state(&mut self, finalweight: W) -> StateId;
    fn add_arc(&mut self, source: StateId, target: StateId, ilabel: Label, olabel: Label, weight: W);
    //set_final? ...
}

////////////////////////////////////////////////////////////////////////////////
////////// FST IMPLEMENTATION 1: A Mutable FST using Vectors
////////////////////////////////////////////////////////////////////////////////

////////// ARC
pub type Label = usize;

#[derive(Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct Arc<W: Weight> {
    ilabel: Label,
    olabel: Label,
    weight: W,
    nextstate: StateId,
}

impl<W: Weight> Arc<W> {
    pub fn new(i: Label, o: Label, w: W, s: StateId) -> Arc<W> {
        Arc { ilabel: i,
              olabel: o,
              weight: w,
              nextstate: s }
    }
}

////////// STATE
#[derive(Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
struct VecState<W: Weight> {
    id: StateId,
    finalweight: W,
    arcs: Vec<Arc<W>>
}

impl<W: Weight> VecState<W> {
    fn new(id: StateId, finalweight: W) -> VecState<W> {
        VecState { id: id,
                   finalweight: finalweight,
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

    fn start_state(&self) -> Option<StateId> {
        self.startstate
    }

    fn final_weight(&self, id: StateId) -> W {
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
        self.states.push(VecState::new(id, finalweight));
        id
    }

    fn add_arc(&mut self, source: StateId, target: StateId, ilabel: Label, olabel: Label, weight: W) {
        assert!(source < self.states.len());
        assert!(target < self.states.len());
        self.states[source].arcs.push(Arc::new(ilabel, olabel, weight, target));
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

////////// ARCITERATOR
#[derive(Debug)]
pub struct VecArcIterator<'a, W: 'a + Weight> {
    state: &'a VecState<W>,
    arcindex: Option<usize>
}

impl<'a, W: Weight> Iterator for VecArcIterator<'a, W> {
    type Item = &'a Arc<W>;

    fn next(&mut self) -> Option<&'a Arc<W>> {
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
