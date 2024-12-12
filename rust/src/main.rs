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
        Add,
        Bound::{Excluded, Included, Unbounded},
        Deref, Range, RangeBounds,
    },
};

mod input;
use input::*;

type I = i128;
type U = u128;

fn main() {
    let garden = INPUT
        .lines()
        .map(|s| s.as_bytes().to_owned())
        .collect::<Vec<_>>();
    let mut visited = vec![vec![false; garden[0].len()]; garden.len()];
    let mut price = 0;
    for i in 0..garden.len() {
        for j in 0..garden[i].len() {
            if visited[i][j] {
                continue;
            }
            let mut area = 0_u128;
            let mut fence_v = HashMap::<(I, I), Vec<I>>::new();
            let mut fence_h = HashMap::<(I, I), Vec<I>>::new();
            let color = garden[i][j];

            let mut to_visits = vec![(i, j)];
            visited[i][j] = true;
            while let Some((i, j)) = to_visits.pop() {
                area += 1;
                assert_eq!(color, garden[i][j]);

                let i = i as i128;
                let j = j as i128;
                for (ni, nj) in [(i, j + 1), (i + 1, j), (i, j - 1), (i - 1, j)] {
                    if ni >= 0 && nj >= 0 {
                        let i = ni as usize;
                        let j = nj as usize;
                        if i < garden.len() && j < garden[i].len() && garden[i][j] == color {
                            if !visited[i][j] {
                                visited[i][j] = true;
                                to_visits.push((i, j));
                            }
                            continue;
                        }
                    }
                    if ni != i {
                        fence_v.entry((i, ni)).or_default().push(j);
                    } else {
                        assert_ne!(nj, j);
                        fence_h.entry((j, nj)).or_default().push(i);
                    }
                }
            }

            let mut fence = 0;
            for mut vs in fence_v.into_values().chain(fence_h.into_values()) {
                vs.sort();
                fence += 1;
                for i in 1..vs.len() {
                    if vs[i] - vs[i - 1] != 1 {
                        fence += 1;
                    }
                }
            }

            price += fence * area;

            // println!("{} {fence}", color as char);
        }
    }
    println!("{price}");
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
            self.print_bytes(b"\n");
        }

        pub fn print_iter(&mut self, mut iter: impl Iterator<Item = impl Display>) {
            if let Some(e) = iter.next() {
                self.print(&e);
                for e in iter {
                    self.print_bytes(b" ");
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
