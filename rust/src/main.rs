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

type Int = i128;

fn get_gcd(mut a: Int, mut b: Int) -> Int {
    while b != 0 {
        let rem = a % b;
        a = b;
        b = rem;
    }
    a
}

fn main() {
    let map = INPUT
        .lines()
        .map(|l| l.as_bytes().to_owned())
        .collect::<Vec<_>>();
    let mut antennas = HashMap::<u8, Vec<(I, I)>>::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != b'.' {
                antennas
                    .entry(map[i][j])
                    .or_default()
                    .push((i as I, j as I));
            }
        }
    }
    let mut antinodes = HashSet::<(I, I)>::new();
    let n = map.len() as I;
    let m = map[0].len() as I;
    for (_, antennas) in antennas {
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let mut diff = (antennas[i].0 - antennas[j].0, antennas[i].1 - antennas[j].1);
                let g = get_gcd(diff.0, diff.1);
                diff.0 /= g;
                diff.1 /= g;
                let mut anti = antennas[i];
                while anti.0 >= 0 && anti.0 < n && anti.1 >= 0 && anti.1 < m {
                    antinodes.insert(anti);
                    anti.0 -= diff.0;
                    anti.1 -= diff.1;
                }
                let mut anti = antennas[i];
                while anti.0 >= 0 && anti.0 < n && anti.1 >= 0 && anti.1 < m {
                    antinodes.insert(anti);
                    anti.0 += diff.0;
                    anti.1 += diff.1;
                }
            }
        }
    }
    let mut count = 0;
    for (i, j) in antinodes {
        if i >= 0 && (i as usize) < map.len() && j >= 0 && (j as usize) < map[i as usize].len() {
            count += 1;
        }
    }
    println!("{count}");
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
