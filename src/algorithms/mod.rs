// Author: Daniel van Niekerk <dvn.demitasse@gmail.com>
//
// Copyright 2016 The Department of Arts and Culture of the Government
// of South Africa
//
// See the "LICENCE" file for information on usage and redistribution
// of this file.
//
// This file contains portions of code ported from CMU Sphinx
// (http://cmusphinx.sourceforge.net/) under the following copyright
// and attribution:
//
// """
// Copyright 1999-2012 Carnegie Mellon University.  
// Portions Copyright 2002 Sun Microsystems, Inc.  
// Portions Copyright 2002 Mitsubishi Electric Research Laboratories.
// All Rights Reserved.  Use is subject to license terms.
// 
// See the file "LICENCE.cmusphinx" for information on usage and
// redistribution of this file, and for a DISCLAIMER OF ALL 
// WARRANTIES.
// """
////////////////////////////////////////////////////////////////////////////////

//! This module implements the generic WFST algorithms. See the source
//! files `main_wfst.rs` for simple examples of intended use.

// TODO: We can still improve the implementations here to loosen the
// type requirements, e.g. have `reverse` work for any input fst type.
use super::semiring::{Weight};
use super::{ExpandedFst, MutableFst, StateId, Arc};

use std::vec::Vec;

/// Extends an `Fst` to a single final state.
///  
/// It adds a new final state with a semiring's "one" final weight and
/// connects the current final states to it using epsilon transitions
/// with weight equal to the original final state's weight.
pub fn extendfinal<W: Weight, F: ExpandedFst<W> + MutableFst<W>> (fst: &mut F) {
    //Collect current final states
    let mut finalstates: Vec<StateId> = Vec::new();
    for i in 0..fst.get_numstates() {
        if !fst.get_finalweight(i).eq(&W::zero()) {
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

/// Undo of the `extendfinal` operation.
pub fn unextendfinal<W: Weight, F: ExpandedFst<W> + MutableFst<W>> (fst: &mut F) {
    //Find final state (assuming only one exists)
    let mut finalstate = 0;
    for i in 0..fst.get_numstates() {
        if !fst.get_finalweight(i).eq(&W::zero()) {
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

/// Reverses an `Fst`: If the input fst transduces string x to y with
/// weight a, then the reverse transduces the reverse of x to the
/// reverse of y with weight a.reverse().
pub fn reverse<W: Weight, F: ExpandedFst<W> + MutableFst<W>, O: MutableFst<W>> (ifst: &mut F) -> O {
    extendfinal(ifst);
    //Swap symbol tables
    let mut ofst = O::new();
    if let Some(osyms) = ifst.get_osyms() {
        ofst.set_isyms(osyms);
    }
    if let Some(isyms) = ifst.get_isyms() {
        ofst.set_osyms(isyms);
    }
    //Set start/end states
    for i in 0..ifst.get_numstates() {
        ofst.add_state(W::zero());
        if !ifst.get_finalweight(i).eq(&W::zero()) {
            ofst.set_start(i);
        }
    }
    ofst.set_finalweight(ifst.get_start().unwrap(), W::one());
    //Create reversed arcs
    for i in 0..ifst.get_numstates() {
        for arc in ifst.arc_iter(i) {
            ofst.add_arc(arc.nextstate(), i, arc.ilabel(), arc.olabel(), arc.weight().reverse())
        }
    }    
    unextendfinal(ifst);
    ofst
}

pub mod shortestpath;
