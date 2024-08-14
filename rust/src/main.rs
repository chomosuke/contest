#![allow(
    unused_imports,
    dead_code,
    clippy::needless_range_loop,
    unused_labels,
    clippy::ptr_arg
)]
use core::hash::Hash;
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

type I = i128;
type U = u128;

/// O(log(x))
pub fn get_gcd(mut x: I, mut y: I) -> I {
    while y != 0 {
        let ty = y;
        y = x.rem_euclid(y);
        x = ty;
    }
    x
}

fn main() {
    let mut sc = Scanner::new(stdin());
    let mut pt = Printer::new(stdout());
    let test_cases = sc.next::<usize>();
    'test: for _ in 0..test_cases {
        let n = sc.next::<usize>();
        let m = sc.next::<usize>();
        let k = sc.next::<usize>();
        let mut ass = Vec::with_capacity(n);
        for _ in 0..n {
            ass.push(sc.next_n::<I>(m));
        }
        let mut capss = Vec::with_capacity(n);
        for _ in 0..n {
            capss.push(
                sc.next_line()
                    .bytes()
                    .map(|b| if b == b'1' { 1 } else { -1 })
                    .collect::<Vec<_>>(),
            );
        }

        let mut cap_height_diff = 0;
        for i in 0..n {
            for j in 0..m {
                cap_height_diff += ass[i][j] * capss[i][j];
            }
        }

        let mut prefix_capss = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                prefix_capss[i + 1][j + 1] = prefix_capss[i][j + 1] + prefix_capss[i + 1][j]
                    - prefix_capss[i][j]
                    + capss[i][j];
            }
        }
        let mut diffs = Vec::new();
        for i in 0..(n - k + 1) {
            for j in 0..(m - k + 1) {
                let diff = prefix_capss[i + k][j + k] + prefix_capss[i][j]
                    - prefix_capss[i][j + k]
                    - prefix_capss[i + k][j];
                if diff != 0 {
                    diffs.push(diff);
                }
            }
        }
        if diffs.is_empty() {
            if cap_height_diff == 0 {
                pt.println("YES");
            } else {
                pt.println("NO");
            }
        } else {
            let mut gcd = diffs[0];
            for &diff in diffs.iter().skip(1) {
                gcd = get_gcd(gcd, diff);
            }
            if cap_height_diff % gcd == 0 {
                pt.println("YES");
            } else {
                pt.println("NO");
            }
        }
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
