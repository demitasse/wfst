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

//! This module implements the shortest path algorithm described in:
//!
//! Mehryar Mohri and Michael Riley. "An efficient algorithm for the
//! N-best-strings problem," In: *Proceedings of the International
//! Conference on Spoken Language Processing 2002* (ICSLP'02), Denver,
//! Colorado, September 2002.
//!
//! See the source file `example_shortestpath.rs` for a simple example
//! of intended use.

extern crate rustc_serialize;
use self::rustc_serialize::Encodable;
use std::hash::{Hash, Hasher, SipHasher};
extern crate bincode;
use self::bincode::SizeLimit;
use self::bincode::rustc_serialize::encode;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

use super::super::semiring::{Weight, NaturalLess};
use super::super::{Fst, ExpandedFst, MutableFst, StateId, Arc};
use super::super::utils::{LinkedHashSet, ComparatorHeap};
use super::super::wfst_vec::VecFst;
use super::{extendfinal, reverse};

fn hash<T: Hash + Debug>(obj: T) -> u64 {
    let mut hasher = SipHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
    // let a = hasher.finish();
    // println!("{:?}\t{:?}", obj, a);
    // a
}

////////////////////////////////////////////////////////////////////////////////
// (state, weight) tuple with Ord trait implementation for use in
// shortest_paths()
#[derive(Clone, Debug)]
struct Pair<W: Weight + Encodable>(StateId, W);

impl<W: Weight + Encodable> PartialEq for Pair<W> {
    fn eq(&self, rhs: &Self) -> bool {
        hash(self) == hash(rhs)
    }
}
impl<W: Weight + Encodable> Eq for Pair<W> {}
impl<W: Weight + Encodable> Hash for Pair<W> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        encode(&self.1, SizeLimit::Infinite).unwrap().hash(state);
    }
}
////////////////////////////////////////////////////////////////////////////////

/// Calculates the shortest distances from each state to the final
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
    //println!("{:?}", d);
    d
}

/// Calculates the n-best shortest path from the initial to the final state
pub fn shortest_paths<W: Weight + NaturalLess + Encodable, F: ExpandedFst<W> + MutableFst<W>, O: MutableFst<W>> (ifst: &mut F, n: usize, det: bool) -> O {
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
    let compare = |p1: &Pair<W>, p2: &Pair<W>| -> Ordering {
        let a1 = p1.1.times(&d[p1.0]);
        let a2 = p2.1.times(&d[p2.0]);
        if a1.eq(&a2) {  //demit: or use approx_eq()
            Ordering::Equal
        } else if a1.natural_less(&a2) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    };
    extendfinal(ifst);
    let nstates = ifst.get_numstates();
    let mut r: Vec<usize> = Vec::with_capacity(nstates);
    r.resize(nstates, 0);
    
    let mut queue = ComparatorHeap::new(&compare); //Priority Queue
    let mut previous = HashMap::<Pair<W>, Option<Pair<W>>>::with_capacity(nstates);
    let mut statemap = HashMap::<Pair<W>, StateId>::with_capacity(nstates);
    
    let i = ifst.get_start().unwrap();
    let pair = Pair(i, W::one());
    queue.push(pair.clone());
    previous.insert(pair.clone(), None);

    while !queue.is_empty() {
        //println!("{:?}", r);
        // ////
        // let mut v = Vec::new();
        // while !queue.is_empty() {
        //     v.push(queue.pop().unwrap());
        // }
        // for e in &v {
        //     let Pair(p1, c1) = e.clone();
        //     println!("\t{:?} {:?}", p1, c1.times(&d[p1]));
        // }
        // while !v.is_empty() {
        //     queue.push(v.pop().unwrap());
        // }
        // ////
        let pair = queue.pop().unwrap();
        let Pair(p, c) = pair.clone();
        //println!("{:?} {:?}", p, c.times(&d[p]));

        let np = ofst.add_state(ifst.get_finalweight(p));
        statemap.insert(pair.clone(), np);

        let _ppair = previous.get(&pair).unwrap().clone();
        if _ppair.is_none() {
            //this is the start state
            ofst.set_start(np);
        } else {
            //add the incoming arc from previous to current
            let ppair = _ppair.unwrap();
            let pp = *statemap.get(&ppair).unwrap();
            let opp = ppair.0;
            for arc in ifst.arc_iter(opp) {
                if arc.nextstate() == p {
                    ofst.add_arc(pp, np, arc.ilabel(), arc.olabel(), arc.weight());
                }
            }
        }

        r[p] += 1;
        if r[p] == n && ifst.get_finalweight(p).ne(&W::zero()) {
            break;
        }

        if r[p] <= n {
            for arc in ifst.arc_iter(p) {
                let nc = c.times(&arc.weight());
                let npair = Pair(arc.nextstate(), nc);
                previous.insert(npair.clone(), Some(pair.clone()));
                queue.push(npair.clone());
            }
        }
    }
    ofst
}
