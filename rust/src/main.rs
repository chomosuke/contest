#![allow(unused_imports, dead_code)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{stdin, BufReader},
};

fn main() {
    let mut sc = Scanner::new(stdin());
    let n = sc.next::<usize>();
    let mut adj_mat = Vec::with_capacity(n);
    for _ in 0..n {
        let adj_row = sc.next_n::<u64>(n).collect::<Vec<_>>();
        adj_mat.push(adj_row);
    }
    let delete = sc.next_n::<usize>(n).map(|v| v - 1).collect::<Vec<_>>();
    let mut sums = Vec::with_capacity(n);
    for (i, &ni) in delete.iter().rev().enumerate() {
        for n1 in 0..n {
            for n2 in 0..n {
                adj_mat[n1][n2] = min(adj_mat[n1][ni] + adj_mat[ni][n2], adj_mat[n1][n2]);
            }
        }
        let contained = &delete[(n - i - 1)..];
        let mut sum = 0;
        for &n1 in contained {
            for &n2 in contained {
                sum += adj_mat[n1][n2];
            }
        }
        sums.push(sum);
    }
    print!("{}", sums.pop().unwrap());
    for s in sums.into_iter().rev() {
        print!(" {s}");
    }
    println!();
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
