extern crate rustc_serialize;

use std::fmt::Debug;
use std::marker::PhantomData;
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

pub trait Fst<W: Weight, A: Arc<W>, S: State<W, A>>: Debug {
    fn get_start(&self) -> Option<StateId>;
    fn get_finalweight(&self, StateId) -> W;       //Weight is Copy
    fn state(&self, StateId) -> Option<&S>;
}

// This interface defined by looking at OpenFST (C++ and Java
// implementations):
pub trait MutableFst<W: Weight, A: Arc<W>, S: State<W, A>>: Fst<W, A, S> {
    fn set_start(&mut self, id: StateId);
    fn add_state(&mut self, finalweight: W) -> StateId;
//    fn del_state(&mut self, StateId);
    fn add_arc(&mut self, source: StateId, target: StateId, ilabel: Label, olabel: Label, weight: W);
    fn set_finalweight(&mut self, id: StateId, finalweight: W);
}

pub trait ExpandedFst<W: Weight, A: Arc<W>, S: State<W, A>>: Fst<W, A, S> {
    fn get_numstates(&self) -> usize;
}

pub trait State<W: Weight, A: Arc<W>>: Debug + IntoIterator {
    fn new(W) -> Self;
    fn get_finalweight(&self) -> W;       //Weight is Copy
    fn set_finalweight(&mut self, finalweight: W);
    fn add_arc(&mut self, ilabel: Label, olabel: Label, weight: W, target: StateId); //DEMIT: Should be MutableState
} 

pub trait Arc<W: Weight>: Clone + Debug {
    fn new(i: Label, o: Label, w: W, s: StateId) -> Self;
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
    fn new(i: Label, o: Label, w: W, s: StateId) -> StdArc<W> {
        StdArc::new(i, o, w, s)
    }

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
pub struct VecState<W: Weight, A: Arc<W>> {
    finalweight: W,
    arcs: Vec<A>
}

impl<W: Weight, A: Arc<W>> VecState<W, A> {
    fn new(finalweight: W) -> VecState<W, A> {
        VecState { finalweight: finalweight,
                   arcs: Vec::new() }
    }
}

impl<W: Weight, A: Arc<W>> State<W, A> for VecState<W, A> {
    fn new(finalweight: W) -> VecState<W, A> {
        VecState::new(finalweight)
    }

    fn get_finalweight(&self) -> W {
        self.finalweight
    }

    fn set_finalweight(&mut self, finalweight: W) {
        self.finalweight = finalweight;
    }

    fn add_arc(&mut self, ilabel: Label, olabel: Label, weight: W, target: StateId) {
        self.arcs.push(A::new(ilabel, olabel, weight, target))
    }

}


////////// FST
#[derive(Clone, Debug, Hash, RustcEncodable, RustcDecodable)]
pub struct VecFst<W: Weight, A: Arc<W>, S: State<W, A>> {
    states: Vec<S>,   //we need to make sure that element indexes are always consistent with arcs
    startstate: Option<usize>,
    isyms: Option<Vec<String>>,
    osyms: Option<Vec<String>>,
    wmarker: PhantomData<W>,
    amarker: PhantomData<A>
}

impl<W: Weight, A: Arc<W>, S: State<W, A>> Fst<W, A, S> for VecFst<W, A, S> {
    fn get_start(&self) -> Option<StateId> {
        self.startstate
    }

    fn get_finalweight(&self, id: StateId) -> W {
        self.states[id].get_finalweight()
    }

    fn state(&self, id: StateId) -> Option<&S> {
        self.states.get(id)
    }
}

impl<W: Weight, A: Arc<W>, S: State<W, A>> MutableFst<W, A, S> for VecFst<W, A, S> {  
    fn set_start(&mut self, id: StateId) {
        assert!(id < self.states.len());
        self.startstate = Some(id);
    }

    fn add_state(&mut self, finalweight: W) -> StateId {
        let id = self.states.len();
        self.states.push(S::new(finalweight));
        id
    }

    // fn del_state(&mut self, id: StateId) {
    //     assert!(id != self.startstate.unwrap());
    //     self.states.remove(id);
    //     //update arcs in remaining states
    //     for i in 0..self.states.len() {
    //         for j in 0..self.states[i].arcs.len() {
    //             if self.states[i].arcs[j].nextstate == id {
    //                 self.states[i].arcs.remove(j);
    //             } else if self.states[i].arcs[j].nextstate > id {
    //                 self.states[i].arcs[j].nextstate -= 1;
    //             }
    //         }
    //     }
    // }

    fn add_arc(&mut self, source: StateId, target: StateId, ilabel: Label, olabel: Label, weight: W) {
        assert!(source < self.states.len());
        assert!(target < self.states.len());
        self.states[source].add_arc(ilabel, olabel, weight, target)
    }

    fn set_finalweight(&mut self, id: StateId, finalweight: W) {
        assert!(id < self.states.len());
        self.states[id].set_finalweight(finalweight);
    }
}

impl<W: Weight, A: Arc<W>, S: State<W, A>> ExpandedFst<W, A, S> for VecFst<W, A, S> {  
    fn get_numstates(&self) -> usize {
        self.states.len()
    }
}


impl<W: Weight, A: Arc<W>, S: State<W, A>> VecFst<W, A, S> {
    pub fn new() -> VecFst<W, A, S> {
        VecFst { states: Vec::new(),
                 startstate: None,
                 isyms: None,
                 osyms: None,
                 wmarker: PhantomData,
                 amarker: PhantomData }

    }
}

//// Arc Iterators
impl<W: Weight, A: Arc<W>> IntoIterator for VecState<W, A> {
    type Item = A;
    type IntoIter = vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        println!("d1");
        self.arcs.into_iter()
    }
}

impl<'a, W: Weight, A: Arc<W>> IntoIterator for &'a VecState<W, A> {
    type Item = &'a A;
    type IntoIter = slice::Iter<'a, A>;
    fn into_iter(self) -> Self::IntoIter {
        println!("d2");
        (&self.arcs).into_iter()
    }
}


//// Aliases
pub type StdFst = VecFst<TropicalWeight<f32>, StdArc<TropicalWeight<f32>>, VecState<TropicalWeight<f32>, StdArc<TropicalWeight<f32>>>>;

////////////////////////////////////////////////////////////////////////////////
////////// MODULES
////////////////////////////////////////////////////////////////////////////////
pub mod operations;
