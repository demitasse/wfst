extern crate semiring;

use std::vec::Vec;

use semiring::Weight;
use super::{ExpandedFst, MutableFst, StateId, Arc};


/// Extends an Fst to a single final state.
///  
/// It adds a new final state with a semiring's one final weight and
/// connects the current final states to it using epsilon transitions with
/// weight equal to the original final state's weight.
pub fn extendfinal<W: Weight, F: ExpandedFst<W> + MutableFst<W>> (fst: &mut F) {
    //Collect current final states
    let mut finalstates: Vec<StateId> = Vec::new();
    for i in 0..fst.get_numstates() {
        if !fst.get_finalweight(i).eq(W::zero()) {
            finalstates.push(i)
        }
    }

    //Add single new final state and link with epsilon transitions
    let newfinal = fst.add_state(W::one());
    for state in finalstates {
        let finalweight = fst.get_finalweight(state);
        fst.add_arc(state, newfinal, 0, 0, finalweight);
        fst.set_finalweight(state, W::zero());
    }
}

pub fn unextendfinal<W: Weight, F: ExpandedFst<W> + MutableFst<W>> (fst: &mut F) {
    //Find final state (assuming only one exists)
    let mut finalstate = 0;
    for i in 0..fst.get_numstates() {
        if !fst.get_finalweight(i).eq(W::zero()) {
            finalstate = i;
            break
        }
    }

    //Transfer finalweight from final arcs to new final states
    for i in 0..fst.get_numstates() {
        for arc in fst.arc_iter(i) {
            if arc.ilabel() == 0 && arc.olabel() == 0 && arc.nextstate() == finalstate {
                fst.set_finalweight(i, arc.weight());
            }
        }
    }
    fst.del_state(finalstate);
}
