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
//! ```
//! use std::collections::HashMap;
//! use histogram_macros::*;
//!
//! let mut num_counts: HashMap<isize, usize> = HashMap::new();
//! for i in [100, 200, -100, 200, 300, 200, 100, 200, 100, 300].iter().copied() {
//!     bump!(num_counts, i);
//! }
//!
//! for (i, c) in [(-100, 1), (100, 3), (200, 4), (300, 2)].iter().copied() {
//!     assert_eq!(count!(num_counts, i), c);
//! }
//!
//! // Record and inspect histogram counts.
//! let mut str_counts: HashMap<String, usize> = HashMap::new();
//! for s in ["a", "b", "a", "b", "c", "b", "a", "c", "b"].iter().copied() {
//!     // Use bump_ref! when passing in keys by reference.
//!     bump_ref!(str_counts, s);
//! }
//!
//! // Use bump! when passing concrete values.
//! bump!(str_counts, "d".to_owned());
//!
//! for (s, c) in [("a", 3), ("b", 4), ("c", 2), ("d", 1), ("e", 0)].iter().copied() {
//!     // Use count_ref! when checking keys by reference.
//!     assert_eq!(count_ref!(str_counts, s), c);
//! }
//!
//! // Use count! when passing concrete values.
//! assert_eq!(count!(str_counts, "f".to_owned()), 0);
//!
//! // Total counts
//! assert_eq!(total!(num_counts), 10);
//! assert_eq!(total!(str_counts), 10);
//!
//! // Ranked ordering
//! assert_eq!(ranking!(num_counts), vec![200, 100, 300, -100]);
//! assert_eq!(ranking!(str_counts), vec!["b", "a", "c", "d"]);
//!
//! // Key with the most counts (the mode)
//! assert_eq!(mode!(num_counts).unwrap(), 200);
//! assert_eq!(mode!(str_counts).unwrap(), "b");
//! ```
//!


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
macro_rules! rankify {
    ($r:expr) => {
        {
            $r.sort();
            $r.drain(..).rev().map(|(_,t)| t).collect::<Vec<_>>()
        }
    }
}

#[macro_export]
macro_rules! ranking {
    ($d:expr) => {
        {
            let mut ranking = $d.iter().map(|(t, n)| (*n, t.clone())).collect::<Vec<_>>();
            rankify!(ranking)
        }
    }
}

#[macro_export]
macro_rules! ranking_by_weight {
    ($d:expr) => {
        {
            let mut ranking = $d.iter().map(|(t, n)| (ordered_float::OrderedFloat(*n), t.clone())).collect::<Vec<_>>();
            rankify!(ranking)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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
    fn old_test() {
        let observations = ["a", "b", "a", "b", "c", "b", "a", "b"];
        let mut h: HashMap<String, i32> = HashMap::new();
        for s in observations.iter().copied() {
            bump_ref!(h, s);
        }

        for (s, c) in [("a", 3), ("b", 4), ("c", 1), ("d", 0)].iter().copied() {
            assert_eq!(count_ref!(h, s), c);
        }

        assert_eq!(total!(h), 8);
        assert_eq!(ranking!(h), vec!["b", "a", "c"]);
        assert_eq!(mode!(h).unwrap(), "b");
    }
}
