#![allow(unused_imports, dead_code, clippy::needless_range_loop, unused_labels)]
use io::*;
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{stdin, stdout, BufReader},
    iter,
    mem::{self, swap},
    usize,
};

fn main() {
    let mut sc = Scanner::new(stdin());
    let mut pt = Printer::new(stdout());
    let test_case = sc.next::<usize>();
    'test: for _ in 0..test_case {
        let n = sc.next::<usize>();
        let a = sc.next::<usize>() - 1;
        let b = sc.next::<usize>() - 1;
        let mut adj_nodes = vec![Vec::<usize>::new(); n];
        for _ in 0..(n - 1) {
            let u = sc.next::<usize>() - 1;
            let v = sc.next::<usize>() - 1;
            adj_nodes[u].push(v);
            adj_nodes[v].push(u);
        }
        // find path between u and v
        let mut a_visited = vec![false; n];
        let mut a_to_visit = vec![(a, a)]; // (src, dst)
        let mut b_to_visit = vec![(b, b)];
        let mut b_moved = 0;
        let middle = 'middle: loop {
            let mut a_next_visit = Vec::new();
            for (src, dst) in a_to_visit {
                a_next_visit.extend(adj_nodes[dst].iter().filter_map(|&nd| {
                    if nd == src {
                        None
                    } else {
                        Some((dst, nd))
                    }
                }));
                a_visited[dst] = true;
            }
            a_to_visit = a_next_visit;

            let mut b_next_visit = Vec::new();
            for (src, dst) in b_to_visit {
                if a_visited[dst] {
                    break 'middle dst;
                }
                b_next_visit.extend(adj_nodes[dst].iter().filter_map(|&nd| {
                    if nd == src {
                        None
                    } else {
                        Some((dst, nd))
                    }
                }));
            }
            b_to_visit = b_next_visit;

            b_moved += 1;
        };
        let mut dist = vec![usize::MAX; n];
        let mut to_visit = vec![middle];
        let mut d = 0usize;
        while !to_visit.is_empty() {
            let mut next_visit = Vec::new();
            for v in to_visit {
                next_visit.extend(adj_nodes[v].iter().filter(|&&nd| dist[nd] == usize::MAX));
                dist[v] = d;
            }
            to_visit = next_visit;
            d += 1;
        }
        let &max_d = dist.iter().max().unwrap();
        pt.println(((n - 1) * 2) - max_d + b_moved);
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
