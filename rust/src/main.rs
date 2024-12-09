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
    let map = INPUT.iter().map(|b| b - b'0');

    let mut spaces_start_len = BTreeMap::<usize, usize>::new();
    let mut files_start_len = Vec::<(usize, usize)>::new();

    let mut start = 0;

    let mut is_blank = false;
    for len in map {
        let len = len as usize;
        if is_blank {
            is_blank = false;
            spaces_start_len.insert(start, len);
        } else {
            is_blank = true;
            files_start_len.push((start, len));
        }
        start += len;
    }

    let mut compact_start_id_len = BTreeMap::<usize, (usize, usize)>::new();
    while let Some((start, len)) = files_start_len.pop() {
        let id = files_start_len.len();
        let mut large_space = start;
        for (&space_start, &space_len) in &spaces_start_len {
            if space_start >= large_space {
                break;
            }
            if space_len >= len {
                large_space = space_start;
                break;
            }
        }
        compact_start_id_len.insert(large_space, (id, len));
        if let Some(space_len) = spaces_start_len.remove(&large_space) {
            let new_space_len = space_len - len;
            if new_space_len > 0 {
                spaces_start_len.insert(large_space + len, new_space_len);
            }
        }
    }

    let mut sum = 0;
    for (start, (id, len)) in compact_start_id_len {
        for i in 0..len {
            sum += (start + i) * id;
        }
    }
    println!("{sum}");
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
