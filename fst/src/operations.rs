extern crate semiring;

use std::vec::Vec;

use semiring::Weight;
use {Fst, ExpandedFst, MutableFst, StateId, Arc, ArcIterator};

pub fn extendfinal<'a, W: Weight, F: ExpandedFst<'a, W> + MutableFst<'a, W>> (fst: &mut F) {
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

pub fn unextendfinal<'a, W: Weight, F: ExpandedFst<'a, W> + MutableFst<'a, W>> (fst: &mut F)
    where F::Iter: ArcIterator<'a, W, F> {
    //Find final state (assuming only one exists)
    let mut finalstate = 0;
    for i in 0..fst.get_numstates() {
        if !fst.get_finalweight(i).eq(W::zero()) {
            finalstate = i;
            break
        }
    }
    
    let aa = F::Iter::new(fst, 1);
    

    // //Transfer finalweight from final arcs to new final states
    // for i in 0..fst.get_numstates() {
    //     let arcs = fst.state(i).unwrap().into_iter().collect::<Vec<_>>();
    //     for arc in arcs {
    //         //let a: i32 = arc;
    //         if arc.ilabel() == 0 && arc.olabel() == 0 && arc.nextstate() == finalstate {
    //             //fst.set_finalweight(i, arc.weight());
    //         }
    //     }
    // }
    // fst.state(0);//del_state(finalstate);
    
}
