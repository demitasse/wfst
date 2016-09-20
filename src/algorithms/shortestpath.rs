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

//! This module implements the shortest path algorithms. See the source
//! files `main_wfst.rs` for simple examples of intended use.

use std::collections::{HashMap, BinaryHeap};

use super::super::semiring::{Weight};
use super::super::{Fst, ExpandedFst, MutableFst, StateId, Arc};
use super::super::utils::{RevOrd, LinkedHashSet};
use super::super::wfst_vec::VecFst;
use super::{extendfinal, reverse};




//DEMIT TODO:
//     -- Implement Ord on (StateId, Weight + NaturalLess)
//     -- Implement NaturalLess trait in semiring modules instead of Ord
//     -- Implement shortest_paths and test in main_wfst.rs

pub fn shortest_distance<W: Weight, F: ExpandedFst<W> + MutableFst<W>> (ifst: &mut F) -> Vec<W> {
    let revfst: VecFst<_> = reverse(ifst);
    let nstates = revfst.get_numstates();

    let mut d: Vec<W> = Vec::with_capacity(nstates);
    let mut r: Vec<W> = Vec::with_capacity(nstates);
    d.resize(nstates, W::zero());
    r.resize(nstates, W::zero());
    
    let mut queue = LinkedHashSet::new();
    queue.insert(revfst.get_start().unwrap());

    d[revfst.get_start().unwrap()] = W::one();
    r[revfst.get_start().unwrap()] = W::one();

    while !queue.is_empty() {
        let s = queue.pop_front().unwrap();
        //println!("{:?}", r[s]);
        
        let rnew = r[s].clone();
        r[s] = W::zero();

        for arc in revfst.arc_iter(s) {
            let nexts = arc.nextstate();
            //println!("\t{:?}", nexts);
            let dnext = d[nexts].clone();
            let dnextnew = dnext.plus(&rnew.times(&arc.weight()));
            //println!("\t\t{:?} {:?}", dnext, dnextnew);
            
            if dnext.ne(&dnextnew) {
                d[nexts] = dnextnew;
                r[nexts] = r[nexts].plus(&rnew.times(&arc.weight()));
                if !queue.contains(&nexts) {
                    queue.insert(nexts);
                }
            }
            
        }
    }
    d
}

pub fn shortest_paths<W: Weight + Ord, F: ExpandedFst<W> + MutableFst<W>, O: MutableFst<W>> (ifst: &mut F, n: usize, det: bool) -> O {
    let orignstates = ifst.get_numstates();
    let ifst = if det {
        println!("Determinize not yet implemented!");
        ifst
    } else {
        ifst
    };

    //Create output Fst and copy symbol tables
    let mut ofst = O::new();
    if let Some(osyms) = ifst.get_osyms() {
        ofst.set_osyms(osyms);
    }
    if let Some(isyms) = ifst.get_isyms() {
        ofst.set_isyms(isyms);
    }
    
    let d = shortest_distance(ifst);
    extendfinal(ifst);
    let nstates = ifst.get_numstates();
    let mut r: Vec<usize> = Vec::with_capacity(nstates);
    
    let mut queue = BinaryHeap::new(); //Priority Queue

    //let mut previous = HashMap::with_capacity(orignstates);
    //let mut statemap = HashMap::with_capacity(orignstates);


    //tmp:
    queue.push(RevOrd(W::zero()));



    unimplemented!()
}

