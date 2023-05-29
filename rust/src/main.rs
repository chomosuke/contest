#![allow(unused_imports, dead_code)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{stdin, BufReader},
};

fn get_bs(
    node: usize,
    parent: usize,
    adj: &[Vec<usize>],
    ranges: &[(usize, usize)],
) -> (usize, usize) {
    let mut bss = Vec::with_capacity(adj[node].len());
    let children = adj[node].iter().filter(|&&n| n != parent);
    for &nn in children.clone() {
        bss.push(get_bs(nn, node, adj, ranges));
    }
    let ars = [ranges[node].0, ranges[node].1];
    let mut bs = [0, 0];
    for (i, a) in ars.into_iter().enumerate() {
        for (j, &c) in children.clone().enumerate() {
            let range = ranges[c];
            let cbs = bss[j];
            bs[i] += max(range.0.abs_diff(a) + cbs.0, range.1.abs_diff(a) + cbs.1);
        }
    }
    (bs[0], bs[1])
}

fn main() {
    let mut sc = Scanner::new(stdin());
    let test_cases = sc.next::<u64>();
    for _ in 0..test_cases {
        let n = sc.next::<usize>();
        let mut ranges = Vec::with_capacity(n);
        for _ in 0..n {
            ranges.push((sc.next::<usize>(), sc.next::<usize>()));
        }
        let mut adj = vec![Vec::new(); n];
        for _ in 0..(n - 1) {
            let v = sc.next::<usize>() - 1;
            let u = sc.next::<usize>() - 1;
            adj[v].push(u);
            adj[u].push(v);
        }
        let bs = get_bs(0, 0, &adj, &ranges);
        println!("{}", max(bs.0, bs.1));
    }
}

mod scanner {
    use std::collections::{HashSet, VecDeque};
    use std::io::{BufReader, Lines, Read};
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
}
#[allow(unused_imports)]
use scanner::*;
