extern crate rustc_serialize;

extern crate semiring;
use semiring::*;

pub type Label = usize;
pub type StateId = usize;

#[derive(Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct Arc<W: Weight> {
    ilabel: Label,
    olabel: Label,
    weight: W,
    nextstate: StateId,
}

//Part of the implementation of a mutable FST (VecFST)
#[derive(Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct VecState<W: Weight> {
    id: StateId,
    finalweight: W,
    arcs: Vec<Arc<W>>
}

//A mutable FST using Vec
#[derive(Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct VecFst<W: Weight> {
    states: Vec<VecState<W>>,   //we need to make sure that element indexes don't change once added
    startstate: Option<usize>,
    isyms: Option<Vec<String>>,
    osyms: Option<Vec<String>>
}

//////////////////////////////////////////////////////////////////////
////////////////////// TRAITS AND IMPLEMENTATIONS ////////////////////

////////// ARC
impl<W: Weight> Arc<W> {
    pub fn new(i: Label, o: Label, w: W, s: StateId) -> Arc<W> {
        Arc { ilabel: i,
              olabel: o,
              weight: w,
              nextstate: s }
    }
}


////////// STATE
impl<W: Weight> VecState<W> {
    pub fn new(id: StateId, finalweight: W) -> VecState<W> {
        VecState { id: id,
                   finalweight: finalweight,
                   arcs: Vec::new() }
    }
}

pub trait State {
}

impl<W: Weight> State for VecState<W> {
}


////////// Arc Iterator
#[derive(Debug)]
pub struct VecArcIterator<'a, W: 'a + Weight> {
    state: &'a VecState<W>,
    arcindex: usize
}

impl<'a, W: Weight> Iterator for VecArcIterator<'a, W> {
    type Item = &'a Arc<W>;

    fn next(&mut self) -> Option<&'a Arc<W>> {
        unimplemented!()
    }
}

////////// FST
// This interface is informed by Section 4.1 of:
// Mehryar Mohri, Fernando Pereira, and Michael Riley. "The design
// principles of a weighted finite-state transducer library."
// Theoretical Computer Science 231.1 (2000): 17-32.
pub trait Fst<'a, W: Weight> {
    type I: Iterator;
    fn start_state(&self) -> Option<StateId>;
    fn final_weight(&self, StateId) -> W;
    fn arc_iter(&'a self, StateId) -> Self::I;
}

impl <'a, W: 'a + Weight> Fst<'a, W> for VecFst<W> {
    type I = VecArcIterator<'a, W>;

    fn start_state(&self) -> Option<StateId> {
        self.startstate
    }

    fn final_weight(&self, id: StateId) -> W {
        self.states[id].finalweight //Weight is Copy
    }

    fn arc_iter(&'a self, id: StateId) -> VecArcIterator<'a, W> {
        VecArcIterator { state: &self.states[id],
                         arcindex: 0 }
    }
}

pub trait MutableFst<'a, W: Weight>: Fst<'a, W> {
    fn set_start(&mut self, id: StateId);
    fn add_state(&mut self, finalweight: W) -> StateId;
    fn add_arc(&mut self, source: StateId, target: StateId, ilabel: Label, olabel: Label, weight: W);
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
