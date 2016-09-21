// Author: Daniel van Niekerk <dvn.demitasse@gmail.com>
//
// Copyright 2016 The Department of Arts and Culture of the Government
// of South Africa
//
// See the "LICENCE" file for information on usage and redistribution
// of this file.

extern crate linked_hash_map;
use self::linked_hash_map::{Keys, LinkedHashMap};
use std::borrow::Borrow;

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash};

use std::cmp::Ordering;
use std::collections::BinaryHeap;

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

////////////////////////////////////////////////////////////////////////////////
//Originally from https://github.com/rroelke/comparatorheap.rs
struct ComparatorHeapItem<'a, T, F: 'a + Fn(&T, &T) -> Ordering> {
    compare: &'a F,
    item: T
}

impl<'a, T, F: 'a + Fn(&T, &T) -> Ordering> Eq for ComparatorHeapItem<'a, T, F> {}
impl<'a, T, F: 'a + Fn(&T, &T) -> Ordering> PartialEq for ComparatorHeapItem<'a, T, F> {
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Equal => true,
            _ => false
        }
    }
}

impl<'a, T, F: 'a + Fn(&T, &T) -> Ordering> PartialOrd for ComparatorHeapItem<'a, T, F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a, T, F: 'a + Fn(&T, &T) -> Ordering> Ord for ComparatorHeapItem<'a, T, F> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.compare)(&self.item, &other.item)
    }
}

pub struct ComparatorHeap<'a, T, F: 'a + Fn(&T, &T) -> Ordering> {
    compare: &'a F,
    heap: BinaryHeap<ComparatorHeapItem<'a, T, F>>
}

impl<'a, T, F: Fn(&T, &T) -> Ordering> ComparatorHeap<'a, T, F> {
    pub fn new(cmp: &'a F) -> Self {
        ComparatorHeap {
            compare: cmp,
            heap: BinaryHeap::new()
        }
    }

    pub fn with_capacity(cmp: &'a F, capacity: usize) -> Self {
        ComparatorHeap {
            compare: cmp,
            heap: BinaryHeap::with_capacity(capacity)
        }
    }

    pub fn capacity(&self) -> usize {
        self.heap.capacity()
    }

    pub fn reserve_exact(&mut self, n: usize) {
        self.heap.reserve_exact(n)
    }

    pub fn reserve(&mut self, n: usize) {
        self.heap.reserve(n)
    }

    pub fn peek<'b>(&'b self) -> Option<&'b T> {
        match self.heap.peek() {
            None => None,
            Some(item) => Some(&item.item)
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.heap.pop() {
            None => None,
            Some(item) => Some(item.item)
        }
    }

    pub fn push(&mut self, item: T) {
        self.heap.push(ComparatorHeapItem {
            compare: self.compare,
            item: item
        })
    }

    // /// pushes an item onto a queue and then pops the greatest item
    // /// off in an optimized fashion
    // pub fn push_pop(&mut self, item: T) -> T {
    //     self.heap.push_pop(ComparatorHeapItem {
    //         compare: self.compare,
    //         item: item
    //     }).item
    // }

    // /// pops the greatest item off a queue then pushes an item
    // /// onto the queue in an optimized fashion
    // pub fn replace(&mut self, item: T) -> Option<T> {
    //     match self.heap.replace(ComparatorHeapItem {
    //         compare: self.compare,
    //         item: item
    //     }) {
    //         None => None,
    //         Some(item) => Some(item.item)
    //     }
    // }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn clear(&mut self) {
        self.heap.clear()
    }

    /// consumes the ComparatorHeap and returns the underlying
    /// vector in arbitrary order.
    pub fn into_vec(self) -> Vec<T> {
        self.heap.into_vec().
                into_iter().map(|item: ComparatorHeapItem<T, F>| -> T {
            item.item
        }).collect()
    }

    /// consumes the ComparatorHeap and returns a vector in sorted
    /// (ascending) order
    pub fn into_sorted_vec(self) -> Vec<T> {
        self.heap.into_sorted_vec().
                into_iter().map(|item: ComparatorHeapItem<T, F>| -> T {
            item.item
        }).collect()
    }
}
