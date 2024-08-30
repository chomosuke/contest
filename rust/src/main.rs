#![allow(
    unused_imports,
    dead_code,
    clippy::needless_range_loop,
    unused_labels,
    clippy::ptr_arg,
    clippy::comparison_chain,
    clippy::collapsible_else_if
)]
use core::hash::Hash;
use io::*;
use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
    io::{stdin, stdout, BufReader, Stdin, Stdout},
    iter,
    mem::{self, swap},
    ops::{
        Bound::{Excluded, Included, Unbounded},
        Deref, Range, RangeBounds,
    },
    usize,
};

type I = i128;
type U = u128;

pub fn edges_to_adj_nodes(edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj_nodes = Vec::new();
    for &(u, v) in edges {
        let max_node = max(u, v);
        while max_node >= adj_nodes.len() {
            adj_nodes.push(Vec::new());
        }
        adj_nodes[u].push(v);
        adj_nodes[v].push(u);
    }
    adj_nodes
}

pub struct RootedTree {
    parents: Vec<Option<usize>>,
    childrens: Vec<Vec<usize>>,
}

pub fn root_tree(adj_nodes: &Vec<Vec<usize>>, root: usize) -> RootedTree {
    let mut parents = vec![None; adj_nodes.len()];
    let mut childrens = vec![Vec::new(); adj_nodes.len()];

    let mut to_visit = adj_nodes[root]
        .iter()
        .map(|&n| (root, n))
        .collect::<Vec<_>>();
    let mut visited = vec![false; adj_nodes.len()];
    while let Some((parent, node)) = to_visit.pop() {
        assert!(!visited[node], "There's a cycle in your tree");
        visited[node] = true;
        parents[node] = Some(parent);
        childrens[parent].push(node);
        to_visit.extend(adj_nodes[node].iter().filter_map(|&n| {
            if n == parent {
                None
            } else {
                Some((node, n))
            }
        }));
    }

    RootedTree { parents, childrens }
}

fn solve(sc: &mut Scanner<Stdin>, pt: &mut Printer<Stdout>) {
    let n = sc.next::<usize>();
    let mut edges = Vec::with_capacity(n);
    for _ in 0..n - 1 {
        let u = sc.next::<usize>() - 1;
        let v = sc.next::<usize>() - 1;
        edges.push((u, v));
    }
    let RootedTree { parents, childrens } = root_tree(&edges_to_adj_nodes(&edges), 0);
    let values = sc.next_line().into_bytes();
    let mut root_value = values[0];
    let mut leaf_value = Vec::new();
    let mut node_blank = 0;
    for (i, c) in childrens.into_iter().enumerate() {
        if c.is_empty() {
            leaf_value.push(values[i]);
        } else if i != 0 && values[i] == b'?' {
            node_blank += 1;
        }
    }
    let count0 = leaf_value.iter().filter(|&&v| v == b'0').count();
    let count1 = leaf_value.iter().filter(|&&v| v == b'1').count();
    let count_blank = leaf_value.iter().filter(|&&v| v == b'?').count();

    let mut iris_turn = true;
    if root_value != b'?' || count1 != count0 || count_blank % 2 == 0 || node_blank % 2 == 0 {
        if root_value == b'?' {
            iris_turn = false;
            let count0 = leaf_value.iter().filter(|&&v| v == b'0').count();
            let count1 = leaf_value.iter().filter(|&&v| v == b'1').count();
            if count0 > count1 {
                root_value = b'1';
            } else {
                root_value = b'0';
            }
        }
        let mut count_same = leaf_value.iter().filter(|&&v| v == root_value).count();
        let mut count_blank = count_blank;
        let mut count_diff = leaf_value.len() - count_same - count_blank;
        while count_blank > 0 {
            if iris_turn {
                iris_turn = false;
                count_diff += 1;
            } else {
                iris_turn = true;
                count_same += 1;
            }
            count_blank -= 1;
        }
        pt.println(count_diff);
    } else {
        pt.println(count1 + count_blank / 2 + 1);
    }
}

fn main() {
    let mut sc = Scanner::new(stdin());
    let mut pt = Printer::new(stdout());
    let test_cases = sc.next::<usize>();
    'test: for _ in 0..test_cases {
        solve(&mut sc, &mut pt);
    }
}

mod io {
    use std::collections::{HashSet, VecDeque};
    use std::fmt::Display;
    use std::io::{BufReader, BufWriter, Lines, Read, Write};
    use std::marker::PhantomData;
    use std::{any::type_name, io::BufRead, str::FromStr};

    pub struct Scanner<R: Read> {
        tokens: VecDeque<String>,
        delimiters: Option<HashSet<char>>,
        lines: Lines<BufReader<R>>,
    }
    impl<R: Read> Scanner<R> {
        pub fn new(source: R) -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: None,
                lines: BufReader::new(source).lines(),
            }
        }

        pub fn with_delimiters(source: R, delimiters: &[char]) -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: Some(delimiters.iter().copied().collect()),
                lines: BufReader::new(source).lines(),
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

        pub fn next_n<T: FromStr>(&mut self, n: usize) -> Vec<T> {
            let mut v = Vec::with_capacity(n);
            for _ in 0..n {
                v.push(self.next());
            }
            v
        }

        pub fn next_line(&mut self) -> String {
            assert!(self.tokens.is_empty(), "You have unprocessed token");
            self.lines
                .next()
                .and_then(|e| e.ok())
                .expect("Failed to read.")
        }

        fn receive_input(&mut self) {
            let line = self
                .lines
                .next()
                .and_then(|e| e.ok())
                .expect("Failed to read.");
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

    pub struct Printer<W: Write> {
        writer: BufWriter<W>,
    }
    impl<W: Write> Printer<W> {
        pub fn new(destination: W) -> Self {
            Self {
                writer: BufWriter::new(destination),
            }
        }

        pub fn print(&mut self, s: impl Display) {
            self.writer
                .write_all(s.to_string().as_bytes())
                .expect("print failed.");
        }

        pub fn print_bytes(&mut self, b: &[u8]) {
            self.writer.write_all(b).expect("print_bytes failed.");
        }

        pub fn println(&mut self, s: impl Display) {
            self.print(s);
            self.newline();
        }

        pub fn newline(&mut self) {
            self.print_bytes(&[b'\n']);
        }

        pub fn print_iter(&mut self, mut iter: impl Iterator<Item = impl Display>) {
            if let Some(e) = iter.next() {
                self.print(&e);
                for e in iter {
                    self.print_bytes(&[b' ']);
                    self.print(&e);
                }
            }
            self.newline();
        }
    }
    impl<W: Write> Drop for Printer<W> {
        fn drop(&mut self) {
            self.writer
                .flush()
                .expect("flush failed when dropping Printer.");
        }
    }
}
