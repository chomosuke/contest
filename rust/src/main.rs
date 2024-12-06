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
use regex::Regex;
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
};

mod input;

use input::*;

type I = i128;
type U = u128;

fn main() {
    let mut lines = input::INPUT
        .lines()
        .map(|l| l.as_bytes().to_owned())
        .collect::<Vec<_>>();
    let mut current = (0, 0);
    'outer: for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == b'^' {
                current = (i, j);
                break 'outer;
            }
        }
    }
    fn next_loc(mut i: i64, mut j: i64, dir: u8) -> (i64, i64) {
        if dir == 0 {
            i -= 1;
        }
        if dir == 2 {
            i += 1;
        }
        if dir == 1 {
            j += 1;
        }
        if dir == 3 {
            j -= 1;
        }
        (i, j)
    }
    fn prev_loc(mut i: i64, mut j: i64, dir: u8) -> (i64, i64) {
        if dir == 0 {
            i += 1;
        }
        if dir == 2 {
            i -= 1;
        }
        if dir == 1 {
            j -= 1;
        }
        if dir == 3 {
            j += 1;
        }
        (i, j)
    }
    let mut count = 0;
    for i0 in 0..lines.len() {
        for j0 in 0..lines[i0].len() {
            if lines[i0][j0] != b'.' {
                continue;
            }
            let mut lines = lines.clone();
            lines[i0][j0] = b'#';
            let mut i = current.0 as i64;
            let mut j = current.1 as i64;
            let mut dir = 0u8;
            let mut l = false;
            while i >= 0 && i < lines.len() as i64 && j >= 0 && j < lines[i as usize].len() as i64 {
                if lines[i as usize][j as usize] == b'#' {
                    (i, j) = prev_loc(i, j, dir);
                    dir += 1;
                    dir %= 4;
                } else {
                    if lines[i as usize][j as usize] != dir {
                        lines[i as usize][j as usize] = dir;
                    } else {
                        l = true;
                        break;
                    }
                }
                (i, j) = next_loc(i, j, dir);
            }
            if l {
                count += 1;
            }
        }
    }
    println!("{count}");
}

pub struct DirectedGraph {
    adj_nodess: Vec<Vec<usize>>,
}
impl DirectedGraph {
    /// O(m + n)
    pub fn from_edges(edges: Vec<(usize, usize)>, node_count: usize) -> Self {
        let mut g = Self {
            adj_nodess: vec![Vec::new(); node_count],
        };
        for edge in edges {
            g.add_edge(edge);
        }
        g
    }

    pub fn node_count(&self) -> usize {
        self.adj_nodess.len()
    }

    /// O(1)
    pub fn add_edge(&mut self, edge: (usize, usize)) {
        self.adj_nodess[edge.0].push(edge.1);
    }

    /// O(n + m)
    pub fn get_topological_sort(&self, from: Option<usize>) -> Result<Vec<usize>, Vec<usize>> {
        let mut rev_sort = Vec::with_capacity(self.node_count());
        let mut states = vec![0; self.node_count()];
        fn have_cycle(
            current: usize,
            rev_sort: &mut Vec<usize>,
            states: &mut [u8],
            adj_nodess: &[Vec<usize>],
        ) -> Option<Vec<usize>> {
            states[current] = 1;
            for &adj_node in &adj_nodess[current] {
                if states[adj_node] == 2 {
                    continue;
                }
                if states[adj_node] == 1 {
                    return Some(vec![adj_node, current]);
                }
                if let Some(mut cycle) = have_cycle(adj_node, rev_sort, states, adj_nodess) {
                    if cycle[0] != *cycle.last().unwrap() {
                        // cycle isn't complete yet
                        cycle.push(current);
                    }
                    return Some(cycle);
                }
            }
            states[current] = 2;
            rev_sort.push(current);
            None
        }
        let origins = if let Some(from) = from {
            from..(from + 1)
        } else {
            0..states.len()
        };
        for node in origins {
            if states[node] == 0 {
                let cycle = have_cycle(node, &mut rev_sort, &mut states, &self.adj_nodess);
                if let Some(cycle) = cycle {
                    return Err(cycle.into_iter().skip(1).rev().collect());
                }
            }
        }
        Ok(rev_sort.into_iter().rev().collect())
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
