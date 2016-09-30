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

//! This implements functions to remove unsuccessful paths from an
//! fst.

// This module is ported from the Java which uses mutable function
// arguments instead of clear return values... :-/

use std::collections::HashSet;
use std::vec::Vec;

use super::super::semiring::{Weight};
use super::super::{Fst, ExpandedFst, MutableFst, StateId, Arc};

/// Copies a path
fn duplicate_path(last_path_idx: usize, from: StateId, to: StateId, paths: &mut Vec<Vec<StateId>>) {
    let new_path = {
        let last_path = paths.get(last_path_idx).unwrap();
        let from_idx = last_path.iter().position(|&x| x == from).unwrap();
        let to_idx = last_path.iter().position(|&x| x == to).unwrap_or(last_path.len() - 1);
        last_path[from_idx..to_idx].to_vec()
    };
    paths.push(new_path);
}

/// Adds an arc top the explored arcs list
fn add_explored_arc<W: Weight, A: Arc<W>> (start: StateId, explored_arcs: &mut Vec<Option<Vec<A>>>, arc: A) {
    if explored_arcs[start].is_none() {
        explored_arcs[start] = Some(Vec::new());
    }
    explored_arcs.get_mut(start).unwrap().as_mut().unwrap().push(arc);
}


/// Calculates the coaccessible states of an fst
fn calc_coaccessible<W: Weight, F: Fst<W>> (fst: &F, state: StateId, paths: &mut Vec<Vec<StateId>>, coaccessible: &mut HashSet<StateId>) {
    //hold the coaccessible added in this loop
    let mut new_coaccessibles = Vec::<StateId>::new();
    for path in paths.iter() {
        if let Some(index) = path.iter().rposition(|&x| x == state) {
            if fst.is_final(state) || coaccessible.contains(&state) {
                for i in (0..index+1).rev() {
                    if !coaccessible.contains(&path[i]) {
                        new_coaccessibles.push(path[i]);
                        coaccessible.insert(path[i]);
                    }
                }
            }
        }
    }

    //run again for the new coaccessibles
    for s in new_coaccessibles {
        calc_coaccessible(fst, s, paths, coaccessible);
    }
}

/// The depth first search recursion
fn dfsnext<W: Weight, F: ExpandedFst<W>> (fst: &F, start: StateId, paths: &mut Vec<Vec<StateId>>, explored_arcs: &mut Vec<Option<Vec<F::Arc>>>, accessible: &mut HashSet<StateId>) {
    let mut last_path_idx = paths.len() - 1;
    paths.get_mut(last_path_idx).unwrap().push(start);

    let mut arccount: usize = 0;
    for arc in fst.arc_iter(start) {
        if explored_arcs[start].is_none() || !&explored_arcs[start].as_ref().unwrap().contains(&arc) {
            last_path_idx = paths.len() - 1;
            arccount += 1;
            if arccount > 1 {
                duplicate_path(last_path_idx, fst.get_start().unwrap(), start, paths);
                last_path_idx = paths.len() - 1;
                paths.get_mut(last_path_idx).unwrap().push(start);
            }
            let ns = arc.nextstate();
            add_explored_arc(start, explored_arcs, arc.clone());
            //detect self-loops
            if ns != start {
                dfsnext(fst, ns, paths, explored_arcs, accessible);
            }
        }
    }
    accessible.insert(start);
}


/// Initialization of a depth first search recursion
fn dfs<W: Weight, F: ExpandedFst<W>> (fst: &F) -> (HashSet<StateId>, HashSet<StateId>) {
    let nstates = fst.get_numstates();
    let mut accessible = HashSet::<StateId>::new();
    let mut coaccessible = HashSet::<StateId>::new();
    let mut explored_arcs = Vec::<Option<Vec<F::Arc>>>::new();
    explored_arcs.resize(nstates, None);
    let mut paths = Vec::<Vec<StateId>>::new();
    paths.push(Vec::new());

    let currstate = fst.get_start().unwrap();
    
    if !accessible.contains(&currstate) {
        dfsnext(fst, currstate, &mut paths, &mut explored_arcs, &mut accessible);
    }

    for i in 0..fst.get_numstates() {
        if fst.is_final(i) {
            calc_coaccessible(fst, i, &mut paths, &mut coaccessible);
        }
    }

    (accessible, coaccessible)
}


/// Trims an fst, removing states and arcs that are not on a
/// successful path
pub fn connect<W: Weight, F: ExpandedFst<W> + MutableFst<W>> (mut fst: F) -> F {
    let (accessible, coaccessible) = dfs(&fst);
    let mut to_delete = Vec::<StateId>::new();

    for i in 0..fst.get_numstates() {
        if !accessible.contains(&i) || !coaccessible.contains(&i) {
            to_delete.push(i);
        }
    }
    fst.del_states(to_delete);
    fst
}
