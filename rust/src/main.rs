#![allow(dead_code)]
type I = i128;
type U = usize;

fn main() {}

mod scanner {
    use std::collections::{HashSet, VecDeque};
    use std::{any::type_name, io::stdin, str::FromStr};

    pub struct Scanner {
        tokens: VecDeque<String>,
        delimiters: Option<HashSet<char>>,
    }
    impl Scanner {
        pub fn new() -> Scanner {
            Scanner {
                tokens: VecDeque::new(),
                delimiters: None,
            }
        }

        pub fn with_delimiters(delimiters: &[char]) -> Scanner {
            Scanner {
                tokens: VecDeque::new(),
                delimiters: Some(delimiters.iter().copied().collect()),
            }
        }

        pub fn next<T: FromStr>(&mut self) -> T {
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

        pub fn next_line(&mut self) -> String {
            if !self.tokens.is_empty() {
                panic!("You have unprocessed token");
            }
            let mut line = String::new();
            stdin().read_line(&mut line).expect("Failed to read.");
            line
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
    }
}
#[allow(unused_imports)]
use scanner::*;

mod multi_set {
    use core::hash::Hash;
    use std::collections::HashMap;

    use crate::U;

    #[derive(Clone)]
    pub struct MultiSet<E: Eq + Hash> {
        hash_map: HashMap<E, U>,
    }
    impl<E: Eq + Hash> MultiSet<E> {
        pub fn new() -> MultiSet<E> {
            MultiSet {
                hash_map: HashMap::<E, U>::new(),
            }
        }

        pub fn count(&self, e: &E) -> &U {
            return self.hash_map.get(e).unwrap_or(&0);
        }

        pub fn insert(&mut self, e: E) {
            let next = self.count(&e) + 1;
            self.hash_map.insert(e, next);
        }
    }
}
#[allow(unused_imports)]
use multi_set::*;

mod binary_searchable {
    use std::cmp::*;

    pub trait BinarySearchable<T> {
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
}
#[allow(unused_imports)]
use binary_searchable::*;

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

mod indexed_vec {
    use std::ops::Bound::*;
    use std::ops::{Deref, RangeBounds};

    use crate::pow2_ceil;

    pub struct IndexedVec<E, F> {
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

