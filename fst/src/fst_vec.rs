use super::*;
use super::semiring::Weight;

use std::rc::Rc;
use std::cell::RefCell;

////////// ARC
#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct StdArc<W: Weight> {
    ilabel: Label,
    olabel: Label,
    weight: W,
    nextstate: StateId,
}

impl<W: Weight> StdArc<W> {
    pub fn new(i: Label, o: Label, w: W, s: StateId) -> Self {
        StdArc { ilabel: i,
                 olabel: o,
                 weight: w,
                 nextstate: s }
    }
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

impl<W: Weight> Arc<W> for Rc<RefCell<StdArc<W>>> {
    fn ilabel(&self) -> Label {
        self.borrow().ilabel()
    }
    fn olabel(&self) -> Label {
        self.borrow().olabel()
    }
    fn weight(&self) -> W {
        self.borrow().weight()
    }
    fn nextstate(&self) -> StateId {
        self.borrow().nextstate()
    }
}


#[derive(Debug)]
pub struct VecArcIterator<W: Weight> {
    state: Rc<RefCell<VecState<W>>>,
    arcindex: usize
}

impl<W: Weight> Iterator for VecArcIterator<W> {
    type Item = Rc<RefCell<StdArc<W>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.arcindex;
        self.arcindex += 1;
        let state = self.state.borrow();
        if i < state.arcs.len() {
            Some(state.arcs[i].clone())
        } else {
            None
        }
    }
}


////////// STATE
#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct VecState<W: Weight> {
    finalweight: W,
    arcs: Vec<Rc<RefCell<StdArc<W>>>>
}

impl<W: Weight> VecState<W> {
    fn new(finalweight: W) -> VecState<W> {
        VecState { finalweight: finalweight,
                   arcs: Vec::new() }
    }
}

////////// FST
#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct VecFst<W: Weight> {
    states: Vec<Rc<RefCell<VecState<W>>>>,   //we need to make sure that element indexes are always consistent with arcs
    startstate: Option<usize>,
    isyms: Option<Vec<String>>,
    osyms: Option<Vec<String>>,
}

impl<W: Weight> VecFst<W> {
    pub fn new() -> Self {
        VecFst { states: Vec::new(),
                 startstate: None,
                 isyms: None,
                 osyms: None }
    }
}

impl<W: Weight> Fst<W> for VecFst<W> {
    type Arc = Rc<RefCell<StdArc<W>>>;
    type Iter = VecArcIterator<W>;

    fn get_start(&self) -> Option<StateId> {
        self.startstate
    }

    fn get_finalweight(&self, id: StateId) -> W {
        self.states[id].borrow().finalweight
    }

    fn arc_iter(&self, id: StateId) -> Self::Iter {
        VecArcIterator { state: self.states[id].clone(),
                         arcindex: 0 }
    }
}

impl<W: Weight> MutableFst<W> for VecFst<W> {  
    fn set_start(&mut self, id: StateId) {
        assert!(id < self.states.len());
        self.startstate = Some(id);
    }

    fn add_state(&mut self, finalweight: W) -> StateId {
        let id = self.states.len();
        self.states.push(Rc::new(RefCell::new(VecState::new(finalweight))));
        id
    }

    fn del_state(&mut self, id: StateId) {
        assert!(id != self.startstate.unwrap());
        self.states.remove(id);
        //update arcs in remaining states
        for i in 0..self.states.len() {
            let narcs = self.states[i].borrow().arcs.len();
            for j in 0..narcs {
                let mut state = self.states[i].borrow_mut();
                let nextstate = state.arcs[j].borrow().nextstate;
                if nextstate == id {
                    state.arcs.remove(j);
                } else if nextstate > id {
                    state.arcs[j].borrow_mut().nextstate -= 1;
                }
            }
        }
    }

    fn add_arc(&mut self, source: StateId, target: StateId, ilabel: Label, olabel: Label, weight: W) {
        assert!(source < self.states.len());
        assert!(target < self.states.len());
        self.states[source]
            .borrow_mut()
            .arcs
            .push(Rc::new(RefCell::new(StdArc::new(ilabel, olabel, weight, target))))
    }

    fn set_finalweight(&mut self, id: StateId, finalweight: W) {
        assert!(id < self.states.len());
        self.states[id].borrow_mut().finalweight = finalweight;
    }
}

impl<W: Weight> ExpandedFst<W> for VecFst<W> {  
    fn get_numstates(&self) -> usize {
        self.states.len()
    }
}
