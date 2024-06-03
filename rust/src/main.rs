#![allow(unused_imports, dead_code, clippy::needless_range_loop, unused_labels)]
use io::*;
use std::{
    cmp::{max, min, Ordering},
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

fn main() {
    let mut sc = Scanner::new(stdin());
    let mut pt = Printer::new(stdout());
    let test_case = sc.next::<usize>();
    'test: for _ in 0..test_case {
        let n = sc.next::<u64>();
        let m = sc.next::<u64>();
        let k = sc.next::<usize>();
        let mut fountains = Vec::with_capacity(k);
        for _ in 0..k {
            fountains.push((sc.next::<u64>(), sc.next::<u64>()))
        }
        let fountains = fountains;
        let mut fs = fountains.clone();
        fs.sort_by_key(|&(r, c)| (c, u64::MAX - r));
        let mut edges = Vec::new();
        let mut row = 0;
        for (i, &(r, c)) in fs.iter().enumerate() {
            if r > row {
                row = r;
                edges.push((i, (r, c)));
            }
        }
        let mut area = 0;
        let mut last_row = 0;
        for &(_, (r, c)) in &edges {
            area += (r - last_row) * (c - 1);
            last_row = r;
        }
        area += (n - last_row) * m;
        let edges_m = edges
            .iter()
            .cloned()
            .enumerate()
            .map(|(e_i, (f_i, rc))| (rc, (f_i, e_i)))
            .collect::<BTreeMap<_, _>>();
        let edges = edges.into_iter().map(|(_i, rc)| rc).collect::<Vec<_>>();
        pt.println(area);
        for f in &fountains {
            if let Some(&(f_i, e_i)) = edges_m.get(f) {
                let last_r = if e_i > 0 { edges[e_i - 1].0 } else { 0 };
                let (nr, nc) = if e_i < edges.len() - 1 {
                    edges[e_i + 1]
                } else {
                    (n, m + 1)
                };
                let (r, c) = f;
                let prev_area = (r - last_r) * (c - 1) + (nr - r) * (nc - 1);

                let mut last_r = last_r;
                let mut new_area = 0;
                for &(r, c) in fs[f_i + 1..].iter() {
                    if r == nr && c == nc {
                        break;
                    }
                    if r > last_r {
                        new_area += (r - last_r) * (c - 1);
                        last_r = r;
                    }
                }
                new_area += (nr - last_r) * (nc - 1);
                pt.print(new_area - prev_area);
            } else {
                pt.print(0);
            }
            pt.print(' ');
        }
        pt.println("");
    }
}

mod io {
    use std::collections::{HashSet, VecDeque};
    use std::fmt::Display;
    use std::io::{BufReader, BufWriter, Lines, Read, Write};
    use std::marker::PhantomData;
    use std::{any::type_name, io::BufRead, str::FromStr};

    pub struct ScannerIter<'a, R: Read, T> {
        remaining: usize,
        sc: &'a mut Scanner<R>,
        item: PhantomData<T>,
    }

    impl<R: Read, T: FromStr> Iterator for ScannerIter<'_, R, T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.remaining == 0 {
                None
            } else {
                self.remaining -= 1;
                Some(self.sc.next::<T>())
            }
        }
    }

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

        pub fn next_n<T: FromStr>(&mut self, n: usize) -> ScannerIter<'_, R, T> {
            ScannerIter {
                remaining: n,
                sc: self,
                item: PhantomData,
            }
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
