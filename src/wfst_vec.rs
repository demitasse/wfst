// Author: Daniel van Niekerk <dvn.demitasse@gmail.com>
//
// Copyright 2016 The Department of Arts and Culture of the Government
// of South Africa
//
// See the "LICENCE" file for information on usage and redistribution
// of this file.

//! This module implements a mutable `Fst` using `std::vec::Vec`,
//! interior mutability (`RefCell`) and reference counted pointers
//! (`Rc`) See the source file `main_wfst.rs` for simple examples of
//! intended use.

use super::*;
use super::semiring::Weight;

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

////////// ARC
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
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
        self.weight.clone()
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
#[derive(Clone, Debug, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    type Symtab = Vec<String>;

    fn get_start(&self) -> Option<StateId> {
        self.startstate
    }

    fn get_finalweight(&self, id: StateId) -> W {
        self.states[id].borrow().finalweight.clone()
    }

    fn arc_iter(&self, id: StateId) -> Self::Iter {
        VecArcIterator { state: self.states[id].clone(),
                         arcindex: 0 }
    }

    fn get_isyms(&self) -> Option<Self::Symtab> {
        self.isyms.clone()
    }

    fn get_osyms(&self) -> Option<Self::Symtab> {
        self.osyms.clone()
    }

    fn is_final(&self, id: StateId) -> bool {
        self.get_finalweight(id).ne(&W::zero())
    }
}

impl<W: Weight> MutableFst<W> for VecFst<W> {  
    fn new() -> Self {
        VecFst::new()
    }

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
            let mut to_delete = Vec::<usize>::new();
            let mut state = self.states[i].borrow_mut();
            for j in 0..narcs {
                let nextstate = state.arcs[j].borrow().nextstate;
                if nextstate == id {
                    to_delete.push(j)
                } else if nextstate > id {
                    state.arcs[j].borrow_mut().nextstate -= 1;
                }
            }
            to_delete.sort();
            for j in (0..to_delete.len()).rev() {
                state.arcs.remove(to_delete[j]);
            }
        }
    }

    fn del_states<T: IntoIterator<Item=StateId>>(&mut self, states: T) {
        let mut v: Vec<_> = states.into_iter().collect();
        v.sort();
        for j in (0..v.len()).rev() {
            self.del_state(v[j]);
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

    fn set_isyms<T: IntoIterator<Item=String>>(&mut self, symtab: T) {
        let mut v = Vec::new();
        for s in symtab {
            v.push(s)
        }
        self.isyms = Some(v)
    }

    fn set_osyms<T: IntoIterator<Item=String>>(&mut self, symtab: T) {
        let mut v = Vec::new();
        for s in symtab {
            v.push(s)
        }
        self.osyms = Some(v)
    }

    fn del_isyms(&mut self) {
        self.isyms = None;
    }

    fn del_osyms(&mut self) {
        self.osyms = None;
    }
}

impl<W: Weight> ExpandedFst<W> for VecFst<W> {  
    fn get_numstates(&self) -> usize {
        self.states.len()
    }
}

impl<W: Weight + fmt::Display> fmt::Display for VecFst<W> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn helper<W: Weight + fmt::Display>(f: &VecFst<W>, i: usize, mut s: String) -> String {
            let narcs = f.states[i].borrow().arcs.len();
            let state = f.states[i].borrow();
            for j in 0..narcs {
                s.push_str(&format!("{}\t{}", i, state.arcs[j].borrow().nextstate));
                //String input label if mapping exists (output `_` if map not complete)
                if let Some(ref syms) = f.isyms {
                    s.push_str(&format!("\t{}", syms.get(state.arcs[j].borrow().ilabel).unwrap_or(&String::from("_"))));
                } else {
                    s.push_str(&format!("\t{}", state.arcs[j].borrow().ilabel));
                }
                //String output label if mapping exists (output `_` if map not complete)
                if let Some(ref syms) = f.osyms {
                    s.push_str(&format!("\t{}", syms.get(state.arcs[j].borrow().olabel).unwrap_or(&String::from("_"))));
                } else {
                    s.push_str(&format!("\t{}", state.arcs[j].borrow().olabel));
                }
                //Don't output weight field if equal to W::one
                let weight = &state.arcs[j].borrow().weight;
                if *weight == W::one() {
                    s.push_str(&format!("\n"));
                } else {
                    s.push_str(&format!("\t{}\n", *weight));
                }         
            }
            if f.is_final(i) {
                s.push_str(&format!("{}", i));
                //Don't output weight field if equal to W::one
                let weight = f.get_finalweight(i);
                if weight == W::one() {
                    s.push_str(&format!("\n"));
                } else {
                    s.push_str(&format!("\t{}\n", weight));
                }
            }
            s
        }
        let mut s = String::new();
        //Start state must be in first line
        if let Some(startstate) = self.startstate {
            s = helper(self, startstate, s);
        }
        //Remaining states
        for i in 0..self.states.len() {
            if !self.startstate.is_none() {
                if i == self.startstate.unwrap() {
                    continue
                }
            }
            s = helper(self, i, s);
        }
        write!(f, "{}", s.trim())
    }
}
