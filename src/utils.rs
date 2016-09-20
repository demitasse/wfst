// Author: Daniel van Niekerk <dvn.demitasse@gmail.com>
//
// Copyright 2016 The Department of Arts and Culture of the Government
// of South Africa
//
// See the "LICENCE" file for information on usage and redistribution
// of this file.

extern crate linked_hash_map;
use self::linked_hash_map::{Keys, LinkedHashMap};

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash};
use std::cmp::Ordering;
use std::borrow::Borrow;

////////////////////////////////////////////////////////////////////////////////
//Reverse ordering
#[derive(Debug)]
pub struct RevOrd<T>(pub T);

impl<T:PartialEq> PartialEq for RevOrd<T> {
  fn eq(&self, other: &Self) -> bool {
    other.0 == self.0
  }
}
impl<T: Eq> Eq for RevOrd<T> {}

impl<T: Ord> Ord for RevOrd<T> {
    fn cmp(&self, other: &Self) -> Ordering {
      other.0.cmp(&self.0)
    }
}

impl<T: PartialOrd> PartialOrd for RevOrd<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    (other.0).partial_cmp(&self.0)
  }
}
////////////////////////////////////////////////////////////////////////////////

// http://stackoverflow.com/questions/37550208/a-set-type-that-preserves-insertion-order
pub struct LinkedHashSet<K, S = RandomState>(LinkedHashMap<K, (), S>);

impl<K: Hash + Eq> LinkedHashSet<K> {
    pub fn new() -> Self {
        LinkedHashSet(LinkedHashMap::new())
    }
}

impl<K: Hash + Eq, S: BuildHasher> LinkedHashSet<K, S> {
    pub fn insert(&mut self, k: K) -> Option<()> {
        self.0.insert(k, ())
    }

    pub fn contains<Q: ?Sized>(&self, k: &Q) -> bool
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.0.contains_key(k)
    }

    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<()>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.0.remove(k)
    }

    pub fn iter(&self) -> Keys<K, ()> {
        self.0.keys()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn pop_front(&mut self) -> Option<K> {
        if let Some((k, _)) = self.0.pop_front() {
            Some(k) 
        } else {
            None
        }
    }
}
