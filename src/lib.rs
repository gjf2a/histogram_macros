//! # Overview
//! This library provides a set of macros to enable any `Map`-like type to represent a histogram.
//! Specifically, the type needs the following methods present in the `HashMap<K,V>` and
//! `BTreeMap<K,V>` data types in the standard library:
//!
//! `pub fn insert(&mut self, k: K, v: V) -> Option<V>`
//! `pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V> where K: Borrow<Q>`
//! `pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V> where K: Borrow<Q>`
//! `pub fn iter(&self) -> Iter<'_, K, V>`
//!
//! Because the similarity between `HashMap`, `BTreeMap`, and other similar types is structural,
//! without implementing a common `trait`, using macros enables the creation of common algorithms.
//!
//! ```
//! use histogram_macros::*;
//! use std::collections::HashMap;
//! use std::collections::VecDeque;
//!
//! let mut num_counts: HashMap<isize, usize> = HashMap::new();
//! for i in [100, 200, -100, 200, 300, 200, 100, 200, 100, 300].iter().copied() {
//!     // Use bump! to increase the count of an element by 1.
//!     bump!(num_counts, i);
//! }
//!
//! for (i, c) in [(-100, 1), (100, 3), (200, 4), (300, 2), (400, 0)].iter().copied() {
//!     // Use count! to find how many times an element has been counted.
//!     assert_eq!(count!(num_counts, i), c);
//! }
//!
//! let mut str_counts: HashMap<String, usize> = HashMap::new();
//! for s in ["a", "b", "a", "b", "c", "b", "a", "c", "b"].iter().copied() {
//!     // Use bump_ref! when passing in keys by reference.
//!     bump_ref!(str_counts, s);
//! }
//!
//! // Use bump! when passing concrete values.
//! bump!(str_counts, "d".to_owned());
//!
//! // Bump count by larger values.
//! bump_by!(num_counts, 200, 3);
//! bump_ref_by!(str_counts, "b", 5);
//!
//! for (s, c) in [("a", 3), ("b", 9), ("c", 2), ("d", 1), ("e", 0)].iter().copied() {
//!     // Use count_ref! when checking keys by reference.
//!     assert_eq!(count_ref!(str_counts, s), c);
//! }
//!
//! // Use count! when passing concrete values.
//! assert_eq!(count!(str_counts, "f".to_owned()), 0);
//!
//! // Total counts
//! assert_eq!(total!(num_counts), 13);
//! assert_eq!(total!(str_counts), 15);
//!
//! // Ranked ordering
//! assert_eq!(ranking!(num_counts), VecDeque::from([(200, 7), (100, 3), (300, 2), (-100, 1)]));
//! assert_eq!(ranking!(str_counts), VecDeque::from([("b".to_owned(), 9), ("a".to_owned(), 3), ("c".to_owned(), 2), ("d".to_owned(), 1)]));
//!
//! // Key with the most counts (the mode)
//! assert_eq!(mode!(num_counts).unwrap(), 200);
//! assert_eq!(mode!(str_counts).unwrap(), "b");
//! ```
//!
//! In addition to integer counts, we can use `bump_by!` and `bump_ref_by!` to assign
//! floating-point valued weights. We use `weight!`, `weight_ref!`, `total_weight!`, and
//! `mode_by_weight!` instead of the `count!` macros to look up results.
//!
//! ```
//! use histogram_macros::*;
//! use std::collections::BTreeMap;
//! use std::collections::VecDeque;
//! use ordered_float::OrderedFloat;
//!
//! let mut num_weights: BTreeMap<isize,f64> = BTreeMap::new();
//!
//! for (n, w) in [(1, 0.4), (2, 0.4), (1, 1.6), (3, 0.8)].iter().copied() {
//!     bump_by!(num_weights, n, w);
//! }
//!
//! for (n, w) in [(1, 2.0), (2, 0.4), (3, 0.8)].iter().copied() {
//!     // Use weight! (or weight_ref!) instead of count! (or count_ref!)
//!     assert_eq!(weight!(num_weights, n), w);
//! }
//!
//! // Total weight
//! assert_eq!(total_weight!(num_weights), 3.2);
//!
//! // Most popular (mode), by weight
//! assert_eq!(mode_by_weight!(num_weights).unwrap(), 1);
//!
//! // Ranked by weight
//! assert_eq!(ranking_by_weight!(num_weights), VecDeque::from([(1, OrderedFloat(2.0)), (3, OrderedFloat(0.8)), (2, OrderedFloat(0.4))]));
//! ```
//!
//! Building a histogram from a sequence of values is a common pattern. You can use the
//! `collect_from_into!`, `collect_from_ref_into!`, `collect_from_by_into!`, and
//! `collect_from_ref_by_into!` macros to abstract this pattern.
//!
//! ```
//! use histogram_macros::*;
//! use std::collections::HashMap;
//!
//! let num_counts = collect_from_into!([100, 200, -100, 200, 300, 200, 100, 200, 100, 300]
//!     .iter().copied(), HashMap::<i64, usize>::new());
//! for (i, c) in [(-100, 1), (100, 3), (200, 4), (300, 2), (400, 0)].iter().copied() {
//!     assert_eq!(count!(num_counts, i), c);
//! }
//!
//! let str_counts = collect_from_ref_into!(["a", "b", "a", "b", "c", "b", "a", "c", "b"]
//!     .iter().copied(), HashMap::<String, usize>::new());
//! for (s, c) in [("a", 3), ("b", 4), ("c", 2), ("d", 0)].iter().copied() {
//!     assert_eq!(count_ref!(str_counts, s), c);
//! }
//!
//! let num_weights = collect_from_by_into!([(1, 0.4), (2, 0.4), (1, 1.6), (3, 0.8)].iter().copied(),
//!     HashMap::<isize, f64>::new());
//! for (n, w) in [(1, 2.0), (2, 0.4), (3, 0.8)].iter().copied() {
//!     assert_eq!(weight!(num_weights, n), w);
//! }
//!
//! let str_weights = collect_from_ref_by_into!([("a", 0.4), ("b", 0.2), ("a", 1.2), ("b", 0.8)]
//!     .iter().copied(), HashMap::<String, f64>::new());
//! for (s, w) in [("a", 1.6), ("b", 1.0)].iter().copied() {
//!     assert_eq!(weight_ref!(str_weights, s), w);
//! }
//! ```
//!
//! Alternatively, one can use histogram_struct! to create a custom histogram data type.
//!
//! ```
//! use histogram_macros::histogram_struct;
//!
//! use std::hash::Hash;
//! use std::collections::{HashSet,HashMap};
//! use std::collections::hash_map::Iter;
//!
//! histogram_struct!{HashHistogram, HashHistKey, HashMap, HashSet, Iter, Hash}
//!
//! let mut hist = HashHistogram::new();
//! let zeros = 10;
//! let ones = 15;
//! let twos = 20;
//!
//! for _ in 0..zeros {
//!     hist.bump(&0);
//! }
//!
//! for _ in 0..ones {
//!     hist.bump(&1);
//! }
//!
//! for _ in 0..twos {
//!     hist.bump(&2);
//! }
//!
//! assert_eq!(3, hist.all_labels().len());
//! assert_eq!(zeros, hist.count(&0));
//! assert_eq!(ones, hist.count(&1));
//! assert_eq!(twos, hist.count(&2));
//! assert_eq!(2, hist.mode().unwrap());
//! assert_eq!(zeros + ones + twos, hist.total_count());
//! ```


