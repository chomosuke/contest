#![allow(unused_imports, dead_code)]
use std::any::type_name;
use std::cmp::*;
use std::collections::*;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::stdin;
use std::ops::Bound::*;
use std::ops::Deref;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Range;
use std::ops::RangeBounds;
use std::str::FromStr;

struct Scanner {
    tokens: VecDeque<String>,
    delimiters: Option<HashSet<char>>,
}
impl Scanner {
    fn new() -> Scanner {
        Scanner {
            tokens: VecDeque::new(),
            delimiters: None,
        }
    }

    fn with_delimiters(delimiters: &[char]) -> Scanner {
        Scanner {
            tokens: VecDeque::new(),
            delimiters: Some(delimiters.iter().copied().collect()),
        }
    }

    fn next<T: FromStr>(&mut self) -> T {
        let token = loop {
            let front = self.tokens.pop_front();
            if let Some(token) = front {
                break token;
            }
            self.receive_input();
        };
        token
            .parse::<T>()
            .unwrap_or_else(|_| panic!("input {} isn't a {}", token, type_name::<T>()))
    }

    fn receive_input(&mut self) {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read.");
        if let Some(delimiters) = &self.delimiters {
            for token in line.split(|c| delimiters.contains(&c)) {
                self.tokens.push_back(token.to_string());
            }
        } else {
            for token in line.split_whitespace() {
                self.tokens.push_back(token.to_string());
            }
        }
    }

    fn next_line(&mut self) -> String {
        if !self.tokens.is_empty() {
            panic!("You have unprocessed token");
        }
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read.");
        line
    }
}

#[derive(Clone)]
struct MultiSet<E: Eq + Hash> {
    hash_map: HashMap<E, U>,
}
impl<E: Eq + Hash> MultiSet<E> {
    fn new() -> MultiSet<E> {
        MultiSet {
            hash_map: HashMap::<E, U>::new(),
        }
    }

    fn count(&self, e: &E) -> &U {
        return self.hash_map.get(e).unwrap_or(&0);
    }

    fn insert(&mut self, e: E) {
        let next = self.count(&e) + 1;
        self.hash_map.insert(e, next);
    }
}

