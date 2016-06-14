extern crate semiring;
//extern crate fst;

use std::vec::Vec;

use semiring::Weight;
use {Fst, ExpandedFst, MutableFst, StateId, Arc};

pub fn extendfinal<'a, W: Weight + 'a, F: ExpandedFst<'a, W> + MutableFst<'a, W>> (fst: &mut F) {
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

pub fn unextendfinal<'a, W: Weight + 'a, F: ExpandedFst<'a, W> + MutableFst<'a, W>> (fst: &'a mut F)
    where <<F as Fst<'a, W>>::Iter as Iterator>::Item: Arc<W>
{
    //Find final state (assuming only one exists)
    let mut finalstate = 0;
    for i in 0..fst.get_numstates() {
        if !fst.get_finalweight(i).eq(W::zero()) {
            finalstate = i;
            break
        }
    }
    
    let a = fst.arc_iter(1).collect::<Vec<_>>(); //DEMIT: something to do with "autoref" lifetime in sig (fst: &'a mut F)
    for arc in a {
        println!("ARC:{:?}", arc);
        let s3 = fst.add_state(W::zero());
        let s4 = fst.add_state(W::zero());
        fst.add_arc(s3, s4, 0, 0, W::zero());
    }


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