//    Copyright 2022, Gabriel J. Ferrer
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

// It won't compile without this import, but it issues a warning with it. Using #allow to make the
// warning go away.
#[allow(unused_imports)]
use std::collections::VecDeque;
use std::hash::Hash;
use std::collections::{HashSet,HashMap};
use std::collections::hash_map::Iter;

#[macro_export]
macro_rules! histogram_struct {
    ($name:ident, $keyname:ident, $inner:ident, $labelset:ident, $iter:ident, $constraint:ident) => {
        pub trait $keyname: std::fmt::Debug + $constraint + Clone + Eq {}
        impl <T: std::fmt::Debug + $constraint + Clone + Eq> $keyname for T {}

        pub struct $name<T:$keyname> {
            histogram: $inner<T, usize>
        }

        impl <T:$keyname> $name<T> {
            pub fn new() -> Self { $name { histogram: $inner::new()}}

            pub fn bump(&mut self, item: &T) {
                self.bump_by(item, 1);
            }

            pub fn bump_by(&mut self, item: &T, increment: usize) {
                match self.histogram.get_mut(item) {
                    None => {self.histogram.insert(item.clone(), increment);}
                    Some(count) => {*count += increment;}
                };
            }

            pub fn len(&self) -> usize {
                self.histogram.len()
            }

            pub fn count(&self, item: &T) -> usize {
                *self.histogram.get(item).unwrap_or(&0)
            }

            pub fn iter(&self) -> $iter<T,usize> {
                self.histogram.iter()
            }

            pub fn all_labels(&self) -> $labelset<T> {
                self.iter()
                    .map(|(k, _)| k.clone())
                    .collect()
            }

            pub fn ranking(&self) -> Vec<T> {
                let mut ranking: Vec<(usize,T)> = self.iter().map(|(t, n)| (*n, t.clone())).collect();
                ranking.sort_by_key(|(n,_)| -(*n as isize));
                ranking.iter().map(|(_,t)| t.clone()).collect()
            }

            pub fn mode(&self) -> Option<T> {
                self.iter()
                    .max_by_key(|(_,count)| **count)
                    .map(|(key, _)| key.clone())
            }

            pub fn total_count(&self) -> usize {
                self.iter().map(|(_,value)| value).sum()
            }
        }
    }
}