        pub fn new(zero: E, combine: F) -> IndexedVec<E, F> {
            IndexedVec {
                combine,
                inner: Vec::new(),
                tree: Vec::new(),
                inner_cap: 0,
                zero,
            }
        }
        pub fn with_capacity(capacity: usize, zero: E, combine: F) -> IndexedVec<E, F> {
            IndexedVec {
                combine,
                inner: Vec::with_capacity(capacity),
                tree: Vec::with_capacity(pow2_ceil(capacity) * 2),
                inner_cap: 0,
                zero,
            }
        }
        pub fn from_vec(vec: Vec<E>, zero: E, combine: F) -> IndexedVec<E, F> {
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

        pub fn query(&self, rng: impl RangeBounds<usize>) -> E {
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
        pub fn push(&mut self, e: E) {
            self.inner.push(e);
            if self.inner.len() > self.inner_cap {
                self.rebuild();
            } else {
                self.update(self.inner.len() - 1);
            }
        }
        pub fn pop(&mut self) -> Option<E> {
            let e = self.inner.pop();
            if self.inner.len() * 2 <= self.inner_cap {
                self.rebuild();
            } else {
                self.update(self.inner.len());
            }
            e
        }
        pub fn set(&mut self, i: usize, e: E) {
            self.inner[i] = e;
            self.update(i);
        }
    }
    #[cfg(test)]
    mod test {
        use super::IndexedVec;

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
}
#[allow(unused_imports)]
use indexed_vec::*;

mod graph {
    use std::{
        cmp::Reverse,
        collections::{BinaryHeap, VecDeque},
    };

    pub struct DepthFirstIter<'a> {
        graph: &'a Graph,
        pushed: Vec<bool>,
        stack: VecDeque<usize>,
    }
    impl DepthFirstIter<'_> {
        fn new(graph: &Graph, start: usize) -> DepthFirstIter {
            let capacity = graph.get_adjacent_nodes().len();
            let mut pushed = vec![false; capacity];
            let mut stack = VecDeque::with_capacity(capacity);
            stack.push_back(start);
            pushed[start] = true;
            DepthFirstIter {
                graph,
                pushed,
                stack,
            }
        }
    }
    impl Iterator for DepthFirstIter<'_> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let DepthFirstIter {
                graph,
                pushed,
                stack,
            } = self;
            if let Some(node) = stack.pop_back() {
                let adjacent_nodes = graph.get_adjacent_nodes();
                for &(node, _) in adjacent_nodes[node].iter().rev() {
                    if !pushed[node] {
                        stack.push_back(node);
                        pushed[node] = true;
                    }
                }
                Some(node)
            } else {
                None
            }
        }
    }
    pub struct BreathFirstIter<'a> {
        graph: &'a Graph,
        pushed: Vec<bool>,
        queue: VecDeque<usize>,
    }
    impl BreathFirstIter<'_> {
        fn new(graph: &Graph, start: usize) -> BreathFirstIter {
            let capacity = graph.get_adjacent_nodes().len();
            let mut pushed = vec![false; capacity];
            let mut queue = VecDeque::with_capacity(capacity);
            queue.push_back(start);
            pushed[start] = true;
            BreathFirstIter {
                graph,
                pushed,
                queue,
            }
        }
    }
    impl Iterator for BreathFirstIter<'_> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let BreathFirstIter {
                graph,
                pushed,
                queue,
            } = self;
            if let Some(node) = queue.pop_front() {
                let adjacent_nodes = graph.get_adjacent_nodes();
                for &(node, _) in adjacent_nodes[node].iter() {
                    if !pushed[node] {
                        queue.push_back(node);
                        pushed[node] = true;
                    }
                }
                Some(node)
            } else {
                None
            }
        }
    }

    pub struct Graph {
        adjacent_nodes: Vec<Vec<(usize, i128)>>,
        negative_weight_count: usize,
    }
    impl Graph {
        pub fn new() -> Graph {
            Graph {
                adjacent_nodes: Vec::new(),
                negative_weight_count: 0,
            }
        }
        pub fn with_capacity(capacity: usize) -> Graph {
            Graph {
                adjacent_nodes: Vec::with_capacity(capacity),
                negative_weight_count: 0,
            }
        }
        pub fn from_edges(edges: Vec<(usize, usize, i128)>, node_count: usize) -> Graph {
            let mut g = Graph {
                adjacent_nodes: vec![Vec::new(); node_count],
                negative_weight_count: 0,
            };
            for edge in edges {
                g.add_edge(edge);
            }
            g
        }

        pub fn add_node(&mut self) -> usize {
            self.adjacent_nodes.push(Vec::new());
            self.adjacent_nodes.len() - 1
        }
        pub fn add_edge(&mut self, edge: (usize, usize, i128)) {
            self.adjacent_nodes[edge.0].push((edge.1, edge.2));
            if edge.2 < 0 {
                self.negative_weight_count += 1;
            }
        }

        pub fn shortest_path_len<F: Fn(usize) -> bool>(
            &self,
            start: usize,
            stop_when: F,
        ) -> Vec<i128> {
            if self.negative_weight_count == 0 {
                self.dijkstra(start, stop_when)
            } else {
                self.bellman_ford(start, stop_when)
            }
        }
        fn dijkstra<F: Fn(usize) -> bool>(&self, start: usize, stop_when: F) -> Vec<i128> {
            let mut shortest_path_len = vec![i128::MAX; self.adjacent_nodes.len()];
            let mut queue = BinaryHeap::new();
            queue.push(Reverse((0, start)));
            while let Some(Reverse((distance, node))) = queue.pop() {
                if shortest_path_len[node] != i128::MAX {
                    continue;
                }
                shortest_path_len[node] = distance;
                if stop_when(node) {
                    break;
                }
                for (adj_node, weight) in &self.adjacent_nodes[node] {
                    queue.push(Reverse((distance + *weight, *adj_node)));
                }
            }
            shortest_path_len
        }
        fn bellman_ford<F: Fn(usize) -> bool>(&self, start: usize, _stop_when: F) -> Vec<i128> {
            let mut shortest_path_len = vec![i128::MAX; self.adjacent_nodes.len()];
            shortest_path_len[start] = 0;
            let mut queue = VecDeque::new();
            queue.push_back(start);
            while let Some(node) = queue.pop_front() {
                for &(adj_node, weight) in &self.adjacent_nodes[node] {
                    if shortest_path_len[node] + weight < shortest_path_len[adj_node] {
                        shortest_path_len[adj_node] = shortest_path_len[node] + weight;
                        queue.push_back(adj_node);
                    }
                }
            }
            shortest_path_len
        }

        pub fn get_adjacent_nodes(&self) -> &Vec<Vec<(usize, i128)>> {
            &self.adjacent_nodes
        }

        pub fn depth_first_iter(
            &self,
            start: usize,
        ) -> DepthFirstIter {
            DepthFirstIter::new(self, start)
        }
        pub fn breath_first_iter(
            &self,
            start: usize,
        ) -> BreathFirstIter {
            BreathFirstIter::new(self, start)
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn dijkstra() {
            let g = Graph::from_edges(
                vec![
                    (1, 2, 5),
                    (1, 4, 9),
                    (1, 5, 1),
                    (2, 1, 5),
                    (2, 3, 2),
                    (3, 2, 2),
                    (3, 4, 6),
                    (4, 1, 9),
                    (4, 3, 6),
                    (4, 5, 2),
                    (5, 1, 1),
                    (5, 4, 2),
                ],
                6,
            );
            assert_eq!(
                g.shortest_path_len(1, |_| false),
                vec![i128::MAX, 0, 5, 7, 3, 1],
            );
        }

        #[test]
        fn bellman_ford() {
            let g = Graph::from_edges(
                vec![
                    (1, 2, 5),
                    (1, 3, 3),
                    (1, 4, 7),
                    (2, 1, 5),
                    (2, 4, 3),
                    (2, 5, 2),
                    (3, 1, 3),
                    (3, 4, 1),
                    (3, 5, -1),
                    (4, 1, 7),
                    (4, 2, 3),
                    (4, 3, 1),
                    (4, 5, 2),
                    (5, 2, 2),
                    (5, 4, 2),
                ],
                6,
            );
            assert_eq!(
                g.shortest_path_len(1, |_| false),
                vec![i128::MAX, 0, 4, 3, 4, 2],
            );
        }

        #[test]
        fn dfs() {
            let g = Graph::from_edges(
                vec![
                    (1, 2, 1),
                    (1, 4, 1),
                    (2, 1, 1),
                    (2, 3, 1),
                    (2, 5, 1),
                    (3, 2, 1),
                    (3, 5, 1),
                    (4, 1, 1),
                    (5, 2, 1),
                    (5, 3, 1),
                ],
                6,
            );

            assert_eq!(
                g.depth_first_iter(1).collect::<Vec<_>>(),
                vec![1, 2, 3, 5, 4]
            );
        }

        #[test]
        fn bfs() {
            let g = Graph::from_edges(
                vec![
                    (1, 2, 1),
                    (1, 4, 1),
                    (2, 1, 1),
                    (2, 3, 1),
                    (2, 5, 1),
                    (3, 2, 1),
                    (3, 6, 1),
                    (4, 1, 1),
                    (5, 2, 1),
                    (5, 6, 1),
                ],
                7,
            );

            assert_eq!(
                g.breath_first_iter(1).collect::<Vec<_>>(),
                vec![1, 2, 4, 3, 5, 6]
            );
        }
    }
}
#[allow(unused_imports)]
use graph::*;
