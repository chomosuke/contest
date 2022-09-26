#![allow(dead_code, clippy::needless_range_loop)]
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
        pub fn new() -> Self {
            Scanner {
                tokens: VecDeque::new(),
                delimiters: None,
            }
        }

        pub fn with_delimiters(delimiters: &[char]) -> Self {
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
            assert!(self.tokens.is_empty(), "You have unprocessed token");
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
        pub fn new() -> Self {
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

        pub fn new(zero: E, combine: F) -> Self {
            IndexedVec {
                combine,
                inner: Vec::new(),
                tree: Vec::new(),
                inner_cap: 0,
                zero,
            }
        }
        pub fn with_capacity(capacity: usize, zero: E, combine: F) -> Self {
            IndexedVec {
                combine,
                inner: Vec::with_capacity(capacity),
                tree: Vec::with_capacity(pow2_ceil(capacity) * 2),
                inner_cap: 0,
                zero,
            }
        }
        pub fn from_vec(vec: Vec<E>, zero: E, combine: F) -> Self {
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

mod search_iter {
    use std::collections::VecDeque;

    pub struct DepthFirstIter<'a> {
        adj_nodess: &'a Vec<Vec<(usize, i128)>>,
        pushed: Vec<bool>,
        stack: VecDeque<usize>,
    }
    impl<'a> DepthFirstIter<'a> {
        pub fn new(adj_nodess: &'a Vec<Vec<(usize, i128)>>, start: usize) -> Self {
            let capacity = adj_nodess.len();
            let mut pushed = vec![false; capacity];
            let mut stack = VecDeque::with_capacity(capacity);
            stack.push_back(start);
            pushed[start] = true;
            DepthFirstIter {
                adj_nodess,
                pushed,
                stack,
            }
        }
    }
    impl Iterator for DepthFirstIter<'_> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let DepthFirstIter {
                adj_nodess,
                pushed,
                stack,
            } = self;
            if let Some(node) = stack.pop_back() {
                for &(node, _) in adj_nodess[node].iter().rev() {
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
        adj_nodess: &'a Vec<Vec<(usize, i128)>>,
        pushed: Vec<bool>,
        queue: VecDeque<usize>,
    }
    impl<'a> BreathFirstIter<'a> {
        pub fn new(adj_nodess: &'a Vec<Vec<(usize, i128)>>, start: usize) -> Self {
            let capacity = adj_nodess.len();
            let mut pushed = vec![false; capacity];
            let mut queue = VecDeque::with_capacity(capacity);
            queue.push_back(start);
            pushed[start] = true;
            BreathFirstIter {
                adj_nodess,
                pushed,
                queue,
            }
        }
    }
    impl Iterator for BreathFirstIter<'_> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let BreathFirstIter {
                adj_nodess,
                pushed,
                queue,
            } = self;
            if let Some(node) = queue.pop_front() {
                for &(node, _) in adj_nodess[node].iter() {
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

    #[cfg(test)]
    mod test {
        use crate::graph::Graph;

        const EDGES: &[(usize, usize, i128)] = &[
            (0, 1, 5),
            (0, 2, 3),
            (0, 3, 7),
            (1, 0, 5),
            (1, 3, 3),
            (1, 4, 2),
            (2, 0, 3),
            (2, 3, 1),
            (3, 0, 7),
            (3, 1, 3),
            (3, 2, 1),
            (3, 4, 2),
            (4, 1, 2),
            (4, 3, 2),
        ];

        #[test]
        fn dfs() {
            let g = Graph::from_edges(EDGES.to_vec(), 5, true);
            assert_eq!(
                g.depth_first_iter(0).collect::<Vec<_>>(),
                vec![0, 1, 4, 2, 3]
            );
        }

        #[test]
        fn bfs() {
            let g = Graph::from_edges(EDGES.to_vec(), 5, true);
            assert_eq!(
                g.breath_first_iter(0).collect::<Vec<_>>(),
                vec![0, 1, 2, 3, 4]
            );
        }
    }
}
#[allow(unused_imports)]
use search_iter::*;

mod graph {
    use crate::{BreathFirstIter, DepthFirstIter};
    use std::{
        cmp::Reverse,
        collections::{BinaryHeap, VecDeque},
    };

    pub struct Graph {
        adj_nodess: Vec<Vec<(usize, i128)>>,
        rev_adj_nodess: Option<Vec<Vec<(usize, i128)>>>,
        neg_edge_count: u128,
        directed: bool,
    }
    impl Graph {
        pub fn new(directed: bool) -> Self {
            Graph {
                adj_nodess: Vec::new(),
                rev_adj_nodess: None,
                neg_edge_count: 0,
                directed,
            }
        }
        pub fn with_capacity(capacity: usize, directed: bool) -> Self {
            Graph {
                adj_nodess: Vec::with_capacity(capacity),
                rev_adj_nodess: None,
                neg_edge_count: 0,
                directed,
            }
        }
        pub fn from_edges(
            edges: Vec<(usize, usize, i128)>,
            node_count: usize,
            directed: bool,
        ) -> Self {
            let mut g = Graph {
                adj_nodess: vec![Vec::new(); node_count],
                rev_adj_nodess: None,
                neg_edge_count: 0,
                directed,
            };
            for edge in edges {
                g.add_edge(edge);
            }
            g
        }

        pub fn enable_rev_adj_nodes(&mut self) {
            if self.rev_adj_nodess.is_some() || !self.directed {
                return;
            }
            let mut rev_adj_nodess = vec![Vec::new(); self.adj_nodess.len()];
            for (node, adj_nodes) in self.adj_nodess.iter().enumerate() {
                for &(adj_node, weight) in adj_nodes {
                    rev_adj_nodess[adj_node].push((node, weight));
                }
            }
            self.rev_adj_nodess = Some(rev_adj_nodess);
        }

        pub fn get_adj_nodess(&self) -> &Vec<Vec<(usize, i128)>> {
            &self.adj_nodess
        }
        pub fn get_rev_adj_nodess(&self) -> Option<&Vec<Vec<(usize, i128)>>> {
            if self.directed {
                self.rev_adj_nodess.as_ref()
            } else {
                Some(&self.adj_nodess)
            }
        }

        pub fn add_node(&mut self) -> usize {
            self.adj_nodess.push(Vec::new());
            if let Some(rev_adj_nodes) = &mut self.rev_adj_nodess {
                rev_adj_nodes.push(Vec::new());
            }
            self.adj_nodess.len() - 1
        }
        pub fn add_edge(&mut self, edge: (usize, usize, i128)) {
            self.adj_nodess[edge.0].push((edge.1, edge.2));
            if !self.directed {
                self.adj_nodess[edge.1].push((edge.0, edge.2));
            }
            if edge.2 < 0 {
                self.neg_edge_count += 1;
            }
            if let Some(rev_adj_nodes) = &mut self.rev_adj_nodess {
                rev_adj_nodes[edge.1].push((edge.0, edge.2));
            }
        }

        pub fn depth_first_iter(&self, start: usize) -> DepthFirstIter {
            DepthFirstIter::new(&self.adj_nodess, start)
        }
        pub fn breath_first_iter(&self, start: usize) -> BreathFirstIter {
            BreathFirstIter::new(&self.adj_nodess, start)
        }

        pub fn get_shortest_path_lens(&self, start: usize) -> Option<Vec<i128>> {
            self.get_shortest_path_lens_till_stop(start, |_| false)
        }
        pub fn get_shortest_path_lens_till_stop<F: Fn(usize) -> bool>(
            &self,
            start: usize,
            stop_when: F,
        ) -> Option<Vec<i128>> {
            if self.neg_edge_count == 0 {
                self.dijkstra(start, stop_when)
            } else {
                self.spfa(start, stop_when)
            }
        }
        fn dijkstra<F: Fn(usize) -> bool>(&self, start: usize, stop_when: F) -> Option<Vec<i128>> {
            let mut shortest_path_len = vec![i128::MAX; self.adj_nodess.len()];
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
                for (adj_node, weight) in &self.adj_nodess[node] {
                    queue.push(Reverse((distance + *weight, *adj_node)));
                }
            }
            Some(shortest_path_len)
        }
        fn spfa<F: Fn(usize) -> bool>(&self, start: usize, _stop_when: F) -> Option<Vec<i128>> {
            let mut shortest_path_len = vec![i128::MAX; self.adj_nodess.len()];
            let mut shortest_path_edge_len = vec![0; self.adj_nodess.len()];
            shortest_path_len[start] = 0;
            let mut queue = VecDeque::new();
            queue.push_back(start);
            while let Some(node) = queue.pop_front() {
                for &(adj_node, weight) in &self.adj_nodess[node] {
                    if shortest_path_len[node] + weight < shortest_path_len[adj_node] {
                        shortest_path_len[adj_node] = shortest_path_len[node] + weight;
                        shortest_path_edge_len[adj_node] = shortest_path_edge_len[node] + 1;
                        if shortest_path_edge_len[adj_node] >= self.adj_nodess.len() {
                            // negative cycle
                            return None;
                        }
                        queue.push_back(adj_node);
                    }
                }
            }
            Some(shortest_path_len)
        }

        pub fn get_all_shortest_path_lens(&self) -> Option<Vec<Vec<i128>>> {
            let n = self.adj_nodess.len();
            let mut shortest_path_lens = vec![vec![i128::MAX; n]; n];
            for (node, adj_nodes) in self.adj_nodess.iter().enumerate() {
                for &(adj_node, weight) in adj_nodes {
                    shortest_path_lens[node][adj_node] = weight;
                }
            }
            for node in 0..n {
                shortest_path_lens[node][node] = 0;
            }
            for nodei in 0..n {
                for node1 in 0..n {
                    for node2 in 0..n {
                        if shortest_path_lens[node1][nodei] != i128::MAX
                            && shortest_path_lens[nodei][node2] != i128::MAX
                        {
                            shortest_path_lens[node1][node2] = shortest_path_lens[node1][node2]
                                .min(
                                    shortest_path_lens[node1][nodei]
                                        + shortest_path_lens[nodei][node2],
                                );
                        }
                    }
                }
            }
            for node in 0..n {
                if shortest_path_lens[node][node] < 0 {
                    return None;
                }
            }
            Some(shortest_path_lens)
        }

        pub fn reconstruct_shortest_path(
            &self,
            shortest_path_lens: &[i128],
            start: usize,
            end: usize,
        ) -> Option<Vec<usize>> {
            let dists_to_start = shortest_path_lens;
            let rev_adj_nodess = self
                .get_rev_adj_nodess()
                .expect("need to enable_rev_adj_nodes before calling find_shortest_path");
            assert!(
                dists_to_start[start] == 0,
                "expected shortest_path_lens[start] to be zero, looks like you got the wrong start node",
            );
            if dists_to_start[end] == i128::MAX {
                return None;
            }
            let mut shortest_path = Vec::new();
            let mut node = end;
            while node != start {
                shortest_path.push(node);
                node = rev_adj_nodess[node]
                    .iter()
                    .filter(|next_node| {
                        dists_to_start[next_node.0] == dists_to_start[node] - next_node.1
                    })
                    .min_by_key(|next_node| dists_to_start[next_node.0])
                    .expect("shortest_path_lens corruption")
                    .0;
            }
            shortest_path.push(node);
            Some(shortest_path.into_iter().rev().collect())
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        const EDGES: &[(usize, usize, i128)] = &[
            (0, 1, 5),
            (0, 2, 3),
            (0, 3, 7),
            (1, 0, 5),
            (1, 3, 3),
            (1, 4, 2),
            (2, 0, 3),
            (2, 3, 1),
            (3, 0, 7),
            (3, 1, 3),
            (3, 2, 1),
            (3, 4, 2),
            (4, 1, 2),
            (4, 3, 2),
        ];

        #[test]
        fn dijkstra() {
            let g = Graph::from_edges(EDGES.to_vec(), 5, true);
            assert_eq!(g.get_shortest_path_lens(0), Some(vec![0, 5, 3, 4, 6]),);
        }

        #[test]
        fn spfa() {
            let mut edges = EDGES.to_vec();
            edges.push((2, 4, -1));
            let g = Graph::from_edges(edges, 5, true);
            assert_eq!(g.get_shortest_path_lens(0), Some(vec![0, 4, 3, 4, 2]),);
        }

        #[test]
        fn spfa_negative_cycle() {
            let mut edges = EDGES.to_vec();
            edges.push((2, 4, -4));
            let g = Graph::from_edges(edges, 5, true);
            assert_eq!(g.get_shortest_path_lens(1), None);
        }

        #[test]
        fn spfa_negative_cycle_no_false_positive() {
            let g = Graph::from_edges(
                vec![
                    (0, 1, 0),
                    (1, 2, 0),
                    (2, 3, -1),
                    (3, 4, 0),
                    (4, 5, 0),
                    (5, 0, 1),
                ],
                6,
                true,
            );
            assert_eq!(g.get_shortest_path_lens(0), Some(vec![0, 0, 0, -1, -1, -1]));
        }

        #[test]
        fn floyd_warshall() {
            let g = Graph::from_edges(EDGES.to_vec(), 5, true);
            assert_eq!(
                g.get_all_shortest_path_lens(),
                Some(vec![
                    vec![0, 5, 3, 4, 6],
                    vec![5, 0, 4, 3, 2],
                    vec![3, 4, 0, 1, 3],
                    vec![4, 3, 1, 0, 2],
                    vec![6, 2, 3, 2, 0],
                ]),
            );
        }

        #[test]
        fn undirected_floyd_warshall() {
            let g = Graph::from_edges(
                vec![
                    (0, 1, 5),
                    (0, 2, 3),
                    (0, 3, 7),
                    (1, 3, 3),
                    (1, 4, 2),
                    (2, 3, 1),
                    (3, 4, 2),
                ],
                5,
                false,
            );
            assert_eq!(
                g.get_all_shortest_path_lens(),
                Some(vec![
                    vec![0, 5, 3, 4, 6],
                    vec![5, 0, 4, 3, 2],
                    vec![3, 4, 0, 1, 3],
                    vec![4, 3, 1, 0, 2],
                    vec![6, 2, 3, 2, 0],
                ]),
            );
        }

        #[test]
        fn floyd_warshall_negative_cycle() {
            let mut edges = EDGES.to_vec();
            edges.push((2, 4, -4));
            let g = Graph::from_edges(edges, 5, false);
            assert_eq!(g.get_all_shortest_path_lens(), None);
        }

        #[test]
        fn floyd_warshall_negative_cycle_no_false_positive() {
            let g = Graph::from_edges(
                vec![
                    (0, 1, 0),
                    (1, 2, 0),
                    (2, 3, -1),
                    (3, 4, 0),
                    (4, 5, 0),
                    (5, 0, 1),
                ],
                6,
                true,
            );
            assert_eq!(
                g.get_all_shortest_path_lens(),
                Some(vec![
                    vec![0, 0, 0, -1, -1, -1],
                    vec![0, 0, 0, -1, -1, -1],
                    vec![0, 0, 0, -1, -1, -1],
                    vec![1, 1, 1, 0, 0, 0],
                    vec![1, 1, 1, 0, 0, 0],
                    vec![1, 1, 1, 0, 0, 0],
                ]),
            );
        }

        #[test]
        fn reconstruct_shortest_path() {
            let mut g = Graph::from_edges(EDGES.to_vec(), 6, true);
            let shortest_path_lens = g.get_shortest_path_lens(0).unwrap();
            g.enable_rev_adj_nodes();
            assert_eq!(
                g.reconstruct_shortest_path(&shortest_path_lens, 0, 4),
                Some(vec![0, 2, 3, 4])
            );
        }
    }
}
#[allow(unused_imports)]
use graph::*;

mod tree {
    use std::collections::HashMap;

    use crate::{BreathFirstIter, DepthFirstIter};

    pub struct Tree {
        adj_nodess: Vec<Vec<(usize, i128)>>,
    }
    impl Tree {
        pub fn new() -> Self {
            Tree {
                adj_nodess: Vec::new(),
            }
        }
        pub fn with_capacity(capacity: usize) -> Self {
            Tree {
                adj_nodess: Vec::with_capacity(capacity),
            }
        }
        pub fn from_edges(edges: Vec<(usize, usize, i128)>, node_count: usize) -> Self {
            assert!(
                edges.len() == node_count - 1,
                "Incorrect amonut of edges for a tree",
            );
            let mut added = vec![false; node_count];
            let mut adj_nodess = vec![Vec::new(); node_count];
            for (node1, node2, weight) in edges {
                assert!(
                    !(added[node1] && added[node2]),
                    "There's a cycle in your tree",
                );
                added[node1] = true;
                added[node2] = true;
                adj_nodess[node1].push((node2, weight));
                adj_nodess[node2].push((node1, weight));
            }
            Tree { adj_nodess }
        }

        pub fn get_adj_nodess(&self) -> &Vec<Vec<(usize, i128)>> {
            &self.adj_nodess
        }

        pub fn depth_first_iter(&self, start: usize) -> DepthFirstIter {
            DepthFirstIter::new(&self.adj_nodess, start)
        }
        pub fn breath_first_iter(&self, start: usize) -> BreathFirstIter {
            BreathFirstIter::new(&self.adj_nodess, start)
        }

        pub fn add_node(&mut self, (connected_node, weight): (usize, i128)) -> usize {
            let new_node = self.adj_nodess.len();
            self.adj_nodess[connected_node].push((new_node, weight));
            self.adj_nodess.push(vec![(connected_node, weight)]);
            new_node
        }

        pub fn get_diameter(&self) -> i128 {
            if self.adj_nodess.is_empty() {
                return 0;
            }
            let mut dist_to_zero = vec![i128::MAX; self.adj_nodess.len()];
            dist_to_zero[0] = 0;
            for node in self.depth_first_iter(0).skip(1) {
                dist_to_zero[node] = self.adj_nodess[node]
                    .iter()
                    .find_map(|&(adj_node, weight)| {
                        if dist_to_zero[adj_node] < i128::MAX {
                            Some(dist_to_zero[adj_node] + weight)
                        } else {
                            None
                        }
                    })
                    .unwrap();
            }
            let start = dist_to_zero
                .into_iter()
                .enumerate()
                .max_by_key(|&(_, dist)| dist)
                .unwrap()
                .0;
            let mut dist_to_start = vec![i128::MAX; self.adj_nodess.len()];
            dist_to_start[start] = 0;
            for node in self.depth_first_iter(start).skip(1) {
                dist_to_start[node] = self.adj_nodess[node]
                    .iter()
                    .find_map(|&(adj_node, weight)| {
                        if dist_to_start[adj_node] < i128::MAX {
                            Some(dist_to_start[adj_node] + weight)
                        } else {
                            None
                        }
                    })
                    .unwrap();
            }
            dist_to_start
                .into_iter()
                .enumerate()
                .max_by_key(|&(_, dist)| dist)
                .unwrap()
                .1
        }

        pub fn get_longest_path_lens(&self) -> Vec<i128> {
            if self.adj_nodess.is_empty() {
                return Vec::new();
            }
            let mut mem = HashMap::with_capacity(self.adj_nodess.len());
            fn longest_path_len_in_dir(
                this: &Tree,
                fst: usize,
                snd: usize,
                weight: i128,
                memoize: &mut HashMap<(usize, usize), i128>,
            ) -> i128 {
                if let Some(path_len) = memoize.get(&(fst, snd)) {
                    return *path_len;
                }
                let path_len = this.adj_nodess[snd]
                    .iter()
                    .filter_map(|&(trd, weight2)| {
                        if trd == fst {
                            None
                        } else {
                            Some(longest_path_len_in_dir(this, snd, trd, weight2, memoize) + weight)
                        }
                    })
                    .max()
                    .unwrap_or(weight);
                memoize.insert((fst, snd), path_len);
                path_len
            }
            let mut f =
                |fst, snd, weight| longest_path_len_in_dir(self, fst, snd, weight, &mut mem);
            let mut longest_path_lens = Vec::with_capacity(self.adj_nodess.len());
            for (node, adj_nodes) in self.adj_nodess.iter().enumerate() {
                let mut longest_path_len = 0;
                for &(adj_node, weight) in adj_nodes {
                    longest_path_len = longest_path_len.max(f(node, adj_node, weight));
                }
                longest_path_lens.push(longest_path_len);
            }
            longest_path_lens
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        const EDGES: &[(usize, usize, i128)] = &[
            (1, 2, 1),
            (1, 3, 1),
            (1, 4, 1),
            (2, 5, 1),
            (2, 6, 1),
            (6, 0, 3),
            (4, 7, 1),
        ];

        #[test]
        fn diameter() {
            let t = Tree::from_edges(EDGES.to_vec(), 8);
            assert_eq!(t.get_diameter(), 7);
        }

        #[test]
        fn all_longest_path_len() {
            let t = Tree::from_edges(EDGES.to_vec(), 8);
            assert_eq!(t.get_longest_path_lens(), vec![7, 5, 4, 6, 6, 5, 4, 7]);
        }
    }
}