histogram_struct!{HashHistogram, HashHistKey, HashMap, HashSet, Iter, Hash}

#[macro_export]
macro_rules! bump_skeleton {
    ($d:expr, $kg:expr, $ki:expr, $v:expr) => {
        match $d.get_mut($kg) {
            None => {$d.insert($ki, $v);}
            Some(count) => {*count += $v}
        }
    }
}

#[macro_export]
macro_rules! bump_ref {
    ($d:expr, $k:expr) => {
        bump_ref_by!($d, $k, 1)
    }
}

#[macro_export]
macro_rules! bump_ref_by {
    ($d:expr, $k:expr, $v:expr) => {
        bump_skeleton!($d, $k, $k.to_owned(), $v)
    }
}

#[macro_export]
macro_rules! bump {
    ($d:expr, $k:expr) => {
        bump_by!($d, $k, 1)
    }
}

#[macro_export]
macro_rules! bump_by {
    ($d:expr, $k:expr, $v:expr) => {
        bump_skeleton!($d, &$k, $k, $v)
    }
}

#[macro_export]
macro_rules! count {
    ($d:expr, $k:expr) => {
        count_ref!($d, &$k)
    }
}

#[macro_export]
macro_rules! count_ref {
    ($d:expr, $k:expr) => {
        get_skeleton!($d, $k, 0)
    }
}

#[macro_export]
macro_rules! weight {
    ($d:expr, $k:expr) => {
        weight_ref!($d, &$k)
    }
}

#[macro_export]
macro_rules! weight_ref {
    ($d:expr, $k:expr) => {
        get_skeleton!($d, $k, 0.0)
    }
}

#[macro_export]
macro_rules! get_skeleton {
    ($d:expr, $k:expr, $z:expr) => {
        *($d.get($k).unwrap_or(&$z))
    }
}

#[macro_export]
macro_rules! total_skeleton {
    ($d:expr, $z:expr) => {
        $d.iter().map(|(_,value)| value).copied().reduce(|acc, n| acc + n).unwrap_or($z)
    }
}

#[macro_export]
macro_rules! total {
    ($d:expr) => {total_skeleton!($d, 0)}
}

#[macro_export]
macro_rules! total_weight {
    ($d:expr) => {total_skeleton!($d, 0.0)}
}

#[macro_export]
macro_rules! mode {
    ($d:expr) => {
        $d.iter()
            .max_by_key(|(_,count)| **count)
            .map(|(key, _)| key.clone())
    }
}

#[macro_export]
macro_rules! mode_by_weight {
    ($d:expr) => {
        $d.iter()
            .max_by_key(|(_,count)| ordered_float::OrderedFloat(**count))
            .map(|(key, _)| key.clone())
    }
}

#[macro_export]
macro_rules! ranking_skeleton {
    ($seq:expr) => {
        {
            let mut v = $seq.collect::<Vec<_>>();
            v.sort();
            v.drain(..).rev().map(|(a, b)| (b, a)).collect::<VecDeque<_>>()
        }
    }
}

#[macro_export]
macro_rules! ranking {
    ($d:expr) => {
        ranking_skeleton!($d.iter().map(|(t, n)| (*n, t.clone())))
    }
}

#[macro_export]
macro_rules! ranking_by_weight {
    ($d:expr) => {
        ranking_skeleton!($d.iter().map(|(t, n)| (ordered_float::OrderedFloat(*n), t.clone())))
    }
}

#[macro_export]
macro_rules! collect_from_skeleton {
    ($iter:expr, $d:expr, $b:ident) => {
        {
            let mut result = $d;
            for item in $iter {
                $b!(result, item);
            }
            result
        }
    }
}

#[macro_export]
macro_rules! collect_from_into {
    ($iter:expr, $d:expr) => {
        collect_from_skeleton!($iter, $d, bump)
    }
}