trait BinarySearchable<T> {
    fn binary_search_leq(&self, x: &T) -> usize;
    fn binary_search_geq(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearchable<T> for [T] {
    fn binary_search_leq(&self, x: &T) -> usize {
        self.binary_search_by(|p| {
            let r = p.cmp(x);
            if r == Ordering::Greater {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
        .err()
        .unwrap()
    }

    fn binary_search_geq(&self, x: &T) -> usize {
        self.binary_search_by(|p| {
            let r = p.cmp(x);
            if r == Ordering::Less {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .err()
        .unwrap()
    }
}

fn log2_ceil(x: usize) -> u32 {
    if x == 0 {
        0
    } else {
        usize::BITS - (x - 1).leading_zeros()
    }
}
fn pow2_ceil(x: usize) -> usize {
    let n = log2_ceil(x);
    2usize.pow(n)
}

struct IndexedVec<E, F> {
    combine: F,
    inner: Vec<E>,
    tree: Vec<E>,
    inner_cap: usize,
    zero: E,
}
impl<E, F> Deref for IndexedVec<E, F> {
    type Target = Vec<E>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<E: Clone, F: Fn(&E, &E) -> E> IndexedVec<E, F> {
    fn parent(i: usize) -> usize {
        i / 2
    }
    fn left(i: usize) -> usize {
        2 * i
    }
    fn right(i: usize) -> usize {
        2 * i + 1
    }

    fn new(zero: E, combine: F) -> IndexedVec<E, F> {
        IndexedVec {
            combine,
            inner: Vec::new(),
            tree: Vec::new(),
            inner_cap: 0,
            zero,
        }
    }
    fn with_capacity(capacity: usize, zero: E, combine: F) -> IndexedVec<E, F> {
        IndexedVec {
            combine,
            inner: Vec::with_capacity(capacity),
            tree: Vec::with_capacity(pow2_ceil(capacity) * 2),
            inner_cap: 0,
            zero,
        }
    }
    fn from_vec(vec: Vec<E>, zero: E, combine: F) -> IndexedVec<E, F> {
        let mut iv = IndexedVec {
            combine,
            inner: vec,
            zero,
            tree: Vec::new(),
            inner_cap: 0,
        };
        iv.rebuild();
        iv
    }

    fn rebuild(&mut self) {
        let inner = &mut self.inner;
        let combine = &self.combine;
        let zero = &self.zero;
        let inner_cap = pow2_ceil(inner.len());
        let mut tree = vec![zero.clone(); inner_cap * 2];
        tree[inner_cap..(inner_cap + inner.len())].clone_from_slice(&inner[..]);
        let mut n = inner_cap;
        while n > 1 {
            n /= 2;
            for i in n..(n * 2) {
                tree[i] = combine(&tree[Self::left(i)], &tree[Self::right(i)]);
            }
        }
        self.tree = tree;
        self.inner_cap = inner_cap;
    }

    fn query(&self, rng: impl RangeBounds<usize>) -> E {
        let start = match rng.start_bound() {
            Excluded(x) => x + 1,
            Included(x) => *x,
            Unbounded => 0,
        };
        let end = match rng.end_bound() {
            Excluded(x) => x - 1,
            Included(x) => *x,
            Unbounded => self.inner.len() - 1,
        };
        let mut start = start + self.inner_cap;
        let mut end = end + self.inner_cap;
        let mut result = self.zero.clone();
        while start <= end {
            if start % 2 == 1 {
                result = (self.combine)(&result, &self.tree[start]);
                start += 1;
            }
            if end % 2 == 0 {
                result = (self.combine)(&result, &self.tree[end]);
                end -= 1;
            }
            start = Self::parent(start);
            end = Self::parent(end);
        }
        result
    }

    fn update(&mut self, index: usize) {
        self.tree[index + self.inner_cap] = if index < self.inner.len() {
            self.inner[index].clone()
        } else {
            self.zero.clone()
        };
        let mut index = index + self.inner_cap;
        while index > 1 {
            index = Self::parent(index);
            self.tree[index] = (self.combine)(
                &self.tree[Self::left(index)],
                &self.tree[Self::right(index)],
            );
        }
    }
    fn push(&mut self, e: E) {
        self.inner.push(e);
        if self.inner.len() > self.inner_cap {
            self.rebuild();
        } else {
            self.update(self.inner.len() - 1);
        }
    }
    fn pop(&mut self) -> Option<E> {
        let e = self.inner.pop();
        if self.inner.len() * 2 <= self.inner_cap {
            self.rebuild();
        } else {
            self.update(self.inner.len());
        }
        e
    }
    fn set(&mut self, i: usize, e: E) {
        self.inner[i] = e;
        self.update(i);
    }
}
#[cfg(test)]
mod test_indexed_vec {
    use crate::IndexedVec;

    #[test]
    fn test() {
        let mut iv = IndexedVec::from_vec(vec![1, 3, 4, 8, 6, 1, 4, 2], i32::MAX, |a, b| {
            if a < b {
                *a
            } else {
                *b
            }
        });
        assert_eq!(iv.query(1..7), 1);
        iv.set(5, 100);
        assert_eq!(iv.query(1..7), 3);
        iv.push(-2);
        assert_eq!(iv.query(..), -2);
        iv.set(8, 100);
        assert_eq!(iv.query(7..=8), 2);
        iv.set(8, -2);
        assert_eq!(iv.query(7..=8), -2);
        assert_eq!(iv.pop(), Some(-2));
        assert_eq!(iv.query(..), 1);
    }
}

type I = i128;
type U = usize;

fn main() {}
