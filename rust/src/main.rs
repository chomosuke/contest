#![allow(unused_imports, dead_code, clippy::needless_range_loop, unused_labels)]
use io::*;
use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
    io::{stdin, stdout, BufReader},
    iter,
    mem::{self, swap},
    ops::{
        Bound::{Excluded, Included, Unbounded},
        Deref, RangeBounds,
    },
    usize,
};

fn edges_to_adj_nodes(edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
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

struct RootedTree {
    parents: Vec<Option<usize>>,
    childrens: Vec<Vec<usize>>,
}

fn root_tree(adj_nodes: &Vec<Vec<usize>>, root: usize) -> RootedTree {
    let mut parents = vec![None; adj_nodes.len()];
    let mut childrens = vec![Vec::new(); adj_nodes.len()];

    if adj_nodes.is_empty() {
        return RootedTree {
            parents: vec![None],
            childrens: vec![Vec::new()],
        };
    }

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

fn min_attack(
    node: usize,
    no_kill: usize,
    tree: &RootedTree,
    arr: &Vec<u128>,
    mem_min_attack_when_kill: &mut Vec<[Option<u128>; 100]>,
) -> u128 {
    for i in 0..100 {
        if mem_min_attack_when_kill[node][i].is_none() {
            let mx = (i as u128 + 1) * arr[node]
                + tree.childrens[node]
                    .iter()
                    .map(|&child| min_attack(child, i, tree, arr, mem_min_attack_when_kill))
                    .sum::<u128>();
            mem_min_attack_when_kill[node][i] = Some(mx);
        }
    }
    mem_min_attack_when_kill[node]
        .iter()
        .enumerate()
        .map(|(i, &m)| if no_kill == i { u128::MAX } else { m.unwrap() })
        .min()
        .unwrap()
}

fn main() {
    let mut sc = Scanner::new(stdin());
    let mut pt = Printer::new(stdout());
    let test_cases = sc.next::<usize>();
    'test: for _ in 0..test_cases {
        let n = sc.next::<usize>();
        let arr = sc.next_n::<u128>(n);
        let mut edges = Vec::with_capacity(n - 1);
        for _ in 0..n - 1 {
            edges.push((sc.next::<usize>() - 1, sc.next::<usize>() - 1));
        }
        let adj_nodes = edges_to_adj_nodes(&edges);
        let rooted = root_tree(&adj_nodes, 0);
        let mut mem = vec![[None; 100]; n];
        let m1 = min_attack(0, 100, &rooted, &arr, &mut mem);

        pt.println(m1);
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