#[macro_export]
macro_rules! collect_from_ref_into {
    ($iter:expr, $d:expr) => {
        collect_from_skeleton!($iter, $d, bump_ref)
    }
}

#[macro_export]
macro_rules! collect_from_by_skeleton {
    ($iter:expr, $d:expr, $b:ident) => {
        {
            let mut result = $d;
            for (k, count) in $iter {
                $b!(result, k, count);
            }
            result
        }
    }
}
#[macro_export]
macro_rules! collect_from_by_into {
    ($iter:expr, $d:expr) => {
        collect_from_by_skeleton!($iter, $d, bump_by)
    }
}

#[macro_export]
macro_rules! collect_from_ref_by_into {
    ($iter:expr, $d:expr) => {
        collect_from_by_skeleton!($iter, $d, bump_ref_by)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, HashMap};

    #[test]
    fn test_str() {
        let mut hist = HashMap::new();
        bump_ref!(hist, "walk");
        bump_ref!(hist, "talk");
        bump_ref!(hist, "walk");
        bump_ref!(hist, "balk");
        assert_eq!(count_ref!(hist, "walk"), 2);
        assert_eq!(count_ref!(hist, "balk"), 1);
        assert_eq!(count_ref!(hist, "talk"), 1);
        assert_eq!(count_ref!(hist, "sulk"), 0);
        assert_eq!(total!(hist), 4);
        assert_eq!(mode!(hist).unwrap(), "walk");
    }

    #[test]
    fn test_string() {
        let mut hist = HashMap::new();
        bump!(hist, "walk".to_owned());
        bump!(hist, "talk".to_owned());
        bump!(hist, "walk".to_owned());
        bump!(hist, "balk".to_owned());
        assert_eq!(count!(hist, "walk".to_owned()), 2);
        assert_eq!(count!(hist, "balk".to_owned()), 1);
        assert_eq!(count!(hist, "talk".to_owned()), 1);
        assert_eq!(count!(hist, "sulk".to_owned()), 0);
        assert_eq!(total!(hist), 4);
    }

    #[test]
    fn test_int() {
        let mut hist = HashMap::new();
        bump!(hist, 6);
        bump!(hist, 5);
        bump!(hist, 6);
        assert_eq!(count!(hist, 4), 0);
        assert_eq!(count!(hist, 5), 1);
        assert_eq!(count!(hist, 6), 2);
        assert_eq!(total!(hist), 3);
        assert_eq!(mode!(hist).unwrap(), 6);

        let r = ranking!(hist);
        println!("{:?}", r);
    }

    #[test]
    fn test_float() {
        let mut hist = HashMap::new();
        bump_ref_by!(hist, "hi", 1.5);
        bump_ref_by!(hist, "bye", 2.6);
        bump_ref_by!(hist, "hi", 0.3);
        assert_eq!(weight_ref!(hist, "hi"), 1.8);
        assert_eq!(weight_ref!(hist, "bye"), 2.6);
        assert_eq!(total_weight!(hist), 4.4);
        assert_eq!(mode_by_weight!(hist).unwrap(), "bye");

        let r = ranking_by_weight!(hist);
        println!("{:?}", r);
    }

    #[test]
    fn test_collect() {
        let h = collect_from_into!([100, 200, -100, 200, 300, 200, 100, 200, 100, 300]
            .iter().copied(), BTreeMap::<i64, usize>::new());
        assert_eq!(format!("{:?}", h), "{-100: 1, 100: 3, 200: 4, 300: 2}");
    }

    #[test]
    fn test_collect_by() {
        let h = collect_from_ref_into!(["a", "b", "a", "b", "c", "b", "a", "c", "b"].iter().copied(), BTreeMap::<String, usize>::new());
        assert_eq!(format!("{:?}", h), r#"{"a": 3, "b": 4, "c": 2}"#);
    }

    #[test]
    fn test_hist() {
        let mut hist = HashHistogram::new();
        let zeros = 10;
        let ones = 15;
        let twos = 20;

        for _ in 0..zeros {
            hist.bump(&0);
        }

        for _ in 0..ones {
            hist.bump(&1);
        }

        for _ in 0..twos {
            hist.bump(&2);
        }

        assert_eq!(3, hist.all_labels().len());
        assert_eq!(zeros, hist.count(&0));
        assert_eq!(ones, hist.count(&1));
        assert_eq!(twos, hist.count(&2));
        assert_eq!(2, hist.mode().unwrap());
        assert_eq!(zeros + ones + twos, hist.total_count());
    }
}
