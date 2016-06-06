extern crate semiring;
//extern crate fst;

use std::vec::Vec;

use semiring::Weight;
use {ExpandedFst, MutableFst, StateId, Arc, State};

pub fn extendfinal<W: Weight, A: Arc<W>, S: State<W, A>, T: ExpandedFst<W, A, S> + MutableFst<W, A, S>> (fst: &mut T) {
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

pub fn unextendfinal<W: Weight, A: Arc<W>, S: State<W, A>, T: ExpandedFst<W, A, S> + MutableFst<W, A, S>> (fst: &mut T) {
    //Find final state (assuming only one exists)
    let mut finalstate = 0;
    for i in 0..fst.get_numstates() {
        if !fst.get_finalweight(i).eq(W::zero()) {
            finalstate = i;
            break
        }
    }
    
    {
        let borrowed_state = fst.state(0).unwrap();
        println!("..........{:?}", borrowed_state);
        println!("");
        println!("..........{:?}", finalstate);
        println!("");
    }


    //Transfer finalweight from final arcs to new final states
    // for i in 0..fst.get_numstates() {
    //     for arc in fst.arc_iter(i).cloned().collect::<Vec<_>>() {
    //         if arc.ilabel() == 0 && arc.olabel() == 0 && arc.nextstate() == finalstate {
    //             fst.set_finalweight(i, arc.weight());
    //         }
    //     }
    // }
    //fst.del_state(finalstate);
    
}
